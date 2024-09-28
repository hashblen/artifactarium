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

//! Generated file from `StoreItemChangeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:StoreItemChangeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StoreItemChangeNotify {
    // message fields
    // @@protoc_insertion_point(field:StoreItemChangeNotify.item_list)
    pub item_list: ::std::vec::Vec<super::Item::Item>,
    // @@protoc_insertion_point(field:StoreItemChangeNotify.store_type)
    pub store_type: ::protobuf::EnumOrUnknown<super::StoreType::StoreType>,
    // special fields
    // @@protoc_insertion_point(special_field:StoreItemChangeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StoreItemChangeNotify {
    fn default() -> &'a StoreItemChangeNotify {
        <StoreItemChangeNotify as ::protobuf::Message>::default_instance()
    }
}

impl StoreItemChangeNotify {
    pub fn new() -> StoreItemChangeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item_list",
            |m: &StoreItemChangeNotify| { &m.item_list },
            |m: &mut StoreItemChangeNotify| { &mut m.item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "store_type",
            |m: &StoreItemChangeNotify| { &m.store_type },
            |m: &mut StoreItemChangeNotify| { &mut m.store_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StoreItemChangeNotify>(
            "StoreItemChangeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StoreItemChangeNotify {
    const NAME: &'static str = "StoreItemChangeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.item_list.push(is.read_message()?);
                },
                96 => {
                    self.store_type = is.read_enum_or_unknown()?;
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
        for value in &self.item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.store_type != ::protobuf::EnumOrUnknown::new(super::StoreType::StoreType::STORE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.store_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.item_list {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.store_type != ::protobuf::EnumOrUnknown::new(super::StoreType::StoreType::STORE_TYPE_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.store_type))?;
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

    fn new() -> StoreItemChangeNotify {
        StoreItemChangeNotify::new()
    }

    fn clear(&mut self) {
        self.item_list.clear();
        self.store_type = ::protobuf::EnumOrUnknown::new(super::StoreType::StoreType::STORE_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StoreItemChangeNotify {
        static instance: StoreItemChangeNotify = StoreItemChangeNotify {
            item_list: ::std::vec::Vec::new(),
            store_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StoreItemChangeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StoreItemChangeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StoreItemChangeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StoreItemChangeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bStoreItemChangeNotify.proto\x1a\nItem.proto\x1a\x0fStoreType.proto\
    \"f\n\x15StoreItemChangeNotify\x12\"\n\titem_list\x18\n\x20\x03(\x0b2\
    \x05.ItemR\x08itemList\x12)\n\nstore_type\x18\x0c\x20\x01(\x0e2\n.StoreT\
    ypeR\tstoreTypeB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::Item::file_descriptor().clone());
            deps.push(super::StoreType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StoreItemChangeNotify::generated_message_descriptor_data());
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
