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

//! Generated file from `MonsterBornType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:MonsterBornType)
pub enum MonsterBornType {
    // @@protoc_insertion_point(enum_value:MonsterBornType.MONSTER_BORN_TYPE_NONE)
    MONSTER_BORN_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:MonsterBornType.MONSTER_BORN_TYPE_DEFAULT)
    MONSTER_BORN_TYPE_DEFAULT = 1,
    // @@protoc_insertion_point(enum_value:MonsterBornType.MONSTER_BORN_TYPE_RANDOM)
    MONSTER_BORN_TYPE_RANDOM = 2,
}

impl ::protobuf::Enum for MonsterBornType {
    const NAME: &'static str = "MonsterBornType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MonsterBornType> {
        match value {
            0 => ::std::option::Option::Some(MonsterBornType::MONSTER_BORN_TYPE_NONE),
            1 => ::std::option::Option::Some(MonsterBornType::MONSTER_BORN_TYPE_DEFAULT),
            2 => ::std::option::Option::Some(MonsterBornType::MONSTER_BORN_TYPE_RANDOM),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<MonsterBornType> {
        match str {
            "MONSTER_BORN_TYPE_NONE" => ::std::option::Option::Some(MonsterBornType::MONSTER_BORN_TYPE_NONE),
            "MONSTER_BORN_TYPE_DEFAULT" => ::std::option::Option::Some(MonsterBornType::MONSTER_BORN_TYPE_DEFAULT),
            "MONSTER_BORN_TYPE_RANDOM" => ::std::option::Option::Some(MonsterBornType::MONSTER_BORN_TYPE_RANDOM),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [MonsterBornType] = &[
        MonsterBornType::MONSTER_BORN_TYPE_NONE,
        MonsterBornType::MONSTER_BORN_TYPE_DEFAULT,
        MonsterBornType::MONSTER_BORN_TYPE_RANDOM,
    ];
}

impl ::protobuf::EnumFull for MonsterBornType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("MonsterBornType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for MonsterBornType {
    fn default() -> Self {
        MonsterBornType::MONSTER_BORN_TYPE_NONE
    }
}

impl MonsterBornType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MonsterBornType>("MonsterBornType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15MonsterBornType.proto*j\n\x0fMonsterBornType\x12\x1a\n\x16MONSTER_\
    BORN_TYPE_NONE\x10\0\x12\x1d\n\x19MONSTER_BORN_TYPE_DEFAULT\x10\x01\x12\
    \x1c\n\x18MONSTER_BORN_TYPE_RANDOM\x10\x02B\x1b\n\x19emu.grasscutter.net\
    .protob\x06proto3\
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
            enums.push(MonsterBornType::generated_enum_descriptor_data());
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
