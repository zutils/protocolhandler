// This file is generated by rust-protobuf 2.1.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct RootMessage {
    // message fields
    pub message_type_hash: ::std::string::String,
    pub unencrypted_message: ::std::vec::Vec<u8>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    cached_size: ::protobuf::CachedSize,
}

impl RootMessage {
    pub fn new() -> RootMessage {
        ::std::default::Default::default()
    }

    // string message_type_hash = 1;

    pub fn clear_message_type_hash(&mut self) {
        self.message_type_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_message_type_hash(&mut self, v: ::std::string::String) {
        self.message_type_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_type_hash(&mut self) -> &mut ::std::string::String {
        &mut self.message_type_hash
    }

    // Take field
    pub fn take_message_type_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message_type_hash, ::std::string::String::new())
    }

    pub fn get_message_type_hash(&self) -> &str {
        &self.message_type_hash
    }

    // bytes unencrypted_message = 2;

    pub fn clear_unencrypted_message(&mut self) {
        self.unencrypted_message.clear();
    }

    // Param is passed by value, moved
    pub fn set_unencrypted_message(&mut self, v: ::std::vec::Vec<u8>) {
        self.unencrypted_message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unencrypted_message(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.unencrypted_message
    }

    // Take field
    pub fn take_unencrypted_message(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.unencrypted_message, ::std::vec::Vec::new())
    }

    pub fn get_unencrypted_message(&self) -> &[u8] {
        &self.unencrypted_message
    }
}

impl ::protobuf::Message for RootMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message_type_hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.unencrypted_message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.message_type_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message_type_hash);
        }
        if !self.unencrypted_message.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.unencrypted_message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.message_type_hash.is_empty() {
            os.write_string(1, &self.message_type_hash)?;
        }
        if !self.unencrypted_message.is_empty() {
            os.write_bytes(2, &self.unencrypted_message)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RootMessage {
        RootMessage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message_type_hash",
                    |m: &RootMessage| { &m.message_type_hash },
                    |m: &mut RootMessage| { &mut m.message_type_hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "unencrypted_message",
                    |m: &RootMessage| { &m.unencrypted_message },
                    |m: &mut RootMessage| { &mut m.unencrypted_message },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RootMessage>(
                    "RootMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static RootMessage {
        static mut instance: ::protobuf::lazy::Lazy<RootMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RootMessage,
        };
        unsafe {
            instance.get(RootMessage::new)
        }
    }
}

impl ::protobuf::Clear for RootMessage {
    fn clear(&mut self) {
        self.clear_message_type_hash();
        self.clear_unencrypted_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RootMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RootMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18schema/rootmessage.proto\x12\0\"M\n\x0bRootMessage\x12\x1d\n\x11me\
    ssage_type_hash\x18\x01\x20\x01(\tB\x02\x18\0\x12\x1f\n\x13unencrypted_m\
    essage\x18\x02\x20\x01(\x0cB\x02\x18\0B\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
