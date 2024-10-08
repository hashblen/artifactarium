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

//! Generated file from `GachaTransferItem.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:GachaTransferItem)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GachaTransferItem {
    // message fields
    // @@protoc_insertion_point(field:GachaTransferItem.item)
    pub item: ::protobuf::MessageField<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:GachaTransferItem.is_transfer_item_new)
    pub is_transfer_item_new: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GachaTransferItem.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GachaTransferItem {
    fn default() -> &'a GachaTransferItem {
        <GachaTransferItem as ::protobuf::Message>::default_instance()
    }
}

impl GachaTransferItem {
    pub fn new() -> GachaTransferItem {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemParam::ItemParam>(
            "item",
            |m: &GachaTransferItem| { &m.item },
            |m: &mut GachaTransferItem| { &mut m.item },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_transfer_item_new",
            |m: &GachaTransferItem| { &m.is_transfer_item_new },
            |m: &mut GachaTransferItem| { &mut m.is_transfer_item_new },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GachaTransferItem>(
            "GachaTransferItem",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GachaTransferItem {
    const NAME: &'static str = "GachaTransferItem";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.item)?;
                },
                120 => {
                    self.is_transfer_item_new = is.read_bool()?;
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
        if let Some(v) = self.item.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.is_transfer_item_new != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.item.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.is_transfer_item_new != false {
            os.write_bool(15, self.is_transfer_item_new)?;
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

    fn new() -> GachaTransferItem {
        GachaTransferItem::new()
    }

    fn clear(&mut self) {
        self.item.clear();
        self.is_transfer_item_new = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GachaTransferItem {
        static instance: GachaTransferItem = GachaTransferItem {
            item: ::protobuf::MessageField::none(),
            is_transfer_item_new: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GachaTransferItem {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GachaTransferItem").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GachaTransferItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GachaTransferItem {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GachaTransferItem.proto\x1a\x0fItemParam.proto\"d\n\x11GachaTransf\
    erItem\x12\x1e\n\x04item\x18\x05\x20\x01(\x0b2\n.ItemParamR\x04item\x12/\
    \n\x14is_transfer_item_new\x18\x0f\x20\x01(\x08R\x11isTransferItemNewB\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::ItemParam::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GachaTransferItem::generated_message_descriptor_data());
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
