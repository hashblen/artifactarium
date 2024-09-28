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

//! Generated file from `TakeFurnitureMakeRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:TakeFurnitureMakeRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeFurnitureMakeRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeFurnitureMakeRsp.return_item_list)
    pub return_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:TakeFurnitureMakeRsp.furniture_make_slot)
    pub furniture_make_slot: ::protobuf::MessageField<super::FurnitureMakeSlot::FurnitureMakeSlot>,
    // @@protoc_insertion_point(field:TakeFurnitureMakeRsp.output_item_list)
    pub output_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:TakeFurnitureMakeRsp.make_id)
    pub make_id: u32,
    // @@protoc_insertion_point(field:TakeFurnitureMakeRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeFurnitureMakeRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeFurnitureMakeRsp {
    fn default() -> &'a TakeFurnitureMakeRsp {
        <TakeFurnitureMakeRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeFurnitureMakeRsp {
    pub fn new() -> TakeFurnitureMakeRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "return_item_list",
            |m: &TakeFurnitureMakeRsp| { &m.return_item_list },
            |m: &mut TakeFurnitureMakeRsp| { &mut m.return_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FurnitureMakeSlot::FurnitureMakeSlot>(
            "furniture_make_slot",
            |m: &TakeFurnitureMakeRsp| { &m.furniture_make_slot },
            |m: &mut TakeFurnitureMakeRsp| { &mut m.furniture_make_slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "output_item_list",
            |m: &TakeFurnitureMakeRsp| { &m.output_item_list },
            |m: &mut TakeFurnitureMakeRsp| { &mut m.output_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "make_id",
            |m: &TakeFurnitureMakeRsp| { &m.make_id },
            |m: &mut TakeFurnitureMakeRsp| { &mut m.make_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TakeFurnitureMakeRsp| { &m.retcode },
            |m: &mut TakeFurnitureMakeRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeFurnitureMakeRsp>(
            "TakeFurnitureMakeRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeFurnitureMakeRsp {
    const NAME: &'static str = "TakeFurnitureMakeRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    self.return_item_list.push(is.read_message()?);
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.furniture_make_slot)?;
                },
                18 => {
                    self.output_item_list.push(is.read_message()?);
                },
                88 => {
                    self.make_id = is.read_uint32()?;
                },
                24 => {
                    self.retcode = is.read_int32()?;
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
        for value in &self.return_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.furniture_make_slot.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.output_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.make_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.make_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.return_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if let Some(v) = self.furniture_make_slot.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        for v in &self.output_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.make_id != 0 {
            os.write_uint32(11, self.make_id)?;
        }
        if self.retcode != 0 {
            os.write_int32(3, self.retcode)?;
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

    fn new() -> TakeFurnitureMakeRsp {
        TakeFurnitureMakeRsp::new()
    }

    fn clear(&mut self) {
        self.return_item_list.clear();
        self.furniture_make_slot.clear();
        self.output_item_list.clear();
        self.make_id = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeFurnitureMakeRsp {
        static instance: TakeFurnitureMakeRsp = TakeFurnitureMakeRsp {
            return_item_list: ::std::vec::Vec::new(),
            furniture_make_slot: ::protobuf::MessageField::none(),
            output_item_list: ::std::vec::Vec::new(),
            make_id: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeFurnitureMakeRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeFurnitureMakeRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeFurnitureMakeRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeFurnitureMakeRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aTakeFurnitureMakeRsp.proto\x1a\x0fItemParam.proto\x1a\x17Furniture\
    MakeSlot.proto\"\xf9\x01\n\x14TakeFurnitureMakeRsp\x124\n\x10return_item\
    _list\x18\x07\x20\x03(\x0b2\n.ItemParamR\x0ereturnItemList\x12B\n\x13fur\
    niture_make_slot\x18\x0e\x20\x01(\x0b2\x12.FurnitureMakeSlotR\x11furnitu\
    reMakeSlot\x124\n\x10output_item_list\x18\x02\x20\x03(\x0b2\n.ItemParamR\
    \x0eoutputItemList\x12\x17\n\x07make_id\x18\x0b\x20\x01(\rR\x06makeId\
    \x12\x18\n\x07retcode\x18\x03\x20\x01(\x05R\x07retcodeB\x1b\n\x19emu.gra\
    sscutter.net.protob\x06proto3\
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
            deps.push(super::ItemParam::file_descriptor().clone());
            deps.push(super::FurnitureMakeSlot::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TakeFurnitureMakeRsp::generated_message_descriptor_data());
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
