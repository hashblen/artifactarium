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

//! Generated file from `WeaponAwakenRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:WeaponAwakenRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct WeaponAwakenRsp {
    // message fields
    // @@protoc_insertion_point(field:WeaponAwakenRsp.old_affix_level_map)
    pub old_affix_level_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:WeaponAwakenRsp.cur_affix_level_map)
    pub cur_affix_level_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:WeaponAwakenRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:WeaponAwakenRsp.target_weapon_awaken_level)
    pub target_weapon_awaken_level: u32,
    // @@protoc_insertion_point(field:WeaponAwakenRsp.target_weapon_guid)
    pub target_weapon_guid: u64,
    // @@protoc_insertion_point(field:WeaponAwakenRsp.avatar_guid)
    pub avatar_guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:WeaponAwakenRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WeaponAwakenRsp {
    fn default() -> &'a WeaponAwakenRsp {
        <WeaponAwakenRsp as ::protobuf::Message>::default_instance()
    }
}

impl WeaponAwakenRsp {
    pub fn new() -> WeaponAwakenRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "old_affix_level_map",
            |m: &WeaponAwakenRsp| { &m.old_affix_level_map },
            |m: &mut WeaponAwakenRsp| { &mut m.old_affix_level_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "cur_affix_level_map",
            |m: &WeaponAwakenRsp| { &m.cur_affix_level_map },
            |m: &mut WeaponAwakenRsp| { &mut m.cur_affix_level_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &WeaponAwakenRsp| { &m.retcode },
            |m: &mut WeaponAwakenRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_weapon_awaken_level",
            |m: &WeaponAwakenRsp| { &m.target_weapon_awaken_level },
            |m: &mut WeaponAwakenRsp| { &mut m.target_weapon_awaken_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_weapon_guid",
            |m: &WeaponAwakenRsp| { &m.target_weapon_guid },
            |m: &mut WeaponAwakenRsp| { &mut m.target_weapon_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &WeaponAwakenRsp| { &m.avatar_guid },
            |m: &mut WeaponAwakenRsp| { &mut m.avatar_guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WeaponAwakenRsp>(
            "WeaponAwakenRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WeaponAwakenRsp {
    const NAME: &'static str = "WeaponAwakenRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.old_affix_level_map.insert(key, value);
                },
                106 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.cur_affix_level_map.insert(key, value);
                },
                48 => {
                    self.retcode = is.read_int32()?;
                },
                8 => {
                    self.target_weapon_awaken_level = is.read_uint32()?;
                },
                120 => {
                    self.target_weapon_guid = is.read_uint64()?;
                },
                56 => {
                    self.avatar_guid = is.read_uint64()?;
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
        for (k, v) in &self.old_affix_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for (k, v) in &self.cur_affix_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.retcode);
        }
        if self.target_weapon_awaken_level != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.target_weapon_awaken_level);
        }
        if self.target_weapon_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(15, self.target_weapon_guid);
        }
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(7, self.avatar_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.old_affix_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for (k, v) in &self.cur_affix_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(106)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.retcode != 0 {
            os.write_int32(6, self.retcode)?;
        }
        if self.target_weapon_awaken_level != 0 {
            os.write_uint32(1, self.target_weapon_awaken_level)?;
        }
        if self.target_weapon_guid != 0 {
            os.write_uint64(15, self.target_weapon_guid)?;
        }
        if self.avatar_guid != 0 {
            os.write_uint64(7, self.avatar_guid)?;
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

    fn new() -> WeaponAwakenRsp {
        WeaponAwakenRsp::new()
    }

    fn clear(&mut self) {
        self.old_affix_level_map.clear();
        self.cur_affix_level_map.clear();
        self.retcode = 0;
        self.target_weapon_awaken_level = 0;
        self.target_weapon_guid = 0;
        self.avatar_guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WeaponAwakenRsp {
        static instance: ::protobuf::rt::Lazy<WeaponAwakenRsp> = ::protobuf::rt::Lazy::new();
        instance.get(WeaponAwakenRsp::new)
    }
}

impl ::protobuf::MessageFull for WeaponAwakenRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WeaponAwakenRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WeaponAwakenRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WeaponAwakenRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15WeaponAwakenRsp.proto\"\xef\x03\n\x0fWeaponAwakenRsp\x12U\n\x13old\
    _affix_level_map\x18\x02\x20\x03(\x0b2&.WeaponAwakenRsp.OldAffixLevelMap\
    EntryR\x10oldAffixLevelMap\x12U\n\x13cur_affix_level_map\x18\r\x20\x03(\
    \x0b2&.WeaponAwakenRsp.CurAffixLevelMapEntryR\x10curAffixLevelMap\x12\
    \x18\n\x07retcode\x18\x06\x20\x01(\x05R\x07retcode\x12;\n\x1atarget_weap\
    on_awaken_level\x18\x01\x20\x01(\rR\x17targetWeaponAwakenLevel\x12,\n\
    \x12target_weapon_guid\x18\x0f\x20\x01(\x04R\x10targetWeaponGuid\x12\x1f\
    \n\x0bavatar_guid\x18\x07\x20\x01(\x04R\navatarGuid\x1aC\n\x15OldAffixLe\
    velMapEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05val\
    ue\x18\x02\x20\x01(\rR\x05value:\x028\x01\x1aC\n\x15CurAffixLevelMapEntr\
    y\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\
    \x20\x01(\rR\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\x06\
    proto3\
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
            messages.push(WeaponAwakenRsp::generated_message_descriptor_data());
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
