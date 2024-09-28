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

//! Generated file from `ChangeHpDebtsReason.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:ChangeHpDebtsReason)
pub enum ChangeHpDebtsReason {
    // @@protoc_insertion_point(enum_value:ChangeHpDebtsReason.CHANGE_HP_DEBTS_NONE)
    CHANGE_HP_DEBTS_NONE = 0,
    // @@protoc_insertion_point(enum_value:ChangeHpDebtsReason.CHANGE_HP_DEBTS_PAY)
    CHANGE_HP_DEBTS_PAY = 1,
    // @@protoc_insertion_point(enum_value:ChangeHpDebtsReason.CHANGE_HP_DEBTS_PAY_FINISH)
    CHANGE_HP_DEBTS_PAY_FINISH = 2,
    // @@protoc_insertion_point(enum_value:ChangeHpDebtsReason.CHANGE_HP_DEBTS_CLEAR)
    CHANGE_HP_DEBTS_CLEAR = 21,
    // @@protoc_insertion_point(enum_value:ChangeHpDebtsReason.CHANGE_HP_DEBTS_REDUCE_ABILITY)
    CHANGE_HP_DEBTS_REDUCE_ABILITY = 41,
    // @@protoc_insertion_point(enum_value:ChangeHpDebtsReason.CHANGE_HP_DEBTS_ADD_ABILITY)
    CHANGE_HP_DEBTS_ADD_ABILITY = 51,
}

impl ::protobuf::Enum for ChangeHpDebtsReason {
    const NAME: &'static str = "ChangeHpDebtsReason";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChangeHpDebtsReason> {
        match value {
            0 => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE),
            1 => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY),
            2 => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY_FINISH),
            21 => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_CLEAR),
            41 => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_REDUCE_ABILITY),
            51 => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_ADD_ABILITY),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<ChangeHpDebtsReason> {
        match str {
            "CHANGE_HP_DEBTS_NONE" => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE),
            "CHANGE_HP_DEBTS_PAY" => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY),
            "CHANGE_HP_DEBTS_PAY_FINISH" => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY_FINISH),
            "CHANGE_HP_DEBTS_CLEAR" => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_CLEAR),
            "CHANGE_HP_DEBTS_REDUCE_ABILITY" => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_REDUCE_ABILITY),
            "CHANGE_HP_DEBTS_ADD_ABILITY" => ::std::option::Option::Some(ChangeHpDebtsReason::CHANGE_HP_DEBTS_ADD_ABILITY),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ChangeHpDebtsReason] = &[
        ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE,
        ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY,
        ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY_FINISH,
        ChangeHpDebtsReason::CHANGE_HP_DEBTS_CLEAR,
        ChangeHpDebtsReason::CHANGE_HP_DEBTS_REDUCE_ABILITY,
        ChangeHpDebtsReason::CHANGE_HP_DEBTS_ADD_ABILITY,
    ];
}

impl ::protobuf::EnumFull for ChangeHpDebtsReason {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ChangeHpDebtsReason").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE => 0,
            ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY => 1,
            ChangeHpDebtsReason::CHANGE_HP_DEBTS_PAY_FINISH => 2,
            ChangeHpDebtsReason::CHANGE_HP_DEBTS_CLEAR => 3,
            ChangeHpDebtsReason::CHANGE_HP_DEBTS_REDUCE_ABILITY => 4,
            ChangeHpDebtsReason::CHANGE_HP_DEBTS_ADD_ABILITY => 5,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ChangeHpDebtsReason {
    fn default() -> Self {
        ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE
    }
}

impl ChangeHpDebtsReason {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ChangeHpDebtsReason>("ChangeHpDebtsReason")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19ChangeHpDebtsReason.proto*\xc8\x01\n\x13ChangeHpDebtsReason\x12\
    \x18\n\x14CHANGE_HP_DEBTS_NONE\x10\0\x12\x17\n\x13CHANGE_HP_DEBTS_PAY\
    \x10\x01\x12\x1e\n\x1aCHANGE_HP_DEBTS_PAY_FINISH\x10\x02\x12\x19\n\x15CH\
    ANGE_HP_DEBTS_CLEAR\x10\x15\x12\"\n\x1eCHANGE_HP_DEBTS_REDUCE_ABILITY\
    \x10)\x12\x1f\n\x1bCHANGE_HP_DEBTS_ADD_ABILITY\x103B\x1b\n\x19emu.grassc\
    utter.net.protob\x06proto3\
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
            enums.push(ChangeHpDebtsReason::generated_enum_descriptor_data());
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
