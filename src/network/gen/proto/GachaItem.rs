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

//! Generated file from `GachaItem.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:GachaItem)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GachaItem {
    // message fields
    // @@protoc_insertion_point(field:GachaItem.gacha_item)
    pub gacha_item: ::protobuf::MessageField<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:GachaItem.is_flash_card)
    pub is_flash_card: bool,
    // @@protoc_insertion_point(field:GachaItem.is_gacha_item_new)
    pub is_gacha_item_new: bool,
    // @@protoc_insertion_point(field:GachaItem.token_item_list)
    pub token_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:GachaItem.transfer_items)
    pub transfer_items: ::std::vec::Vec<super::GachaTransferItem::GachaTransferItem>,
    // special fields
    // @@protoc_insertion_point(special_field:GachaItem.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GachaItem {
    fn default() -> &'a GachaItem {
        <GachaItem as ::protobuf::Message>::default_instance()
    }
}

impl GachaItem {
    pub fn new() -> GachaItem {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemParam::ItemParam>(
            "gacha_item",
            |m: &GachaItem| { &m.gacha_item },
            |m: &mut GachaItem| { &mut m.gacha_item },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_flash_card",
            |m: &GachaItem| { &m.is_flash_card },
            |m: &mut GachaItem| { &mut m.is_flash_card },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_gacha_item_new",
            |m: &GachaItem| { &m.is_gacha_item_new },
            |m: &mut GachaItem| { &mut m.is_gacha_item_new },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "token_item_list",
            |m: &GachaItem| { &m.token_item_list },
            |m: &mut GachaItem| { &mut m.token_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "transfer_items",
            |m: &GachaItem| { &m.transfer_items },
            |m: &mut GachaItem| { &mut m.transfer_items },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GachaItem>(
            "GachaItem",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GachaItem {
    const NAME: &'static str = "GachaItem";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.gacha_item)?;
                },
                32 => {
                    self.is_flash_card = is.read_bool()?;
                },
                64 => {
                    self.is_gacha_item_new = is.read_bool()?;
                },
                114 => {
                    self.token_item_list.push(is.read_message()?);
                },
                122 => {
                    self.transfer_items.push(is.read_message()?);
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
        if let Some(v) = self.gacha_item.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.is_flash_card != false {
            my_size += 1 + 1;
        }
        if self.is_gacha_item_new != false {
            my_size += 1 + 1;
        }
        for value in &self.token_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.transfer_items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.gacha_item.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.is_flash_card != false {
            os.write_bool(4, self.is_flash_card)?;
        }
        if self.is_gacha_item_new != false {
            os.write_bool(8, self.is_gacha_item_new)?;
        }
        for v in &self.token_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        for v in &self.transfer_items {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> GachaItem {
        GachaItem::new()
    }

    fn clear(&mut self) {
        self.gacha_item.clear();
        self.is_flash_card = false;
        self.is_gacha_item_new = false;
        self.token_item_list.clear();
        self.transfer_items.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GachaItem {
        static instance: GachaItem = GachaItem {
            gacha_item: ::protobuf::MessageField::none(),
            is_flash_card: false,
            is_gacha_item_new: false,
            token_item_list: ::std::vec::Vec::new(),
            transfer_items: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GachaItem {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GachaItem").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GachaItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GachaItem {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fGachaItem.proto\x1a\x0fItemParam.proto\x1a\x17GachaTransferItem.pr\
    oto\"\xf4\x01\n\tGachaItem\x12)\n\ngacha_item\x18\x01\x20\x01(\x0b2\n.It\
    emParamR\tgachaItem\x12\"\n\ris_flash_card\x18\x04\x20\x01(\x08R\x0bisFl\
    ashCard\x12)\n\x11is_gacha_item_new\x18\x08\x20\x01(\x08R\x0eisGachaItem\
    New\x122\n\x0ftoken_item_list\x18\x0e\x20\x03(\x0b2\n.ItemParamR\rtokenI\
    temList\x129\n\x0etransfer_items\x18\x0f\x20\x03(\x0b2\x12.GachaTransfer\
    ItemR\rtransferItemsB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::GachaTransferItem::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GachaItem::generated_message_descriptor_data());
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
