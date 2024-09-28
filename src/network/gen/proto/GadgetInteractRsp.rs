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

//! Generated file from `GadgetInteractRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:GadgetInteractRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GadgetInteractRsp {
    // message fields
    // @@protoc_insertion_point(field:GadgetInteractRsp.op_type)
    pub op_type: ::protobuf::EnumOrUnknown<super::InterOpType::InterOpType>,
    // @@protoc_insertion_point(field:GadgetInteractRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:GadgetInteractRsp.gadget_entity_id)
    pub gadget_entity_id: u32,
    // @@protoc_insertion_point(field:GadgetInteractRsp.interact_type)
    pub interact_type: ::protobuf::EnumOrUnknown<super::InteractType::InteractType>,
    // @@protoc_insertion_point(field:GadgetInteractRsp.gadget_id)
    pub gadget_id: u32,
    // @@protoc_insertion_point(field:GadgetInteractRsp.CHDDOFMLBLM)
    pub CHDDOFMLBLM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GadgetInteractRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GadgetInteractRsp {
    fn default() -> &'a GadgetInteractRsp {
        <GadgetInteractRsp as ::protobuf::Message>::default_instance()
    }
}

impl GadgetInteractRsp {
    pub fn new() -> GadgetInteractRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "op_type",
            |m: &GadgetInteractRsp| { &m.op_type },
            |m: &mut GadgetInteractRsp| { &mut m.op_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GadgetInteractRsp| { &m.retcode },
            |m: &mut GadgetInteractRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gadget_entity_id",
            |m: &GadgetInteractRsp| { &m.gadget_entity_id },
            |m: &mut GadgetInteractRsp| { &mut m.gadget_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "interact_type",
            |m: &GadgetInteractRsp| { &m.interact_type },
            |m: &mut GadgetInteractRsp| { &mut m.interact_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gadget_id",
            |m: &GadgetInteractRsp| { &m.gadget_id },
            |m: &mut GadgetInteractRsp| { &mut m.gadget_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CHDDOFMLBLM",
            |m: &GadgetInteractRsp| { &m.CHDDOFMLBLM },
            |m: &mut GadgetInteractRsp| { &mut m.CHDDOFMLBLM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GadgetInteractRsp>(
            "GadgetInteractRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GadgetInteractRsp {
    const NAME: &'static str = "GadgetInteractRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.op_type = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.retcode = is.read_int32()?;
                },
                48 => {
                    self.gadget_entity_id = is.read_uint32()?;
                },
                64 => {
                    self.interact_type = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.gadget_id = is.read_uint32()?;
                },
                88 => {
                    self.CHDDOFMLBLM = is.read_uint32()?;
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
        if self.op_type != ::protobuf::EnumOrUnknown::new(super::InterOpType::InterOpType::INTER_OP_TYPE_FINISH) {
            my_size += ::protobuf::rt::int32_size(1, self.op_type.value());
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.retcode);
        }
        if self.gadget_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.gadget_entity_id);
        }
        if self.interact_type != ::protobuf::EnumOrUnknown::new(super::InteractType::InteractType::INTERACT_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.interact_type.value());
        }
        if self.gadget_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.gadget_id);
        }
        if self.CHDDOFMLBLM != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.CHDDOFMLBLM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.op_type != ::protobuf::EnumOrUnknown::new(super::InterOpType::InterOpType::INTER_OP_TYPE_FINISH) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.op_type))?;
        }
        if self.retcode != 0 {
            os.write_int32(5, self.retcode)?;
        }
        if self.gadget_entity_id != 0 {
            os.write_uint32(6, self.gadget_entity_id)?;
        }
        if self.interact_type != ::protobuf::EnumOrUnknown::new(super::InteractType::InteractType::INTERACT_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.interact_type))?;
        }
        if self.gadget_id != 0 {
            os.write_uint32(9, self.gadget_id)?;
        }
        if self.CHDDOFMLBLM != 0 {
            os.write_uint32(11, self.CHDDOFMLBLM)?;
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

    fn new() -> GadgetInteractRsp {
        GadgetInteractRsp::new()
    }

    fn clear(&mut self) {
        self.op_type = ::protobuf::EnumOrUnknown::new(super::InterOpType::InterOpType::INTER_OP_TYPE_FINISH);
        self.retcode = 0;
        self.gadget_entity_id = 0;
        self.interact_type = ::protobuf::EnumOrUnknown::new(super::InteractType::InteractType::INTERACT_TYPE_NONE);
        self.gadget_id = 0;
        self.CHDDOFMLBLM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GadgetInteractRsp {
        static instance: GadgetInteractRsp = GadgetInteractRsp {
            op_type: ::protobuf::EnumOrUnknown::from_i32(0),
            retcode: 0,
            gadget_entity_id: 0,
            interact_type: ::protobuf::EnumOrUnknown::from_i32(0),
            gadget_id: 0,
            CHDDOFMLBLM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GadgetInteractRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GadgetInteractRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GadgetInteractRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GadgetInteractRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GadgetInteractRsp.proto\x1a\x11InterOpType.proto\x1a\x12InteractTy\
    pe.proto\"\xf1\x01\n\x11GadgetInteractRsp\x12%\n\x07op_type\x18\x01\x20\
    \x01(\x0e2\x0c.InterOpTypeR\x06opType\x12\x18\n\x07retcode\x18\x05\x20\
    \x01(\x05R\x07retcode\x12(\n\x10gadget_entity_id\x18\x06\x20\x01(\rR\x0e\
    gadgetEntityId\x122\n\rinteract_type\x18\x08\x20\x01(\x0e2\r.InteractTyp\
    eR\x0cinteractType\x12\x1b\n\tgadget_id\x18\t\x20\x01(\rR\x08gadgetId\
    \x12\x20\n\x0bCHDDOFMLBLM\x18\x0b\x20\x01(\rR\x0bCHDDOFMLBLMB\x1b\n\x19e\
    mu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::InterOpType::file_descriptor().clone());
            deps.push(super::InteractType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GadgetInteractRsp::generated_message_descriptor_data());
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
