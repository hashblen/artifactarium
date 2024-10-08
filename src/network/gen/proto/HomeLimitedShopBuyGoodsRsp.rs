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

//! Generated file from `HomeLimitedShopBuyGoodsRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:HomeLimitedShopBuyGoodsRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeLimitedShopBuyGoodsRsp {
    // message fields
    // @@protoc_insertion_point(field:HomeLimitedShopBuyGoodsRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:HomeLimitedShopBuyGoodsRsp.buy_count)
    pub buy_count: u32,
    // @@protoc_insertion_point(field:HomeLimitedShopBuyGoodsRsp.goods)
    pub goods: ::protobuf::MessageField<super::HomeLimitedShopGoods::HomeLimitedShopGoods>,
    // @@protoc_insertion_point(field:HomeLimitedShopBuyGoodsRsp.goods_list)
    pub goods_list: ::std::vec::Vec<super::HomeLimitedShopGoods::HomeLimitedShopGoods>,
    // special fields
    // @@protoc_insertion_point(special_field:HomeLimitedShopBuyGoodsRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeLimitedShopBuyGoodsRsp {
    fn default() -> &'a HomeLimitedShopBuyGoodsRsp {
        <HomeLimitedShopBuyGoodsRsp as ::protobuf::Message>::default_instance()
    }
}

impl HomeLimitedShopBuyGoodsRsp {
    pub fn new() -> HomeLimitedShopBuyGoodsRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &HomeLimitedShopBuyGoodsRsp| { &m.retcode },
            |m: &mut HomeLimitedShopBuyGoodsRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buy_count",
            |m: &HomeLimitedShopBuyGoodsRsp| { &m.buy_count },
            |m: &mut HomeLimitedShopBuyGoodsRsp| { &mut m.buy_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HomeLimitedShopGoods::HomeLimitedShopGoods>(
            "goods",
            |m: &HomeLimitedShopBuyGoodsRsp| { &m.goods },
            |m: &mut HomeLimitedShopBuyGoodsRsp| { &mut m.goods },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "goods_list",
            |m: &HomeLimitedShopBuyGoodsRsp| { &m.goods_list },
            |m: &mut HomeLimitedShopBuyGoodsRsp| { &mut m.goods_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeLimitedShopBuyGoodsRsp>(
            "HomeLimitedShopBuyGoodsRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeLimitedShopBuyGoodsRsp {
    const NAME: &'static str = "HomeLimitedShopBuyGoodsRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.retcode = is.read_int32()?;
                },
                56 => {
                    self.buy_count = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.goods)?;
                },
                18 => {
                    self.goods_list.push(is.read_message()?);
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(14, self.retcode);
        }
        if self.buy_count != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.buy_count);
        }
        if let Some(v) = self.goods.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.goods_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_int32(14, self.retcode)?;
        }
        if self.buy_count != 0 {
            os.write_uint32(7, self.buy_count)?;
        }
        if let Some(v) = self.goods.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        for v in &self.goods_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> HomeLimitedShopBuyGoodsRsp {
        HomeLimitedShopBuyGoodsRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.buy_count = 0;
        self.goods.clear();
        self.goods_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeLimitedShopBuyGoodsRsp {
        static instance: HomeLimitedShopBuyGoodsRsp = HomeLimitedShopBuyGoodsRsp {
            retcode: 0,
            buy_count: 0,
            goods: ::protobuf::MessageField::none(),
            goods_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeLimitedShopBuyGoodsRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeLimitedShopBuyGoodsRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeLimitedShopBuyGoodsRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeLimitedShopBuyGoodsRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20HomeLimitedShopBuyGoodsRsp.proto\x1a\x1aHomeLimitedShopGoods.proto\
    \"\xb6\x01\n\x1aHomeLimitedShopBuyGoodsRsp\x12\x18\n\x07retcode\x18\x0e\
    \x20\x01(\x05R\x07retcode\x12\x1b\n\tbuy_count\x18\x07\x20\x01(\rR\x08bu\
    yCount\x12+\n\x05goods\x18\x0f\x20\x01(\x0b2\x15.HomeLimitedShopGoodsR\
    \x05goods\x124\n\ngoods_list\x18\x02\x20\x03(\x0b2\x15.HomeLimitedShopGo\
    odsR\tgoodsListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::HomeLimitedShopGoods::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HomeLimitedShopBuyGoodsRsp::generated_message_descriptor_data());
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
