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

//! Generated file from `AvatarSkillDepotChangeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:AvatarSkillDepotChangeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarSkillDepotChangeNotify {
    // message fields
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.core_proud_skill_level)
    pub core_proud_skill_level: u32,
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.proud_skill_extra_level_map)
    pub proud_skill_extra_level_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.avatar_guid)
    pub avatar_guid: u64,
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.skill_level_map)
    pub skill_level_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.proud_skill_list)
    pub proud_skill_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.talent_id_list)
    pub talent_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AvatarSkillDepotChangeNotify.skill_depot_id)
    pub skill_depot_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarSkillDepotChangeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarSkillDepotChangeNotify {
    fn default() -> &'a AvatarSkillDepotChangeNotify {
        <AvatarSkillDepotChangeNotify as ::protobuf::Message>::default_instance()
    }
}

impl AvatarSkillDepotChangeNotify {
    pub fn new() -> AvatarSkillDepotChangeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &AvatarSkillDepotChangeNotify| { &m.entity_id },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "core_proud_skill_level",
            |m: &AvatarSkillDepotChangeNotify| { &m.core_proud_skill_level },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.core_proud_skill_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "proud_skill_extra_level_map",
            |m: &AvatarSkillDepotChangeNotify| { &m.proud_skill_extra_level_map },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.proud_skill_extra_level_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &AvatarSkillDepotChangeNotify| { &m.avatar_guid },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.avatar_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "skill_level_map",
            |m: &AvatarSkillDepotChangeNotify| { &m.skill_level_map },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.skill_level_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "proud_skill_list",
            |m: &AvatarSkillDepotChangeNotify| { &m.proud_skill_list },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.proud_skill_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "talent_id_list",
            |m: &AvatarSkillDepotChangeNotify| { &m.talent_id_list },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.talent_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_depot_id",
            |m: &AvatarSkillDepotChangeNotify| { &m.skill_depot_id },
            |m: &mut AvatarSkillDepotChangeNotify| { &mut m.skill_depot_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarSkillDepotChangeNotify>(
            "AvatarSkillDepotChangeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarSkillDepotChangeNotify {
    const NAME: &'static str = "AvatarSkillDepotChangeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.entity_id = is.read_uint32()?;
                },
                48 => {
                    self.core_proud_skill_level = is.read_uint32()?;
                },
                66 => {
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
                    self.proud_skill_extra_level_map.insert(key, value);
                },
                104 => {
                    self.avatar_guid = is.read_uint64()?;
                },
                10 => {
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
                    self.skill_level_map.insert(key, value);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.proud_skill_list)?;
                },
                96 => {
                    self.proud_skill_list.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.talent_id_list)?;
                },
                72 => {
                    self.talent_id_list.push(is.read_uint32()?);
                },
                24 => {
                    self.skill_depot_id = is.read_uint32()?;
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
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.entity_id);
        }
        if self.core_proud_skill_level != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.core_proud_skill_level);
        }
        for (k, v) in &self.proud_skill_extra_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(13, self.avatar_guid);
        }
        for (k, v) in &self.skill_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.proud_skill_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.talent_id_list);
        if self.skill_depot_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.skill_depot_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.entity_id != 0 {
            os.write_uint32(11, self.entity_id)?;
        }
        if self.core_proud_skill_level != 0 {
            os.write_uint32(6, self.core_proud_skill_level)?;
        }
        for (k, v) in &self.proud_skill_extra_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(66)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.avatar_guid != 0 {
            os.write_uint64(13, self.avatar_guid)?;
        }
        for (k, v) in &self.skill_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(10)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        os.write_repeated_packed_uint32(12, &self.proud_skill_list)?;
        os.write_repeated_packed_uint32(9, &self.talent_id_list)?;
        if self.skill_depot_id != 0 {
            os.write_uint32(3, self.skill_depot_id)?;
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

    fn new() -> AvatarSkillDepotChangeNotify {
        AvatarSkillDepotChangeNotify::new()
    }

    fn clear(&mut self) {
        self.entity_id = 0;
        self.core_proud_skill_level = 0;
        self.proud_skill_extra_level_map.clear();
        self.avatar_guid = 0;
        self.skill_level_map.clear();
        self.proud_skill_list.clear();
        self.talent_id_list.clear();
        self.skill_depot_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarSkillDepotChangeNotify {
        static instance: ::protobuf::rt::Lazy<AvatarSkillDepotChangeNotify> = ::protobuf::rt::Lazy::new();
        instance.get(AvatarSkillDepotChangeNotify::new)
    }
}

impl ::protobuf::MessageFull for AvatarSkillDepotChangeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarSkillDepotChangeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarSkillDepotChangeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarSkillDepotChangeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"AvatarSkillDepotChangeNotify.proto\"\xe9\x04\n\x1cAvatarSkillDepotCh\
    angeNotify\x12\x1b\n\tentity_id\x18\x0b\x20\x01(\rR\x08entityId\x123\n\
    \x16core_proud_skill_level\x18\x06\x20\x01(\rR\x13coreProudSkillLevel\
    \x12x\n\x1bproud_skill_extra_level_map\x18\x08\x20\x03(\x0b2:.AvatarSkil\
    lDepotChangeNotify.ProudSkillExtraLevelMapEntryR\x17proudSkillExtraLevel\
    Map\x12\x1f\n\x0bavatar_guid\x18\r\x20\x01(\x04R\navatarGuid\x12X\n\x0fs\
    kill_level_map\x18\x01\x20\x03(\x0b20.AvatarSkillDepotChangeNotify.Skill\
    LevelMapEntryR\rskillLevelMap\x12(\n\x10proud_skill_list\x18\x0c\x20\x03\
    (\rR\x0eproudSkillList\x12$\n\x0etalent_id_list\x18\t\x20\x03(\rR\x0ctal\
    entIdList\x12$\n\x0eskill_depot_id\x18\x03\x20\x01(\rR\x0cskillDepotId\
    \x1aJ\n\x1cProudSkillExtraLevelMapEntry\x12\x10\n\x03key\x18\x01\x20\x01\
    (\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01\
    \x1a@\n\x12SkillLevelMapEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03ke\
    y\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01B\x1b\n\x19em\
    u.grasscutter.net.protob\x06proto3\
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
            messages.push(AvatarSkillDepotChangeNotify::generated_message_descriptor_data());
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
