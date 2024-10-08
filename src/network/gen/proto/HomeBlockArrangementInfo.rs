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

//! Generated file from `HomeBlockArrangementInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:HomeBlockArrangementInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeBlockArrangementInfo {
    // message fields
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.comfort_value)
    pub comfort_value: u32,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.deploy_furniure_list)
    pub deploy_furniure_list: ::std::vec::Vec<super::HomeFurnitureData::HomeFurnitureData>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.dot_pattern_list)
    pub dot_pattern_list: ::std::vec::Vec<super::HomeBlockDotPattern::HomeBlockDotPattern>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.persistent_furniture_list)
    pub persistent_furniture_list: ::std::vec::Vec<super::HomeFurnitureData::HomeFurnitureData>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.furniture_group_list)
    pub furniture_group_list: ::std::vec::Vec<super::HomeFurnitureGroupData::HomeFurnitureGroupData>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.deploy_npc_list)
    pub deploy_npc_list: ::std::vec::Vec<super::HomeNpcData::HomeNpcData>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.deploy_animal_list)
    pub deploy_animal_list: ::std::vec::Vec<super::HomeAnimalData::HomeAnimalData>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.furniture_custom_suite_list)
    pub furniture_custom_suite_list: ::std::vec::Vec<super::HomeFurnitureCustomSuiteData::HomeFurnitureCustomSuiteData>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.furniture_suite_list)
    pub furniture_suite_list: ::std::vec::Vec<super::HomeFurnitureSuiteData::HomeFurnitureSuiteData>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.block_id)
    pub block_id: u32,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.is_unlocked)
    pub is_unlocked: bool,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.weekend_djinn_info_list)
    pub weekend_djinn_info_list: ::std::vec::Vec<super::WeekendDjinnInfo::WeekendDjinnInfo>,
    // @@protoc_insertion_point(field:HomeBlockArrangementInfo.field_list)
    pub field_list: ::std::vec::Vec<super::HomeBlockFieldData::HomeBlockFieldData>,
    // special fields
    // @@protoc_insertion_point(special_field:HomeBlockArrangementInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeBlockArrangementInfo {
    fn default() -> &'a HomeBlockArrangementInfo {
        <HomeBlockArrangementInfo as ::protobuf::Message>::default_instance()
    }
}

impl HomeBlockArrangementInfo {
    pub fn new() -> HomeBlockArrangementInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "comfort_value",
            |m: &HomeBlockArrangementInfo| { &m.comfort_value },
            |m: &mut HomeBlockArrangementInfo| { &mut m.comfort_value },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "deploy_furniure_list",
            |m: &HomeBlockArrangementInfo| { &m.deploy_furniure_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.deploy_furniure_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "dot_pattern_list",
            |m: &HomeBlockArrangementInfo| { &m.dot_pattern_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.dot_pattern_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "persistent_furniture_list",
            |m: &HomeBlockArrangementInfo| { &m.persistent_furniture_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.persistent_furniture_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "furniture_group_list",
            |m: &HomeBlockArrangementInfo| { &m.furniture_group_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.furniture_group_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "deploy_npc_list",
            |m: &HomeBlockArrangementInfo| { &m.deploy_npc_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.deploy_npc_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "deploy_animal_list",
            |m: &HomeBlockArrangementInfo| { &m.deploy_animal_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.deploy_animal_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "furniture_custom_suite_list",
            |m: &HomeBlockArrangementInfo| { &m.furniture_custom_suite_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.furniture_custom_suite_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "furniture_suite_list",
            |m: &HomeBlockArrangementInfo| { &m.furniture_suite_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.furniture_suite_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "block_id",
            |m: &HomeBlockArrangementInfo| { &m.block_id },
            |m: &mut HomeBlockArrangementInfo| { &mut m.block_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_unlocked",
            |m: &HomeBlockArrangementInfo| { &m.is_unlocked },
            |m: &mut HomeBlockArrangementInfo| { &mut m.is_unlocked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "weekend_djinn_info_list",
            |m: &HomeBlockArrangementInfo| { &m.weekend_djinn_info_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.weekend_djinn_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "field_list",
            |m: &HomeBlockArrangementInfo| { &m.field_list },
            |m: &mut HomeBlockArrangementInfo| { &mut m.field_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeBlockArrangementInfo>(
            "HomeBlockArrangementInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeBlockArrangementInfo {
    const NAME: &'static str = "HomeBlockArrangementInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.comfort_value = is.read_uint32()?;
                },
                18 => {
                    self.deploy_furniure_list.push(is.read_message()?);
                },
                26 => {
                    self.dot_pattern_list.push(is.read_message()?);
                },
                34 => {
                    self.persistent_furniture_list.push(is.read_message()?);
                },
                50 => {
                    self.furniture_group_list.push(is.read_message()?);
                },
                66 => {
                    self.deploy_npc_list.push(is.read_message()?);
                },
                74 => {
                    self.deploy_animal_list.push(is.read_message()?);
                },
                82 => {
                    self.furniture_custom_suite_list.push(is.read_message()?);
                },
                90 => {
                    self.furniture_suite_list.push(is.read_message()?);
                },
                96 => {
                    self.block_id = is.read_uint32()?;
                },
                104 => {
                    self.is_unlocked = is.read_bool()?;
                },
                114 => {
                    self.weekend_djinn_info_list.push(is.read_message()?);
                },
                122 => {
                    self.field_list.push(is.read_message()?);
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
        if self.comfort_value != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.comfort_value);
        }
        for value in &self.deploy_furniure_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.dot_pattern_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.persistent_furniture_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.furniture_group_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.deploy_npc_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.deploy_animal_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.furniture_custom_suite_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.furniture_suite_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.block_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.block_id);
        }
        if self.is_unlocked != false {
            my_size += 1 + 1;
        }
        for value in &self.weekend_djinn_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.field_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.comfort_value != 0 {
            os.write_uint32(1, self.comfort_value)?;
        }
        for v in &self.deploy_furniure_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.dot_pattern_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.persistent_furniture_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.furniture_group_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for v in &self.deploy_npc_list {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.deploy_animal_list {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.furniture_custom_suite_list {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        for v in &self.furniture_suite_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.block_id != 0 {
            os.write_uint32(12, self.block_id)?;
        }
        if self.is_unlocked != false {
            os.write_bool(13, self.is_unlocked)?;
        }
        for v in &self.weekend_djinn_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        for v in &self.field_list {
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

    fn new() -> HomeBlockArrangementInfo {
        HomeBlockArrangementInfo::new()
    }

    fn clear(&mut self) {
        self.comfort_value = 0;
        self.deploy_furniure_list.clear();
        self.dot_pattern_list.clear();
        self.persistent_furniture_list.clear();
        self.furniture_group_list.clear();
        self.deploy_npc_list.clear();
        self.deploy_animal_list.clear();
        self.furniture_custom_suite_list.clear();
        self.furniture_suite_list.clear();
        self.block_id = 0;
        self.is_unlocked = false;
        self.weekend_djinn_info_list.clear();
        self.field_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeBlockArrangementInfo {
        static instance: HomeBlockArrangementInfo = HomeBlockArrangementInfo {
            comfort_value: 0,
            deploy_furniure_list: ::std::vec::Vec::new(),
            dot_pattern_list: ::std::vec::Vec::new(),
            persistent_furniture_list: ::std::vec::Vec::new(),
            furniture_group_list: ::std::vec::Vec::new(),
            deploy_npc_list: ::std::vec::Vec::new(),
            deploy_animal_list: ::std::vec::Vec::new(),
            furniture_custom_suite_list: ::std::vec::Vec::new(),
            furniture_suite_list: ::std::vec::Vec::new(),
            block_id: 0,
            is_unlocked: false,
            weekend_djinn_info_list: ::std::vec::Vec::new(),
            field_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeBlockArrangementInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeBlockArrangementInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeBlockArrangementInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeBlockArrangementInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eHomeBlockArrangementInfo.proto\x1a\x17HomeFurnitureData.proto\x1a\
    \x19HomeBlockDotPattern.proto\x1a\x1cHomeFurnitureGroupData.proto\x1a\
    \x11HomeNpcData.proto\x1a\x14HomeAnimalData.proto\x1a\"HomeFurnitureCust\
    omSuiteData.proto\x1a\x1cHomeFurnitureSuiteData.proto\x1a\x16WeekendDjin\
    nInfo.proto\x1a\x18HomeBlockFieldData.proto\"\xb8\x06\n\x18HomeBlockArra\
    ngementInfo\x12#\n\rcomfort_value\x18\x01\x20\x01(\rR\x0ccomfortValue\
    \x12D\n\x14deploy_furniure_list\x18\x02\x20\x03(\x0b2\x12.HomeFurnitureD\
    ataR\x12deployFurniureList\x12>\n\x10dot_pattern_list\x18\x03\x20\x03(\
    \x0b2\x14.HomeBlockDotPatternR\x0edotPatternList\x12N\n\x19persistent_fu\
    rniture_list\x18\x04\x20\x03(\x0b2\x12.HomeFurnitureDataR\x17persistentF\
    urnitureList\x12I\n\x14furniture_group_list\x18\x06\x20\x03(\x0b2\x17.Ho\
    meFurnitureGroupDataR\x12furnitureGroupList\x124\n\x0fdeploy_npc_list\
    \x18\x08\x20\x03(\x0b2\x0c.HomeNpcDataR\rdeployNpcList\x12=\n\x12deploy_\
    animal_list\x18\t\x20\x03(\x0b2\x0f.HomeAnimalDataR\x10deployAnimalList\
    \x12\\\n\x1bfurniture_custom_suite_list\x18\n\x20\x03(\x0b2\x1d.HomeFurn\
    itureCustomSuiteDataR\x18furnitureCustomSuiteList\x12I\n\x14furniture_su\
    ite_list\x18\x0b\x20\x03(\x0b2\x17.HomeFurnitureSuiteDataR\x12furnitureS\
    uiteList\x12\x19\n\x08block_id\x18\x0c\x20\x01(\rR\x07blockId\x12\x1f\n\
    \x0bis_unlocked\x18\r\x20\x01(\x08R\nisUnlocked\x12H\n\x17weekend_djinn_\
    info_list\x18\x0e\x20\x03(\x0b2\x11.WeekendDjinnInfoR\x14weekendDjinnInf\
    oList\x122\n\nfield_list\x18\x0f\x20\x03(\x0b2\x13.HomeBlockFieldDataR\t\
    fieldListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(9);
            deps.push(super::HomeFurnitureData::file_descriptor().clone());
            deps.push(super::HomeBlockDotPattern::file_descriptor().clone());
            deps.push(super::HomeFurnitureGroupData::file_descriptor().clone());
            deps.push(super::HomeNpcData::file_descriptor().clone());
            deps.push(super::HomeAnimalData::file_descriptor().clone());
            deps.push(super::HomeFurnitureCustomSuiteData::file_descriptor().clone());
            deps.push(super::HomeFurnitureSuiteData::file_descriptor().clone());
            deps.push(super::WeekendDjinnInfo::file_descriptor().clone());
            deps.push(super::HomeBlockFieldData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HomeBlockArrangementInfo::generated_message_descriptor_data());
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
