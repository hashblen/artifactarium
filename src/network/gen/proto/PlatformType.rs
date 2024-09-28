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

//! Generated file from `PlatformType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:PlatformType)
pub enum PlatformType {
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_EDITOR)
    PLATFORM_TYPE_EDITOR = 0,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_IOS)
    PLATFORM_TYPE_IOS = 1,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_ANDROID)
    PLATFORM_TYPE_ANDROID = 2,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_PC)
    PLATFORM_TYPE_PC = 3,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_PS4)
    PLATFORM_TYPE_PS4 = 4,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_SERVER)
    PLATFORM_TYPE_SERVER = 5,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_ANDROID)
    PLATFORM_TYPE_CLOUD_ANDROID = 6,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_IOS)
    PLATFORM_TYPE_CLOUD_IOS = 7,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_PS5)
    PLATFORM_TYPE_PS5 = 8,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_WEB)
    PLATFORM_TYPE_CLOUD_WEB = 9,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_TV)
    PLATFORM_TYPE_CLOUD_TV = 10,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_MAC)
    PLATFORM_TYPE_CLOUD_MAC = 11,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_PC)
    PLATFORM_TYPE_CLOUD_PC = 12,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_THIRD_PARTY_MOBILE)
    PLATFORM_TYPE_CLOUD_THIRD_PARTY_MOBILE = 13,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_THIRD_PARTY_PC)
    PLATFORM_TYPE_CLOUD_THIRD_PARTY_PC = 14,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_WEB_ANDROID)
    PLATFORM_TYPE_CLOUD_WEB_ANDROID = 15,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_WEB_IOS)
    PLATFORM_TYPE_CLOUD_WEB_IOS = 16,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_WEB_PC)
    PLATFORM_TYPE_CLOUD_WEB_PC = 17,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_WEB_MAC)
    PLATFORM_TYPE_CLOUD_WEB_MAC = 18,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_WEB_TOUCH)
    PLATFORM_TYPE_CLOUD_WEB_TOUCH = 19,
    // @@protoc_insertion_point(enum_value:PlatformType.PLATFORM_TYPE_CLOUD_WEB_KEYBOARD)
    PLATFORM_TYPE_CLOUD_WEB_KEYBOARD = 20,
}

impl ::protobuf::Enum for PlatformType {
    const NAME: &'static str = "PlatformType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PlatformType> {
        match value {
            0 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_EDITOR),
            1 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_IOS),
            2 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_ANDROID),
            3 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_PC),
            4 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_PS4),
            5 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_SERVER),
            6 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_ANDROID),
            7 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_IOS),
            8 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_PS5),
            9 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB),
            10 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_TV),
            11 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_MAC),
            12 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_PC),
            13 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_THIRD_PARTY_MOBILE),
            14 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_THIRD_PARTY_PC),
            15 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_ANDROID),
            16 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_IOS),
            17 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_PC),
            18 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_MAC),
            19 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_TOUCH),
            20 => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_KEYBOARD),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<PlatformType> {
        match str {
            "PLATFORM_TYPE_EDITOR" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_EDITOR),
            "PLATFORM_TYPE_IOS" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_IOS),
            "PLATFORM_TYPE_ANDROID" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_ANDROID),
            "PLATFORM_TYPE_PC" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_PC),
            "PLATFORM_TYPE_PS4" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_PS4),
            "PLATFORM_TYPE_SERVER" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_SERVER),
            "PLATFORM_TYPE_CLOUD_ANDROID" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_ANDROID),
            "PLATFORM_TYPE_CLOUD_IOS" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_IOS),
            "PLATFORM_TYPE_PS5" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_PS5),
            "PLATFORM_TYPE_CLOUD_WEB" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB),
            "PLATFORM_TYPE_CLOUD_TV" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_TV),
            "PLATFORM_TYPE_CLOUD_MAC" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_MAC),
            "PLATFORM_TYPE_CLOUD_PC" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_PC),
            "PLATFORM_TYPE_CLOUD_THIRD_PARTY_MOBILE" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_THIRD_PARTY_MOBILE),
            "PLATFORM_TYPE_CLOUD_THIRD_PARTY_PC" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_THIRD_PARTY_PC),
            "PLATFORM_TYPE_CLOUD_WEB_ANDROID" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_ANDROID),
            "PLATFORM_TYPE_CLOUD_WEB_IOS" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_IOS),
            "PLATFORM_TYPE_CLOUD_WEB_PC" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_PC),
            "PLATFORM_TYPE_CLOUD_WEB_MAC" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_MAC),
            "PLATFORM_TYPE_CLOUD_WEB_TOUCH" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_TOUCH),
            "PLATFORM_TYPE_CLOUD_WEB_KEYBOARD" => ::std::option::Option::Some(PlatformType::PLATFORM_TYPE_CLOUD_WEB_KEYBOARD),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PlatformType] = &[
        PlatformType::PLATFORM_TYPE_EDITOR,
        PlatformType::PLATFORM_TYPE_IOS,
        PlatformType::PLATFORM_TYPE_ANDROID,
        PlatformType::PLATFORM_TYPE_PC,
        PlatformType::PLATFORM_TYPE_PS4,
        PlatformType::PLATFORM_TYPE_SERVER,
        PlatformType::PLATFORM_TYPE_CLOUD_ANDROID,
        PlatformType::PLATFORM_TYPE_CLOUD_IOS,
        PlatformType::PLATFORM_TYPE_PS5,
        PlatformType::PLATFORM_TYPE_CLOUD_WEB,
        PlatformType::PLATFORM_TYPE_CLOUD_TV,
        PlatformType::PLATFORM_TYPE_CLOUD_MAC,
        PlatformType::PLATFORM_TYPE_CLOUD_PC,
        PlatformType::PLATFORM_TYPE_CLOUD_THIRD_PARTY_MOBILE,
        PlatformType::PLATFORM_TYPE_CLOUD_THIRD_PARTY_PC,
        PlatformType::PLATFORM_TYPE_CLOUD_WEB_ANDROID,
        PlatformType::PLATFORM_TYPE_CLOUD_WEB_IOS,
        PlatformType::PLATFORM_TYPE_CLOUD_WEB_PC,
        PlatformType::PLATFORM_TYPE_CLOUD_WEB_MAC,
        PlatformType::PLATFORM_TYPE_CLOUD_WEB_TOUCH,
        PlatformType::PLATFORM_TYPE_CLOUD_WEB_KEYBOARD,
    ];
}

impl ::protobuf::EnumFull for PlatformType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PlatformType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PlatformType {
    fn default() -> Self {
        PlatformType::PLATFORM_TYPE_EDITOR
    }
}

impl PlatformType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PlatformType>("PlatformType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12PlatformType.proto*\x8c\x05\n\x0cPlatformType\x12\x18\n\x14PLATFOR\
    M_TYPE_EDITOR\x10\0\x12\x15\n\x11PLATFORM_TYPE_IOS\x10\x01\x12\x19\n\x15\
    PLATFORM_TYPE_ANDROID\x10\x02\x12\x14\n\x10PLATFORM_TYPE_PC\x10\x03\x12\
    \x15\n\x11PLATFORM_TYPE_PS4\x10\x04\x12\x18\n\x14PLATFORM_TYPE_SERVER\
    \x10\x05\x12\x1f\n\x1bPLATFORM_TYPE_CLOUD_ANDROID\x10\x06\x12\x1b\n\x17P\
    LATFORM_TYPE_CLOUD_IOS\x10\x07\x12\x15\n\x11PLATFORM_TYPE_PS5\x10\x08\
    \x12\x1b\n\x17PLATFORM_TYPE_CLOUD_WEB\x10\t\x12\x1a\n\x16PLATFORM_TYPE_C\
    LOUD_TV\x10\n\x12\x1b\n\x17PLATFORM_TYPE_CLOUD_MAC\x10\x0b\x12\x1a\n\x16\
    PLATFORM_TYPE_CLOUD_PC\x10\x0c\x12*\n&PLATFORM_TYPE_CLOUD_THIRD_PARTY_MO\
    BILE\x10\r\x12&\n\"PLATFORM_TYPE_CLOUD_THIRD_PARTY_PC\x10\x0e\x12#\n\x1f\
    PLATFORM_TYPE_CLOUD_WEB_ANDROID\x10\x0f\x12\x1f\n\x1bPLATFORM_TYPE_CLOUD\
    _WEB_IOS\x10\x10\x12\x1e\n\x1aPLATFORM_TYPE_CLOUD_WEB_PC\x10\x11\x12\x1f\
    \n\x1bPLATFORM_TYPE_CLOUD_WEB_MAC\x10\x12\x12!\n\x1dPLATFORM_TYPE_CLOUD_\
    WEB_TOUCH\x10\x13\x12$\n\x20PLATFORM_TYPE_CLOUD_WEB_KEYBOARD\x10\x14B\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            enums.push(PlatformType::generated_enum_descriptor_data());
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
