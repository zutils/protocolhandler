//! buildfunctions provides functions to be used in plugins' build.rs file.
use protoc_rust_grpc as prg;
use failure::{Error, format_err};
use std::fs::File;	
use std::path::PathBuf;

/// Call protoc on protobuffer and create non-rpc code
pub fn build_rust_code_from_protobuffer(proto_filename: &PathBuf) -> Result<(), Error> {
	use std::path::{Path, PathBuf};
	use pb_rs::types::{Config, FileDescriptor};
	use std::env;

	println!("Building protobuf for {:?}", &proto_filename);

	let path_str = proto_filename.to_str().ok_or(format_err!("Cannot create str from PathBuf!"))?;

	let base_name = base_name(proto_filename);
	let out_dir = out_dir(&proto_filename);
	make_sure_path_exists(&out_dir)?;

    let config = Config {
        in_file: proto_filename,
        out_file: PathBuf::from(out_dir + "/" + basename + ".proto"),
        single_module: false,
        import_search_path: vec![PathBuf::from("./schema")],
        no_output: false,
        error_cycle: false,
        headers: false, // What does this do? hmm...
    };

    FileDescriptor::write_proto(&config).unwrap();

	println!("Protobuf to rust creation successful. {}", path_str);
	Ok(())
}

/// Call protoc on protobuffer and create only the rpc code
pub fn build_rust_rpc_code_from_protobuffer(proto_filename: &PathBuf) -> Result<(), Error> {
	println!("Building protobuf rpc for {:?}", &proto_filename);
	let path_str = proto_filename.to_str().ok_or(format_err!("Cannot create str from PathBuf!"))?;

	let args = prg::Args {
			out_dir: &out_dir(&proto_filename),
			input: &[path_str],
			includes: &["./schema"],
			rust_protobuf: false,
			..Default::default()
	};

	prg::run(args).expect("protoc-rust-grpc");

	println!("Protoc-rust-grpc ran on {}", path_str);

	Ok(())
}

/// Adds the file to IPFS so that 1) we can get it's hash and 2) So that we can generate a schema url from that hash
/// In parent program, lib.rs loads in the schema_link at compile time so that the library can use it.
pub fn add_file_and_write_ipfs_hash(path: &PathBuf) -> Result<(), Error> {
	use hyper::rt::Future;
	use std::sync::{Arc, Mutex};
	let client = ipfs_api::IpfsClient::default();
	
	println!("Adding {:?} to ipfs...", path);
	let should_panic = Arc::new(Mutex::new(false));
	let should_panic_clone = should_panic.clone();
	let file = File::open(path)?;
	let base_name = base_name(path);
	let req = client.add(file)
					.map(move |result| { 
						let schema_link = result.hash;
						let schema_link_file_location = format!("./schema_links/{}.txt", base_name);
                        write_to_file(&PathBuf::from(schema_link_file_location), &schema_link).unwrap();
                    })
					.map_err(move |_e| {
						let mut data = should_panic_clone.lock().unwrap();
						*data = true; 
					});

	hyper::rt::run(req);

	// We have to panic in the main thread.
	if *should_panic.lock().unwrap() == true {
		panic!(r#"Unable to retrieve schema URL from ipfs. Make sure that IPFS daemon is running! You can get IPFS from ipfs.io\nIf you REALLY don't want to use ipfs, and care to handle the schema_link manually, modify your build.rs file."#);
	}

    Ok(())
}

pub fn for_all_in_dir(path_str: &str, func: fn(&PathBuf) -> Result<(),Error>) {
	use std::fs;
    let paths = fs::read_dir(path_str).unwrap();

    for path in paths {
		let path = path.unwrap().path();
		if let Err(e) = func(&path) {
			println!("{:?}", e);
		}
    }
}

pub fn write_to_file(new_file: &PathBuf, contents: &str) -> Result<(), Error> {
	use std::io::Write;

	println!("Writing file: {:?}", new_file);
	let mut file = File::create(new_file)?;
	file.write_all(contents.as_bytes())?;
	Ok(())
}

fn base_name(protobuf_path: &PathBuf) -> String {
	let base_name: String = protobuf_path.file_stem().unwrap().to_str().unwrap().to_string();
	base_name
}

fn out_dir(protobuf_path: &PathBuf) -> String {
	"src/".to_owned() + &base_name(protobuf_path) + "_interface"
}

fn make_sure_path_exists(path: &str) -> Result<(), Error> {
	use std::fs;
	fs::create_dir_all(path)?;
    Ok(())
}