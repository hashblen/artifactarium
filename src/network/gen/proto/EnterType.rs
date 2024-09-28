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

//! Generated file from `EnterType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EnterType)
pub enum EnterType {
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_NONE)
    ENTER_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_SELF)
    ENTER_TYPE_SELF = 1,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_GOTO)
    ENTER_TYPE_GOTO = 2,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_JUMP)
    ENTER_TYPE_JUMP = 3,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_OTHER)
    ENTER_TYPE_OTHER = 4,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_BACK)
    ENTER_TYPE_BACK = 5,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_DUNGEON)
    ENTER_TYPE_DUNGEON = 6,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_DUNGEON_REPLAY)
    ENTER_TYPE_DUNGEON_REPLAY = 7,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_GOTO_BY_PORTAL)
    ENTER_TYPE_GOTO_BY_PORTAL = 8,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_SELF_HOME)
    ENTER_TYPE_SELF_HOME = 9,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_OTHER_HOME)
    ENTER_TYPE_OTHER_HOME = 10,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_GOTO_RECREATE)
    ENTER_TYPE_GOTO_RECREATE = 11,
    // @@protoc_insertion_point(enum_value:EnterType.ENTER_TYPE_GOTO_BY_TPL)
    ENTER_TYPE_GOTO_BY_TPL = 12,
}

impl ::protobuf::Enum for EnterType {
    const NAME: &'static str = "EnterType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnterType> {
        match value {
            0 => ::std::option::Option::Some(EnterType::ENTER_TYPE_NONE),
            1 => ::std::option::Option::Some(EnterType::ENTER_TYPE_SELF),
            2 => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO),
            3 => ::std::option::Option::Some(EnterType::ENTER_TYPE_JUMP),
            4 => ::std::option::Option::Some(EnterType::ENTER_TYPE_OTHER),
            5 => ::std::option::Option::Some(EnterType::ENTER_TYPE_BACK),
            6 => ::std::option::Option::Some(EnterType::ENTER_TYPE_DUNGEON),
            7 => ::std::option::Option::Some(EnterType::ENTER_TYPE_DUNGEON_REPLAY),
            8 => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO_BY_PORTAL),
            9 => ::std::option::Option::Some(EnterType::ENTER_TYPE_SELF_HOME),
            10 => ::std::option::Option::Some(EnterType::ENTER_TYPE_OTHER_HOME),
            11 => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO_RECREATE),
            12 => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO_BY_TPL),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EnterType> {
        match str {
            "ENTER_TYPE_NONE" => ::std::option::Option::Some(EnterType::ENTER_TYPE_NONE),
            "ENTER_TYPE_SELF" => ::std::option::Option::Some(EnterType::ENTER_TYPE_SELF),
            "ENTER_TYPE_GOTO" => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO),
            "ENTER_TYPE_JUMP" => ::std::option::Option::Some(EnterType::ENTER_TYPE_JUMP),
            "ENTER_TYPE_OTHER" => ::std::option::Option::Some(EnterType::ENTER_TYPE_OTHER),
            "ENTER_TYPE_BACK" => ::std::option::Option::Some(EnterType::ENTER_TYPE_BACK),
            "ENTER_TYPE_DUNGEON" => ::std::option::Option::Some(EnterType::ENTER_TYPE_DUNGEON),
            "ENTER_TYPE_DUNGEON_REPLAY" => ::std::option::Option::Some(EnterType::ENTER_TYPE_DUNGEON_REPLAY),
            "ENTER_TYPE_GOTO_BY_PORTAL" => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO_BY_PORTAL),
            "ENTER_TYPE_SELF_HOME" => ::std::option::Option::Some(EnterType::ENTER_TYPE_SELF_HOME),
            "ENTER_TYPE_OTHER_HOME" => ::std::option::Option::Some(EnterType::ENTER_TYPE_OTHER_HOME),
            "ENTER_TYPE_GOTO_RECREATE" => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO_RECREATE),
            "ENTER_TYPE_GOTO_BY_TPL" => ::std::option::Option::Some(EnterType::ENTER_TYPE_GOTO_BY_TPL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EnterType] = &[
        EnterType::ENTER_TYPE_NONE,
        EnterType::ENTER_TYPE_SELF,
        EnterType::ENTER_TYPE_GOTO,
        EnterType::ENTER_TYPE_JUMP,
        EnterType::ENTER_TYPE_OTHER,
        EnterType::ENTER_TYPE_BACK,
        EnterType::ENTER_TYPE_DUNGEON,
        EnterType::ENTER_TYPE_DUNGEON_REPLAY,
        EnterType::ENTER_TYPE_GOTO_BY_PORTAL,
        EnterType::ENTER_TYPE_SELF_HOME,
        EnterType::ENTER_TYPE_OTHER_HOME,
        EnterType::ENTER_TYPE_GOTO_RECREATE,
        EnterType::ENTER_TYPE_GOTO_BY_TPL,
    ];
}

impl ::protobuf::EnumFull for EnterType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("EnterType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for EnterType {
    fn default() -> Self {
        EnterType::ENTER_TYPE_NONE
    }
}

impl EnterType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<EnterType>("EnterType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fEnterType.proto*\xcf\x02\n\tEnterType\x12\x13\n\x0fENTER_TYPE_NONE\
    \x10\0\x12\x13\n\x0fENTER_TYPE_SELF\x10\x01\x12\x13\n\x0fENTER_TYPE_GOTO\
    \x10\x02\x12\x13\n\x0fENTER_TYPE_JUMP\x10\x03\x12\x14\n\x10ENTER_TYPE_OT\
    HER\x10\x04\x12\x13\n\x0fENTER_TYPE_BACK\x10\x05\x12\x16\n\x12ENTER_TYPE\
    _DUNGEON\x10\x06\x12\x1d\n\x19ENTER_TYPE_DUNGEON_REPLAY\x10\x07\x12\x1d\
    \n\x19ENTER_TYPE_GOTO_BY_PORTAL\x10\x08\x12\x18\n\x14ENTER_TYPE_SELF_HOM\
    E\x10\t\x12\x19\n\x15ENTER_TYPE_OTHER_HOME\x10\n\x12\x1c\n\x18ENTER_TYPE\
    _GOTO_RECREATE\x10\x0b\x12\x1a\n\x16ENTER_TYPE_GOTO_BY_TPL\x10\x0cB\x1b\
    \n\x19emu.grasscutter.net.protob\x06proto3\
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
            enums.push(EnterType::generated_enum_descriptor_data());
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
