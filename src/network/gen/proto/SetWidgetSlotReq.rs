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

//! Generated file from `SetWidgetSlotReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:SetWidgetSlotReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetWidgetSlotReq {
    // message fields
    // @@protoc_insertion_point(field:SetWidgetSlotReq.tag_list)
    pub tag_list: ::std::vec::Vec<::protobuf::EnumOrUnknown<super::WidgetSlotTag::WidgetSlotTag>>,
    // @@protoc_insertion_point(field:SetWidgetSlotReq.op)
    pub op: ::protobuf::EnumOrUnknown<super::WidgetSlotOp::WidgetSlotOp>,
    // @@protoc_insertion_point(field:SetWidgetSlotReq.material_id)
    pub material_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SetWidgetSlotReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetWidgetSlotReq {
    fn default() -> &'a SetWidgetSlotReq {
        <SetWidgetSlotReq as ::protobuf::Message>::default_instance()
    }
}

impl SetWidgetSlotReq {
    pub fn new() -> SetWidgetSlotReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "tag_list",
            |m: &SetWidgetSlotReq| { &m.tag_list },
            |m: &mut SetWidgetSlotReq| { &mut m.tag_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "op",
            |m: &SetWidgetSlotReq| { &m.op },
            |m: &mut SetWidgetSlotReq| { &mut m.op },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "material_id",
            |m: &SetWidgetSlotReq| { &m.material_id },
            |m: &mut SetWidgetSlotReq| { &mut m.material_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetWidgetSlotReq>(
            "SetWidgetSlotReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetWidgetSlotReq {
    const NAME: &'static str = "SetWidgetSlotReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.tag_list.push(is.read_enum_or_unknown()?);
                },
                18 => {
                    ::protobuf::rt::read_repeated_packed_enum_or_unknown_into(is, &mut self.tag_list)?
                },
                96 => {
                    self.op = is.read_enum_or_unknown()?;
                },
                64 => {
                    self.material_id = is.read_uint32()?;
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
        my_size += ::protobuf::rt::vec_packed_enum_or_unknown_size(2, &self.tag_list);
        if self.op != ::protobuf::EnumOrUnknown::new(super::WidgetSlotOp::WidgetSlotOp::WIDGET_SLOT_OP_ATTACH) {
            my_size += ::protobuf::rt::int32_size(12, self.op.value());
        }
        if self.material_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.material_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_enum_or_unknown(2, &self.tag_list)?;
        if self.op != ::protobuf::EnumOrUnknown::new(super::WidgetSlotOp::WidgetSlotOp::WIDGET_SLOT_OP_ATTACH) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.op))?;
        }
        if self.material_id != 0 {
            os.write_uint32(8, self.material_id)?;
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

    fn new() -> SetWidgetSlotReq {
        SetWidgetSlotReq::new()
    }

    fn clear(&mut self) {
        self.tag_list.clear();
        self.op = ::protobuf::EnumOrUnknown::new(super::WidgetSlotOp::WidgetSlotOp::WIDGET_SLOT_OP_ATTACH);
        self.material_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetWidgetSlotReq {
        static instance: SetWidgetSlotReq = SetWidgetSlotReq {
            tag_list: ::std::vec::Vec::new(),
            op: ::protobuf::EnumOrUnknown::from_i32(0),
            material_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetWidgetSlotReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetWidgetSlotReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetWidgetSlotReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetWidgetSlotReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16SetWidgetSlotReq.proto\x1a\x13WidgetSlotTag.proto\x1a\x12WidgetSlo\
    tOp.proto\"}\n\x10SetWidgetSlotReq\x12)\n\x08tag_list\x18\x02\x20\x03(\
    \x0e2\x0e.WidgetSlotTagR\x07tagList\x12\x1d\n\x02op\x18\x0c\x20\x01(\x0e\
    2\r.WidgetSlotOpR\x02op\x12\x1f\n\x0bmaterial_id\x18\x08\x20\x01(\rR\nma\
    terialIdB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::WidgetSlotTag::file_descriptor().clone());
            deps.push(super::WidgetSlotOp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetWidgetSlotReq::generated_message_descriptor_data());
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
