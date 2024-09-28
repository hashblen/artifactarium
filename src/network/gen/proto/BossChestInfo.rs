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

//! Generated file from `BossChestInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:BossChestInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BossChestInfo {
    // message fields
    // @@protoc_insertion_point(field:BossChestInfo.monster_config_id)
    pub monster_config_id: u32,
    // @@protoc_insertion_point(field:BossChestInfo.resin)
    pub resin: u32,
    // @@protoc_insertion_point(field:BossChestInfo.remain_uid_list)
    pub remain_uid_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BossChestInfo.qualify_uid_list)
    pub qualify_uid_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BossChestInfo.uid_discount_map)
    pub uid_discount_map: ::std::collections::HashMap<u32, super::WeeklyBossResinDiscountInfo::WeeklyBossResinDiscountInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:BossChestInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BossChestInfo {
    fn default() -> &'a BossChestInfo {
        <BossChestInfo as ::protobuf::Message>::default_instance()
    }
}

impl BossChestInfo {
    pub fn new() -> BossChestInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "monster_config_id",
            |m: &BossChestInfo| { &m.monster_config_id },
            |m: &mut BossChestInfo| { &mut m.monster_config_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "resin",
            |m: &BossChestInfo| { &m.resin },
            |m: &mut BossChestInfo| { &mut m.resin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "remain_uid_list",
            |m: &BossChestInfo| { &m.remain_uid_list },
            |m: &mut BossChestInfo| { &mut m.remain_uid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "qualify_uid_list",
            |m: &BossChestInfo| { &m.qualify_uid_list },
            |m: &mut BossChestInfo| { &mut m.qualify_uid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "uid_discount_map",
            |m: &BossChestInfo| { &m.uid_discount_map },
            |m: &mut BossChestInfo| { &mut m.uid_discount_map },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BossChestInfo>(
            "BossChestInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BossChestInfo {
    const NAME: &'static str = "BossChestInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.monster_config_id = is.read_uint32()?;
                },
                16 => {
                    self.resin = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.remain_uid_list)?;
                },
                24 => {
                    self.remain_uid_list.push(is.read_uint32()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.qualify_uid_list)?;
                },
                32 => {
                    self.qualify_uid_list.push(is.read_uint32()?);
                },
                42 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.uid_discount_map.insert(key, value);
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
        if self.monster_config_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.monster_config_id);
        }
        if self.resin != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.resin);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(3, &self.remain_uid_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.qualify_uid_list);
        for (k, v) in &self.uid_discount_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.monster_config_id != 0 {
            os.write_uint32(1, self.monster_config_id)?;
        }
        if self.resin != 0 {
            os.write_uint32(2, self.resin)?;
        }
        os.write_repeated_packed_uint32(3, &self.remain_uid_list)?;
        os.write_repeated_packed_uint32(4, &self.qualify_uid_list)?;
        for (k, v) in &self.uid_discount_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(42)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
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

    fn new() -> BossChestInfo {
        BossChestInfo::new()
    }

    fn clear(&mut self) {
        self.monster_config_id = 0;
        self.resin = 0;
        self.remain_uid_list.clear();
        self.qualify_uid_list.clear();
        self.uid_discount_map.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BossChestInfo {
        static instance: ::protobuf::rt::Lazy<BossChestInfo> = ::protobuf::rt::Lazy::new();
        instance.get(BossChestInfo::new)
    }
}

impl ::protobuf::MessageFull for BossChestInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BossChestInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BossChestInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BossChestInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13BossChestInfo.proto\x1a!WeeklyBossResinDiscountInfo.proto\"\xd2\
    \x02\n\rBossChestInfo\x12*\n\x11monster_config_id\x18\x01\x20\x01(\rR\
    \x0fmonsterConfigId\x12\x14\n\x05resin\x18\x02\x20\x01(\rR\x05resin\x12&\
    \n\x0fremain_uid_list\x18\x03\x20\x03(\rR\rremainUidList\x12(\n\x10quali\
    fy_uid_list\x18\x04\x20\x03(\rR\x0equalifyUidList\x12L\n\x10uid_discount\
    _map\x18\x05\x20\x03(\x0b2\".BossChestInfo.UidDiscountMapEntryR\x0euidDi\
    scountMap\x1a_\n\x13UidDiscountMapEntry\x12\x10\n\x03key\x18\x01\x20\x01\
    (\rR\x03key\x122\n\x05value\x18\x02\x20\x01(\x0b2\x1c.WeeklyBossResinDis\
    countInfoR\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\x06pr\
    oto3\
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
            deps.push(super::WeeklyBossResinDiscountInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BossChestInfo::generated_message_descriptor_data());
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
