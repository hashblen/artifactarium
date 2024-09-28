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

//! Generated file from `VisionType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:VisionType)
pub enum VisionType {
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_NONE)
    VISION_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_MEET)
    VISION_TYPE_MEET = 1,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_REBORN)
    VISION_TYPE_REBORN = 2,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_REPLACE)
    VISION_TYPE_REPLACE = 3,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_WAYPOINT_REBORN)
    VISION_TYPE_WAYPOINT_REBORN = 4,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_MISS)
    VISION_TYPE_MISS = 5,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_DIE)
    VISION_TYPE_DIE = 6,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_GATHER_ESCAPE)
    VISION_TYPE_GATHER_ESCAPE = 7,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_REFRESH)
    VISION_TYPE_REFRESH = 8,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_TRANSPORT)
    VISION_TYPE_TRANSPORT = 9,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_REPLACE_DIE)
    VISION_TYPE_REPLACE_DIE = 10,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_REPLACE_NO_NOTIFY)
    VISION_TYPE_REPLACE_NO_NOTIFY = 11,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_BORN)
    VISION_TYPE_BORN = 12,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_PICKUP)
    VISION_TYPE_PICKUP = 13,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_REMOVE)
    VISION_TYPE_REMOVE = 14,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_CHANGE_COSTUME)
    VISION_TYPE_CHANGE_COSTUME = 15,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_FISH_REFRESH)
    VISION_TYPE_FISH_REFRESH = 16,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_FISH_BIG_SHOCK)
    VISION_TYPE_FISH_BIG_SHOCK = 17,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_FISH_QTE_SUCC)
    VISION_TYPE_FISH_QTE_SUCC = 18,
    // @@protoc_insertion_point(enum_value:VisionType.VISION_TYPE_CAPTURE_DISAPPEAR)
    VISION_TYPE_CAPTURE_DISAPPEAR = 19,
}

impl ::protobuf::Enum for VisionType {
    const NAME: &'static str = "VisionType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<VisionType> {
        match value {
            0 => ::std::option::Option::Some(VisionType::VISION_TYPE_NONE),
            1 => ::std::option::Option::Some(VisionType::VISION_TYPE_MEET),
            2 => ::std::option::Option::Some(VisionType::VISION_TYPE_REBORN),
            3 => ::std::option::Option::Some(VisionType::VISION_TYPE_REPLACE),
            4 => ::std::option::Option::Some(VisionType::VISION_TYPE_WAYPOINT_REBORN),
            5 => ::std::option::Option::Some(VisionType::VISION_TYPE_MISS),
            6 => ::std::option::Option::Some(VisionType::VISION_TYPE_DIE),
            7 => ::std::option::Option::Some(VisionType::VISION_TYPE_GATHER_ESCAPE),
            8 => ::std::option::Option::Some(VisionType::VISION_TYPE_REFRESH),
            9 => ::std::option::Option::Some(VisionType::VISION_TYPE_TRANSPORT),
            10 => ::std::option::Option::Some(VisionType::VISION_TYPE_REPLACE_DIE),
            11 => ::std::option::Option::Some(VisionType::VISION_TYPE_REPLACE_NO_NOTIFY),
            12 => ::std::option::Option::Some(VisionType::VISION_TYPE_BORN),
            13 => ::std::option::Option::Some(VisionType::VISION_TYPE_PICKUP),
            14 => ::std::option::Option::Some(VisionType::VISION_TYPE_REMOVE),
            15 => ::std::option::Option::Some(VisionType::VISION_TYPE_CHANGE_COSTUME),
            16 => ::std::option::Option::Some(VisionType::VISION_TYPE_FISH_REFRESH),
            17 => ::std::option::Option::Some(VisionType::VISION_TYPE_FISH_BIG_SHOCK),
            18 => ::std::option::Option::Some(VisionType::VISION_TYPE_FISH_QTE_SUCC),
            19 => ::std::option::Option::Some(VisionType::VISION_TYPE_CAPTURE_DISAPPEAR),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<VisionType> {
        match str {
            "VISION_TYPE_NONE" => ::std::option::Option::Some(VisionType::VISION_TYPE_NONE),
            "VISION_TYPE_MEET" => ::std::option::Option::Some(VisionType::VISION_TYPE_MEET),
            "VISION_TYPE_REBORN" => ::std::option::Option::Some(VisionType::VISION_TYPE_REBORN),
            "VISION_TYPE_REPLACE" => ::std::option::Option::Some(VisionType::VISION_TYPE_REPLACE),
            "VISION_TYPE_WAYPOINT_REBORN" => ::std::option::Option::Some(VisionType::VISION_TYPE_WAYPOINT_REBORN),
            "VISION_TYPE_MISS" => ::std::option::Option::Some(VisionType::VISION_TYPE_MISS),
            "VISION_TYPE_DIE" => ::std::option::Option::Some(VisionType::VISION_TYPE_DIE),
            "VISION_TYPE_GATHER_ESCAPE" => ::std::option::Option::Some(VisionType::VISION_TYPE_GATHER_ESCAPE),
            "VISION_TYPE_REFRESH" => ::std::option::Option::Some(VisionType::VISION_TYPE_REFRESH),
            "VISION_TYPE_TRANSPORT" => ::std::option::Option::Some(VisionType::VISION_TYPE_TRANSPORT),
            "VISION_TYPE_REPLACE_DIE" => ::std::option::Option::Some(VisionType::VISION_TYPE_REPLACE_DIE),
            "VISION_TYPE_REPLACE_NO_NOTIFY" => ::std::option::Option::Some(VisionType::VISION_TYPE_REPLACE_NO_NOTIFY),
            "VISION_TYPE_BORN" => ::std::option::Option::Some(VisionType::VISION_TYPE_BORN),
            "VISION_TYPE_PICKUP" => ::std::option::Option::Some(VisionType::VISION_TYPE_PICKUP),
            "VISION_TYPE_REMOVE" => ::std::option::Option::Some(VisionType::VISION_TYPE_REMOVE),
            "VISION_TYPE_CHANGE_COSTUME" => ::std::option::Option::Some(VisionType::VISION_TYPE_CHANGE_COSTUME),
            "VISION_TYPE_FISH_REFRESH" => ::std::option::Option::Some(VisionType::VISION_TYPE_FISH_REFRESH),
            "VISION_TYPE_FISH_BIG_SHOCK" => ::std::option::Option::Some(VisionType::VISION_TYPE_FISH_BIG_SHOCK),
            "VISION_TYPE_FISH_QTE_SUCC" => ::std::option::Option::Some(VisionType::VISION_TYPE_FISH_QTE_SUCC),
            "VISION_TYPE_CAPTURE_DISAPPEAR" => ::std::option::Option::Some(VisionType::VISION_TYPE_CAPTURE_DISAPPEAR),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [VisionType] = &[
        VisionType::VISION_TYPE_NONE,
        VisionType::VISION_TYPE_MEET,
        VisionType::VISION_TYPE_REBORN,
        VisionType::VISION_TYPE_REPLACE,
        VisionType::VISION_TYPE_WAYPOINT_REBORN,
        VisionType::VISION_TYPE_MISS,
        VisionType::VISION_TYPE_DIE,
        VisionType::VISION_TYPE_GATHER_ESCAPE,
        VisionType::VISION_TYPE_REFRESH,
        VisionType::VISION_TYPE_TRANSPORT,
        VisionType::VISION_TYPE_REPLACE_DIE,
        VisionType::VISION_TYPE_REPLACE_NO_NOTIFY,
        VisionType::VISION_TYPE_BORN,
        VisionType::VISION_TYPE_PICKUP,
        VisionType::VISION_TYPE_REMOVE,
        VisionType::VISION_TYPE_CHANGE_COSTUME,
        VisionType::VISION_TYPE_FISH_REFRESH,
        VisionType::VISION_TYPE_FISH_BIG_SHOCK,
        VisionType::VISION_TYPE_FISH_QTE_SUCC,
        VisionType::VISION_TYPE_CAPTURE_DISAPPEAR,
    ];
}

impl ::protobuf::EnumFull for VisionType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("VisionType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for VisionType {
    fn default() -> Self {
        VisionType::VISION_TYPE_NONE
    }
}

impl VisionType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<VisionType>("VisionType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10VisionType.proto*\xae\x04\n\nVisionType\x12\x14\n\x10VISION_TYPE_N\
    ONE\x10\0\x12\x14\n\x10VISION_TYPE_MEET\x10\x01\x12\x16\n\x12VISION_TYPE\
    _REBORN\x10\x02\x12\x17\n\x13VISION_TYPE_REPLACE\x10\x03\x12\x1f\n\x1bVI\
    SION_TYPE_WAYPOINT_REBORN\x10\x04\x12\x14\n\x10VISION_TYPE_MISS\x10\x05\
    \x12\x13\n\x0fVISION_TYPE_DIE\x10\x06\x12\x1d\n\x19VISION_TYPE_GATHER_ES\
    CAPE\x10\x07\x12\x17\n\x13VISION_TYPE_REFRESH\x10\x08\x12\x19\n\x15VISIO\
    N_TYPE_TRANSPORT\x10\t\x12\x1b\n\x17VISION_TYPE_REPLACE_DIE\x10\n\x12!\n\
    \x1dVISION_TYPE_REPLACE_NO_NOTIFY\x10\x0b\x12\x14\n\x10VISION_TYPE_BORN\
    \x10\x0c\x12\x16\n\x12VISION_TYPE_PICKUP\x10\r\x12\x16\n\x12VISION_TYPE_\
    REMOVE\x10\x0e\x12\x1e\n\x1aVISION_TYPE_CHANGE_COSTUME\x10\x0f\x12\x1c\n\
    \x18VISION_TYPE_FISH_REFRESH\x10\x10\x12\x1e\n\x1aVISION_TYPE_FISH_BIG_S\
    HOCK\x10\x11\x12\x1d\n\x19VISION_TYPE_FISH_QTE_SUCC\x10\x12\x12!\n\x1dVI\
    SION_TYPE_CAPTURE_DISAPPEAR\x10\x13B\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            enums.push(VisionType::generated_enum_descriptor_data());
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
