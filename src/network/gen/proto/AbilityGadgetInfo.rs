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

//! Generated file from `AbilityGadgetInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:AbilityGadgetInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AbilityGadgetInfo {
    // message fields
    // @@protoc_insertion_point(field:AbilityGadgetInfo.camp_id)
    pub camp_id: u32,
    // @@protoc_insertion_point(field:AbilityGadgetInfo.camp_target_type)
    pub camp_target_type: u32,
    // @@protoc_insertion_point(field:AbilityGadgetInfo.target_entity_id)
    pub target_entity_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AbilityGadgetInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AbilityGadgetInfo {
    fn default() -> &'a AbilityGadgetInfo {
        <AbilityGadgetInfo as ::protobuf::Message>::default_instance()
    }
}

impl AbilityGadgetInfo {
    pub fn new() -> AbilityGadgetInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "camp_id",
            |m: &AbilityGadgetInfo| { &m.camp_id },
            |m: &mut AbilityGadgetInfo| { &mut m.camp_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "camp_target_type",
            |m: &AbilityGadgetInfo| { &m.camp_target_type },
            |m: &mut AbilityGadgetInfo| { &mut m.camp_target_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_entity_id",
            |m: &AbilityGadgetInfo| { &m.target_entity_id },
            |m: &mut AbilityGadgetInfo| { &mut m.target_entity_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AbilityGadgetInfo>(
            "AbilityGadgetInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AbilityGadgetInfo {
    const NAME: &'static str = "AbilityGadgetInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.camp_id = is.read_uint32()?;
                },
                16 => {
                    self.camp_target_type = is.read_uint32()?;
                },
                24 => {
                    self.target_entity_id = is.read_uint32()?;
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
        if self.camp_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.camp_id);
        }
        if self.camp_target_type != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.camp_target_type);
        }
        if self.target_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.target_entity_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.camp_id != 0 {
            os.write_uint32(1, self.camp_id)?;
        }
        if self.camp_target_type != 0 {
            os.write_uint32(2, self.camp_target_type)?;
        }
        if self.target_entity_id != 0 {
            os.write_uint32(3, self.target_entity_id)?;
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

    fn new() -> AbilityGadgetInfo {
        AbilityGadgetInfo::new()
    }

    fn clear(&mut self) {
        self.camp_id = 0;
        self.camp_target_type = 0;
        self.target_entity_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AbilityGadgetInfo {
        static instance: AbilityGadgetInfo = AbilityGadgetInfo {
            camp_id: 0,
            camp_target_type: 0,
            target_entity_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AbilityGadgetInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AbilityGadgetInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AbilityGadgetInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AbilityGadgetInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17AbilityGadgetInfo.proto\"\x80\x01\n\x11AbilityGadgetInfo\x12\x17\n\
    \x07camp_id\x18\x01\x20\x01(\rR\x06campId\x12(\n\x10camp_target_type\x18\
    \x02\x20\x01(\rR\x0ecampTargetType\x12(\n\x10target_entity_id\x18\x03\
    \x20\x01(\rR\x0etargetEntityIdB\x1b\n\x19emu.grasscutter.net.protob\x06p\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AbilityGadgetInfo::generated_message_descriptor_data());
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
