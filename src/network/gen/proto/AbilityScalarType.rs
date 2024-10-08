// This file is generated by rust-protobuf 3.5.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `AbilityScalarType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:AbilityScalarType)
pub enum AbilityScalarType {
    // @@protoc_insertion_point(enum_value:AbilityScalarType.ABILITY_SCALAR_TYPE_UNKNOW)
    ABILITY_SCALAR_TYPE_UNKNOW = 0,
    // @@protoc_insertion_point(enum_value:AbilityScalarType.ABILITY_SCALAR_TYPE_FLOAT)
    ABILITY_SCALAR_TYPE_FLOAT = 1,
    // @@protoc_insertion_point(enum_value:AbilityScalarType.ABILITY_SCALAR_TYPE_INT)
    ABILITY_SCALAR_TYPE_INT = 2,
    // @@protoc_insertion_point(enum_value:AbilityScalarType.ABILITY_SCALAR_TYPE_BOOL)
    ABILITY_SCALAR_TYPE_BOOL = 3,
    // @@protoc_insertion_point(enum_value:AbilityScalarType.ABILITY_SCALAR_TYPE_TRIGGER)
    ABILITY_SCALAR_TYPE_TRIGGER = 4,
    // @@protoc_insertion_point(enum_value:AbilityScalarType.ABILITY_SCALAR_TYPE_STRING)
    ABILITY_SCALAR_TYPE_STRING = 5,
    // @@protoc_insertion_point(enum_value:AbilityScalarType.ABILITY_SCALAR_TYPE_UINT)
    ABILITY_SCALAR_TYPE_UINT = 6,
}

impl ::protobuf::Enum for AbilityScalarType {
    const NAME: &'static str = "AbilityScalarType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AbilityScalarType> {
        match value {
            0 => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_UNKNOW),
            1 => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_FLOAT),
            2 => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_INT),
            3 => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_BOOL),
            4 => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_TRIGGER),
            5 => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_STRING),
            6 => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_UINT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<AbilityScalarType> {
        match str {
            "ABILITY_SCALAR_TYPE_UNKNOW" => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_UNKNOW),
            "ABILITY_SCALAR_TYPE_FLOAT" => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_FLOAT),
            "ABILITY_SCALAR_TYPE_INT" => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_INT),
            "ABILITY_SCALAR_TYPE_BOOL" => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_BOOL),
            "ABILITY_SCALAR_TYPE_TRIGGER" => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_TRIGGER),
            "ABILITY_SCALAR_TYPE_STRING" => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_STRING),
            "ABILITY_SCALAR_TYPE_UINT" => ::std::option::Option::Some(AbilityScalarType::ABILITY_SCALAR_TYPE_UINT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [AbilityScalarType] = &[
        AbilityScalarType::ABILITY_SCALAR_TYPE_UNKNOW,
        AbilityScalarType::ABILITY_SCALAR_TYPE_FLOAT,
        AbilityScalarType::ABILITY_SCALAR_TYPE_INT,
        AbilityScalarType::ABILITY_SCALAR_TYPE_BOOL,
        AbilityScalarType::ABILITY_SCALAR_TYPE_TRIGGER,
        AbilityScalarType::ABILITY_SCALAR_TYPE_STRING,
        AbilityScalarType::ABILITY_SCALAR_TYPE_UINT,
    ];
}

impl ::protobuf::EnumFull for AbilityScalarType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("AbilityScalarType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for AbilityScalarType {
    fn default() -> Self {
        AbilityScalarType::ABILITY_SCALAR_TYPE_UNKNOW
    }
}

impl AbilityScalarType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<AbilityScalarType>("AbilityScalarType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17AbilityScalarType.proto*\xec\x01\n\x11AbilityScalarType\x12\x1e\n\
    \x1aABILITY_SCALAR_TYPE_UNKNOW\x10\0\x12\x1d\n\x19ABILITY_SCALAR_TYPE_FL\
    OAT\x10\x01\x12\x1b\n\x17ABILITY_SCALAR_TYPE_INT\x10\x02\x12\x1c\n\x18AB\
    ILITY_SCALAR_TYPE_BOOL\x10\x03\x12\x1f\n\x1bABILITY_SCALAR_TYPE_TRIGGER\
    \x10\x04\x12\x1e\n\x1aABILITY_SCALAR_TYPE_STRING\x10\x05\x12\x1c\n\x18AB\
    ILITY_SCALAR_TYPE_UINT\x10\x06B\x1b\n\x19emu.grasscutter.net.protob\x06p\
    roto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(AbilityScalarType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
