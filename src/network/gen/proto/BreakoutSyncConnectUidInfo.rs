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

//! Generated file from `BreakoutSyncConnectUidInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:BreakoutSyncConnectUidInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BreakoutSyncConnectUidInfo {
    // message fields
    // @@protoc_insertion_point(field:BreakoutSyncConnectUidInfo.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:BreakoutSyncConnectUidInfo.skill_id_list)
    pub skill_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BreakoutSyncConnectUidInfo.skill_level_list)
    pub skill_level_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:BreakoutSyncConnectUidInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BreakoutSyncConnectUidInfo {
    fn default() -> &'a BreakoutSyncConnectUidInfo {
        <BreakoutSyncConnectUidInfo as ::protobuf::Message>::default_instance()
    }
}

impl BreakoutSyncConnectUidInfo {
    pub fn new() -> BreakoutSyncConnectUidInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &BreakoutSyncConnectUidInfo| { &m.uid },
            |m: &mut BreakoutSyncConnectUidInfo| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "skill_id_list",
            |m: &BreakoutSyncConnectUidInfo| { &m.skill_id_list },
            |m: &mut BreakoutSyncConnectUidInfo| { &mut m.skill_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "skill_level_list",
            |m: &BreakoutSyncConnectUidInfo| { &m.skill_level_list },
            |m: &mut BreakoutSyncConnectUidInfo| { &mut m.skill_level_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BreakoutSyncConnectUidInfo>(
            "BreakoutSyncConnectUidInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BreakoutSyncConnectUidInfo {
    const NAME: &'static str = "BreakoutSyncConnectUidInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.uid = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.skill_id_list)?;
                },
                16 => {
                    self.skill_id_list.push(is.read_uint32()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.skill_level_list)?;
                },
                24 => {
                    self.skill_level_list.push(is.read_uint32()?);
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
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.uid);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.skill_id_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(3, &self.skill_level_list);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.uid != 0 {
            os.write_uint32(1, self.uid)?;
        }
        os.write_repeated_packed_uint32(2, &self.skill_id_list)?;
        os.write_repeated_packed_uint32(3, &self.skill_level_list)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> BreakoutSyncConnectUidInfo {
        BreakoutSyncConnectUidInfo::new()
    }

    fn clear(&mut self) {
        self.uid = 0;
        self.skill_id_list.clear();
        self.skill_level_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BreakoutSyncConnectUidInfo {
        static instance: BreakoutSyncConnectUidInfo = BreakoutSyncConnectUidInfo {
            uid: 0,
            skill_id_list: ::std::vec::Vec::new(),
            skill_level_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BreakoutSyncConnectUidInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BreakoutSyncConnectUidInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BreakoutSyncConnectUidInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BreakoutSyncConnectUidInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20BreakoutSyncConnectUidInfo.proto\"|\n\x1aBreakoutSyncConnectUidInf\
    o\x12\x10\n\x03uid\x18\x01\x20\x01(\rR\x03uid\x12\"\n\rskill_id_list\x18\
    \x02\x20\x03(\rR\x0bskillIdList\x12(\n\x10skill_level_list\x18\x03\x20\
    \x03(\rR\x0eskillLevelListB\x1b\n\x19emu.grasscutter.net.protob\x06proto\
    3\
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
            messages.push(BreakoutSyncConnectUidInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
