//! The pluginhandler handles loading the correct plugins and routing calls between them.

use std::path::PathBuf;

use failure::Error;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use signals::{Signal, Emitter, Am};

use crate::{Transport, VecTransport, ModuleToTransportGlue};
use crate::propagator::{Propagator, TransportNode};
use crate::transportresponse::TransportResponse;

//#[derive(Default)]
pub struct PluginHandler {
    libraries: Am<Vec<libloading::Library>>,
}

impl PluginHandler {
    pub fn new() -> Self {
        PluginHandler{ libraries: Am::new(Vec::new()), }
    }
}

impl DynamicLibraryLoader for PluginHandler {
    fn get_library_list(&self) -> Am<Vec<libloading::Library>> {
        self.libraries.clone()
    }
}

trait CommonFFI {
    fn call_ffi_propagate(&self, transport: &Transport) -> Result<Vec<Transport>, Error>;
    fn call_ffi_init(&self) -> Result<(), Error>;
}

impl CommonFFI for libloading::Library {
    // TODO: Handle c-style ffi
    fn call_ffi_propagate(&self, transport: &Transport) -> Result<Vec<Transport>, Error> {
        log::trace!("Calling FFI function 'propagate_ffi(...)'...");

        let bytes = quick_protobuf::serialize_into_vec(transport)?;

        let from_ffi = unsafe {
            let propagate: libloading::Symbol<unsafe extern fn(&[u8]) -> Vec<u8>> = self.get(b"propagate_ffi")?;
            propagate(&bytes)
        };

        let ret: VecTransport = quick_protobuf::deserialize_from_slice(&from_ffi)?;
        log::trace!("...Received from FFI: {:?}", ret);
        Ok(ret.vec)
    }

    fn call_ffi_init(&self) -> Result<(), Error> {
        log::debug!("Calling FFI function 'init()'...");
        unsafe {
            let init: libloading::Symbol<unsafe extern fn()> = self.get(b"init")?;
            init();
        }
        log::debug!("...init() successful!");
        Ok(())
    }
}

pub fn ffi_handle_received_bytes(node: &TransportNode, bytes: &[u8]) -> Vec<u8> {
    let mut ret = VecTransport::default();

    match quick_protobuf::deserialize_from_slice::<Transport>(bytes) {
        Err(e) => {
            let transport = TransportResponse::create_Error(&format!("Cannot parse data! Possibly incorrect version. {:?}", e));
            ret.vec.push(transport);
        },
        Ok(transport) => {
            let mut vectransport_data: Vec<Transport> = node.propagate_transport(&transport);
            ret.vec.append(&mut vectransport_data);
        },
    } 

    // serialize_into_vec returns a result - one that we cannot pass back. Fail as gracefully as we can :(
    match quick_protobuf::serialize_into_vec(&ret) {
        Ok(bytes) => bytes.to_vec(),
        Err(e) => {
            log::error!("Cannot write VecTransport to bytes! {:?}", e);
            Vec::new() // Return NOTHING :( TODO: Write test case for this.
        }
    }
}

/// Allow us to use CommonModule functions on the PluginHandler
impl ModuleToTransportGlue for PluginHandler {}

/// We want to propagate over any dynamic library
impl Propagator for PluginHandler {
    fn propagate_transport(&self, transport: &Transport) -> Vec<Transport>  {
        let mut ret = Vec::new();
        let libraries = self.libraries.lock();
        for lib in libraries.iter() {
            match lib.call_ffi_propagate(transport) {
                Ok(mut owned_transport) => ret.append(&mut owned_transport),
                Err(e) => log::error!("Error when propagating over dynamic library! {:?}", e),
            }
        }
        ret
    }
}

pub trait DynamicLibraryLoader {
    fn get_library_list(&self) -> Am<Vec<libloading::Library>>;

    /// Take a path glob and load in all plugins in that glob
    fn load_all_plugins(&self, path_glob: &str) -> Result<(), Error> {
        let library = self.get_library_list();
        let emitter = Signal::new_arc_mutex(move |path: PathBuf| {
            let new_plugin = load_plugin(&path)?;
            library.lock().push(new_plugin);
            Ok(())
        });

        glob::glob(path_glob)?.filter_map(Result::ok).for_each(|path: PathBuf| {
            emitter.lock().emit(path);
        });

        Ok(())
    }

    /// Continuously load from plugin directories
    fn continuously_watch_for_new_plugins(&self, watch_path: PathBuf) {
        let library = self.get_library_list();
        let emitter = Signal::new_arc_mutex(move |path: PathBuf| {
            let new_plugin = load_plugin(&path)?;
            library.lock().push(new_plugin);
            Ok(())
        });

        ::std::thread::spawn(move || {           
            if let Err(e) = blocking_watch_directory(watch_path, emitter ){
                log::error!("{:?}", e);
            }
        });
    }

    fn load_plugin(&self, path: &PathBuf) -> Result<(), Error> {
        let new_plugin = load_plugin(&path)?;
        self.get_library_list().lock().push(new_plugin);
        Ok(())
    }
}

/// So that you can load different plugins while the application is running.
fn load_plugin(path: &PathBuf) -> Result<libloading::Library, Error> {
    if !path.exists() {
        return Err(failure::format_err!("Failed to load library. {:?} does not exist!", path));
    }

    log::debug!("Loading library {:?}...", path);
    let library = libloading::Library::new(path)?;
    library.call_ffi_init()?;
    log::debug!("...{:?} loaded successfully.", path);
    Ok(library)
}

// Start file watcher on watch_path. Emit on_path_changed if a file changes.
fn blocking_watch_directory<E>(watch_path: PathBuf, on_path_changed: Am<E>) -> Result<(), Error> 
    where E: Emitter<input=PathBuf>
{
    use std::sync::mpsc::channel;

    let (transmit, receive) = channel();
    let mut watcher = raw_watcher(transmit).unwrap();
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    // Continuously loop and receive events
    loop {
        match receive.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                log::debug!("Raw Event: {:?} {:?} {:?}", op, path, cookie);
                on_path_changed.lock().emit(path);
            },
            Ok(event) => log::error!("Broken Directory Watcher Event: {:?}", event),
            Err(e) => log::error!("Directory Watcher Error: {:?}", e),
        }
    }
}
