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

//! Generated file from `AvatarExpeditionGetRewardRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:AvatarExpeditionGetRewardRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarExpeditionGetRewardRsp {
    // message fields
    // @@protoc_insertion_point(field:AvatarExpeditionGetRewardRsp.extra_item_list)
    pub extra_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:AvatarExpeditionGetRewardRsp.expedition_info_map)
    pub expedition_info_map: ::std::collections::HashMap<u64, super::AvatarExpeditionInfo::AvatarExpeditionInfo>,
    // @@protoc_insertion_point(field:AvatarExpeditionGetRewardRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:AvatarExpeditionGetRewardRsp.item_list)
    pub item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarExpeditionGetRewardRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarExpeditionGetRewardRsp {
    fn default() -> &'a AvatarExpeditionGetRewardRsp {
        <AvatarExpeditionGetRewardRsp as ::protobuf::Message>::default_instance()
    }
}

impl AvatarExpeditionGetRewardRsp {
    pub fn new() -> AvatarExpeditionGetRewardRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "extra_item_list",
            |m: &AvatarExpeditionGetRewardRsp| { &m.extra_item_list },
            |m: &mut AvatarExpeditionGetRewardRsp| { &mut m.extra_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "expedition_info_map",
            |m: &AvatarExpeditionGetRewardRsp| { &m.expedition_info_map },
            |m: &mut AvatarExpeditionGetRewardRsp| { &mut m.expedition_info_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &AvatarExpeditionGetRewardRsp| { &m.retcode },
            |m: &mut AvatarExpeditionGetRewardRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item_list",
            |m: &AvatarExpeditionGetRewardRsp| { &m.item_list },
            |m: &mut AvatarExpeditionGetRewardRsp| { &mut m.item_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarExpeditionGetRewardRsp>(
            "AvatarExpeditionGetRewardRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarExpeditionGetRewardRsp {
    const NAME: &'static str = "AvatarExpeditionGetRewardRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.extra_item_list.push(is.read_message()?);
                },
                74 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint64()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.expedition_info_map.insert(key, value);
                },
                96 => {
                    self.retcode = is.read_int32()?;
                },
                42 => {
                    self.item_list.push(is.read_message()?);
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
        for value in &self.extra_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for (k, v) in &self.expedition_info_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint64_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(12, self.retcode);
        }
        for value in &self.item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.extra_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        for (k, v) in &self.expedition_info_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint64_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(74)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint64(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.retcode != 0 {
            os.write_int32(12, self.retcode)?;
        }
        for v in &self.item_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> AvatarExpeditionGetRewardRsp {
        AvatarExpeditionGetRewardRsp::new()
    }

    fn clear(&mut self) {
        self.extra_item_list.clear();
        self.expedition_info_map.clear();
        self.retcode = 0;
        self.item_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarExpeditionGetRewardRsp {
        static instance: ::protobuf::rt::Lazy<AvatarExpeditionGetRewardRsp> = ::protobuf::rt::Lazy::new();
        instance.get(AvatarExpeditionGetRewardRsp::new)
    }
}

impl ::protobuf::MessageFull for AvatarExpeditionGetRewardRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarExpeditionGetRewardRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarExpeditionGetRewardRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarExpeditionGetRewardRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"AvatarExpeditionGetRewardRsp.proto\x1a\x0fItemParam.proto\x1a\x1aAva\
    tarExpeditionInfo.proto\"\xd8\x02\n\x1cAvatarExpeditionGetRewardRsp\x122\
    \n\x0fextra_item_list\x18\r\x20\x03(\x0b2\n.ItemParamR\rextraItemList\
    \x12d\n\x13expedition_info_map\x18\t\x20\x03(\x0b24.AvatarExpeditionGetR\
    ewardRsp.ExpeditionInfoMapEntryR\x11expeditionInfoMap\x12\x18\n\x07retco\
    de\x18\x0c\x20\x01(\x05R\x07retcode\x12'\n\titem_list\x18\x05\x20\x03(\
    \x0b2\n.ItemParamR\x08itemList\x1a[\n\x16ExpeditionInfoMapEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\x04R\x03key\x12+\n\x05value\x18\x02\x20\x01(\
    \x0b2\x15.AvatarExpeditionInfoR\x05value:\x028\x01B\x1b\n\x19emu.grasscu\
    tter.net.protob\x06proto3\
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
            deps.push(super::AvatarExpeditionInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AvatarExpeditionGetRewardRsp::generated_message_descriptor_data());
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
