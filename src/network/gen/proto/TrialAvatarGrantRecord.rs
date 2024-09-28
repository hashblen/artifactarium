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

//! Generated file from `TrialAvatarGrantRecord.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:TrialAvatarGrantRecord)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrialAvatarGrantRecord {
    // message fields
    // @@protoc_insertion_point(field:TrialAvatarGrantRecord.grant_reason)
    pub grant_reason: u32,
    // @@protoc_insertion_point(field:TrialAvatarGrantRecord.from_parent_quest_id)
    pub from_parent_quest_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TrialAvatarGrantRecord.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrialAvatarGrantRecord {
    fn default() -> &'a TrialAvatarGrantRecord {
        <TrialAvatarGrantRecord as ::protobuf::Message>::default_instance()
    }
}

impl TrialAvatarGrantRecord {
    pub fn new() -> TrialAvatarGrantRecord {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "grant_reason",
            |m: &TrialAvatarGrantRecord| { &m.grant_reason },
            |m: &mut TrialAvatarGrantRecord| { &mut m.grant_reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "from_parent_quest_id",
            |m: &TrialAvatarGrantRecord| { &m.from_parent_quest_id },
            |m: &mut TrialAvatarGrantRecord| { &mut m.from_parent_quest_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrialAvatarGrantRecord>(
            "TrialAvatarGrantRecord",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrialAvatarGrantRecord {
    const NAME: &'static str = "TrialAvatarGrantRecord";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.grant_reason = is.read_uint32()?;
                },
                16 => {
                    self.from_parent_quest_id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.grant_reason != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.grant_reason);
        }
        if self.from_parent_quest_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.from_parent_quest_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.grant_reason != 0 {
            os.write_uint32(1, self.grant_reason)?;
        }
        if self.from_parent_quest_id != 0 {
            os.write_uint32(2, self.from_parent_quest_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TrialAvatarGrantRecord {
        TrialAvatarGrantRecord::new()
    }

    fn clear(&mut self) {
        self.grant_reason = 0;
        self.from_parent_quest_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrialAvatarGrantRecord {
        static instance: TrialAvatarGrantRecord = TrialAvatarGrantRecord {
            grant_reason: 0,
            from_parent_quest_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrialAvatarGrantRecord {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrialAvatarGrantRecord").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrialAvatarGrantRecord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrialAvatarGrantRecord {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `TrialAvatarGrantRecord`
pub mod trial_avatar_grant_record {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:TrialAvatarGrantRecord.GrantReason)
    pub enum GrantReason {
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_INVALID)
        GRANT_REASON_INVALID = 0,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_QUEST)
        GRANT_REASON_BY_QUEST = 1,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_TRIAL_AVATAR_ACTIVITY)
        GRANT_REASON_BY_TRIAL_AVATAR_ACTIVITY = 2,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_DUNGEON_ELEMENT_CHALLENGE)
        GRANT_REASON_BY_DUNGEON_ELEMENT_CHALLENGE = 3,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_MIST_TRIAL_ACTIVITY)
        GRANT_REASON_BY_MIST_TRIAL_ACTIVITY = 4,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_SUMO_ACTIVITY)
        GRANT_REASON_BY_SUMO_ACTIVITY = 5,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_POTION_ACTIVITY)
        GRANT_REASON_BY_POTION_ACTIVITY = 6,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_CRYSTAL_LINK_ACTIVITY)
        GRANT_REASON_BY_CRYSTAL_LINK_ACTIVITY = 7,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_IRODORI_MASTER)
        GRANT_REASON_BY_IRODORI_MASTER = 8,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_GM)
        GRANT_REASON_BY_GM = 9,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_INSTABLE_SPRAY_ACTIVITY)
        GRANT_REASON_BY_INSTABLE_SPRAY_ACTIVITY = 10,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_MUQADAS_POTION_ACTIVITY)
        GRANT_REASON_BY_MUQADAS_POTION_ACTIVITY = 11,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_VINTAGE_HUNTING)
        GRANT_REASON_BY_VINTAGE_HUNTING = 12,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.GRANT_REASON_BY_CHAR_AMUSEMENT)
        GRANT_REASON_BY_CHAR_AMUSEMENT = 13,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.FJBKKFJNBHD_HMLDNDBKNFL)
        FJBKKFJNBHD_HMLDNDBKNFL = 14,
        // @@protoc_insertion_point(enum_value:TrialAvatarGrantRecord.GrantReason.FJBKKFJNBHD_PJDEJIHFCPP)
        FJBKKFJNBHD_PJDEJIHFCPP = 15,
    }

    impl ::protobuf::Enum for GrantReason {
        const NAME: &'static str = "GrantReason";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<GrantReason> {
            match value {
                0 => ::std::option::Option::Some(GrantReason::GRANT_REASON_INVALID),
                1 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_QUEST),
                2 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_TRIAL_AVATAR_ACTIVITY),
                3 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_DUNGEON_ELEMENT_CHALLENGE),
                4 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_MIST_TRIAL_ACTIVITY),
                5 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_SUMO_ACTIVITY),
                6 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_POTION_ACTIVITY),
                7 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_CRYSTAL_LINK_ACTIVITY),
                8 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_IRODORI_MASTER),
                9 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_GM),
                10 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_INSTABLE_SPRAY_ACTIVITY),
                11 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_MUQADAS_POTION_ACTIVITY),
                12 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_VINTAGE_HUNTING),
                13 => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_CHAR_AMUSEMENT),
                14 => ::std::option::Option::Some(GrantReason::FJBKKFJNBHD_HMLDNDBKNFL),
                15 => ::std::option::Option::Some(GrantReason::FJBKKFJNBHD_PJDEJIHFCPP),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<GrantReason> {
            match str {
                "GRANT_REASON_INVALID" => ::std::option::Option::Some(GrantReason::GRANT_REASON_INVALID),
                "GRANT_REASON_BY_QUEST" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_QUEST),
                "GRANT_REASON_BY_TRIAL_AVATAR_ACTIVITY" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_TRIAL_AVATAR_ACTIVITY),
                "GRANT_REASON_BY_DUNGEON_ELEMENT_CHALLENGE" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_DUNGEON_ELEMENT_CHALLENGE),
                "GRANT_REASON_BY_MIST_TRIAL_ACTIVITY" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_MIST_TRIAL_ACTIVITY),
                "GRANT_REASON_BY_SUMO_ACTIVITY" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_SUMO_ACTIVITY),
                "GRANT_REASON_BY_POTION_ACTIVITY" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_POTION_ACTIVITY),
                "GRANT_REASON_BY_CRYSTAL_LINK_ACTIVITY" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_CRYSTAL_LINK_ACTIVITY),
                "GRANT_REASON_BY_IRODORI_MASTER" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_IRODORI_MASTER),
                "GRANT_REASON_BY_GM" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_GM),
                "GRANT_REASON_BY_INSTABLE_SPRAY_ACTIVITY" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_INSTABLE_SPRAY_ACTIVITY),
                "GRANT_REASON_BY_MUQADAS_POTION_ACTIVITY" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_MUQADAS_POTION_ACTIVITY),
                "GRANT_REASON_BY_VINTAGE_HUNTING" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_VINTAGE_HUNTING),
                "GRANT_REASON_BY_CHAR_AMUSEMENT" => ::std::option::Option::Some(GrantReason::GRANT_REASON_BY_CHAR_AMUSEMENT),
                "FJBKKFJNBHD_HMLDNDBKNFL" => ::std::option::Option::Some(GrantReason::FJBKKFJNBHD_HMLDNDBKNFL),
                "FJBKKFJNBHD_PJDEJIHFCPP" => ::std::option::Option::Some(GrantReason::FJBKKFJNBHD_PJDEJIHFCPP),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [GrantReason] = &[
            GrantReason::GRANT_REASON_INVALID,
            GrantReason::GRANT_REASON_BY_QUEST,
            GrantReason::GRANT_REASON_BY_TRIAL_AVATAR_ACTIVITY,
            GrantReason::GRANT_REASON_BY_DUNGEON_ELEMENT_CHALLENGE,
            GrantReason::GRANT_REASON_BY_MIST_TRIAL_ACTIVITY,
            GrantReason::GRANT_REASON_BY_SUMO_ACTIVITY,
            GrantReason::GRANT_REASON_BY_POTION_ACTIVITY,
            GrantReason::GRANT_REASON_BY_CRYSTAL_LINK_ACTIVITY,
            GrantReason::GRANT_REASON_BY_IRODORI_MASTER,
            GrantReason::GRANT_REASON_BY_GM,
            GrantReason::GRANT_REASON_BY_INSTABLE_SPRAY_ACTIVITY,
            GrantReason::GRANT_REASON_BY_MUQADAS_POTION_ACTIVITY,
            GrantReason::GRANT_REASON_BY_VINTAGE_HUNTING,
            GrantReason::GRANT_REASON_BY_CHAR_AMUSEMENT,
            GrantReason::FJBKKFJNBHD_HMLDNDBKNFL,
            GrantReason::FJBKKFJNBHD_PJDEJIHFCPP,
        ];
    }

    impl ::protobuf::EnumFull for GrantReason {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("TrialAvatarGrantRecord.GrantReason").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for GrantReason {
        fn default() -> Self {
            GrantReason::GRANT_REASON_INVALID
        }
    }

    impl GrantReason {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<GrantReason>("TrialAvatarGrantRecord.GrantReason")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cTrialAvatarGrantRecord.proto\"\xc0\x05\n\x16TrialAvatarGrantRecord\
    \x12!\n\x0cgrant_reason\x18\x01\x20\x01(\rR\x0bgrantReason\x12/\n\x14fro\
    m_parent_quest_id\x18\x02\x20\x01(\rR\x11fromParentQuestId\"\xd1\x04\n\
    \x0bGrantReason\x12\x18\n\x14GRANT_REASON_INVALID\x10\0\x12\x19\n\x15GRA\
    NT_REASON_BY_QUEST\x10\x01\x12)\n%GRANT_REASON_BY_TRIAL_AVATAR_ACTIVITY\
    \x10\x02\x12-\n)GRANT_REASON_BY_DUNGEON_ELEMENT_CHALLENGE\x10\x03\x12'\n\
    #GRANT_REASON_BY_MIST_TRIAL_ACTIVITY\x10\x04\x12!\n\x1dGRANT_REASON_BY_S\
    UMO_ACTIVITY\x10\x05\x12#\n\x1fGRANT_REASON_BY_POTION_ACTIVITY\x10\x06\
    \x12)\n%GRANT_REASON_BY_CRYSTAL_LINK_ACTIVITY\x10\x07\x12\"\n\x1eGRANT_R\
    EASON_BY_IRODORI_MASTER\x10\x08\x12\x16\n\x12GRANT_REASON_BY_GM\x10\t\
    \x12+\n'GRANT_REASON_BY_INSTABLE_SPRAY_ACTIVITY\x10\n\x12+\n'GRANT_REASO\
    N_BY_MUQADAS_POTION_ACTIVITY\x10\x0b\x12#\n\x1fGRANT_REASON_BY_VINTAGE_H\
    UNTING\x10\x0c\x12\"\n\x1eGRANT_REASON_BY_CHAR_AMUSEMENT\x10\r\x12\x1b\n\
    \x17FJBKKFJNBHD_HMLDNDBKNFL\x10\x0e\x12\x1b\n\x17FJBKKFJNBHD_PJDEJIHFCPP\
    \x10\x0fB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrialAvatarGrantRecord::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(trial_avatar_grant_record::GrantReason::generated_enum_descriptor_data());
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
