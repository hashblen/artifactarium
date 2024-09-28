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

//! Generated file from `SelectWorktopOptionReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:SelectWorktopOptionReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SelectWorktopOptionReq {
    // message fields
    // @@protoc_insertion_point(field:SelectWorktopOptionReq.CHDDOFMLBLM)
    pub CHDDOFMLBLM: u32,
    // @@protoc_insertion_point(field:SelectWorktopOptionReq.gadget_entity_id)
    pub gadget_entity_id: u32,
    // @@protoc_insertion_point(field:SelectWorktopOptionReq.option_id)
    pub option_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SelectWorktopOptionReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SelectWorktopOptionReq {
    fn default() -> &'a SelectWorktopOptionReq {
        <SelectWorktopOptionReq as ::protobuf::Message>::default_instance()
    }
}

impl SelectWorktopOptionReq {
    pub fn new() -> SelectWorktopOptionReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CHDDOFMLBLM",
            |m: &SelectWorktopOptionReq| { &m.CHDDOFMLBLM },
            |m: &mut SelectWorktopOptionReq| { &mut m.CHDDOFMLBLM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gadget_entity_id",
            |m: &SelectWorktopOptionReq| { &m.gadget_entity_id },
            |m: &mut SelectWorktopOptionReq| { &mut m.gadget_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "option_id",
            |m: &SelectWorktopOptionReq| { &m.option_id },
            |m: &mut SelectWorktopOptionReq| { &mut m.option_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SelectWorktopOptionReq>(
            "SelectWorktopOptionReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SelectWorktopOptionReq {
    const NAME: &'static str = "SelectWorktopOptionReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.CHDDOFMLBLM = is.read_uint32()?;
                },
                8 => {
                    self.gadget_entity_id = is.read_uint32()?;
                },
                104 => {
                    self.option_id = is.read_uint32()?;
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
        if self.CHDDOFMLBLM != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.CHDDOFMLBLM);
        }
        if self.gadget_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.gadget_entity_id);
        }
        if self.option_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.option_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CHDDOFMLBLM != 0 {
            os.write_uint32(8, self.CHDDOFMLBLM)?;
        }
        if self.gadget_entity_id != 0 {
            os.write_uint32(1, self.gadget_entity_id)?;
        }
        if self.option_id != 0 {
            os.write_uint32(13, self.option_id)?;
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

    fn new() -> SelectWorktopOptionReq {
        SelectWorktopOptionReq::new()
    }

    fn clear(&mut self) {
        self.CHDDOFMLBLM = 0;
        self.gadget_entity_id = 0;
        self.option_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SelectWorktopOptionReq {
        static instance: SelectWorktopOptionReq = SelectWorktopOptionReq {
            CHDDOFMLBLM: 0,
            gadget_entity_id: 0,
            option_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SelectWorktopOptionReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SelectWorktopOptionReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SelectWorktopOptionReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelectWorktopOptionReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cSelectWorktopOptionReq.proto\"\x81\x01\n\x16SelectWorktopOptionReq\
    \x12\x20\n\x0bCHDDOFMLBLM\x18\x08\x20\x01(\rR\x0bCHDDOFMLBLM\x12(\n\x10g\
    adget_entity_id\x18\x01\x20\x01(\rR\x0egadgetEntityId\x12\x1b\n\toption_\
    id\x18\r\x20\x01(\rR\x08optionIdB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SelectWorktopOptionReq::generated_message_descriptor_data());
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
