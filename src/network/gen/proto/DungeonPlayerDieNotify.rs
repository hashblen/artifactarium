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

//! Generated file from `DungeonPlayerDieNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:DungeonPlayerDieNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DungeonPlayerDieNotify {
    // message fields
    // @@protoc_insertion_point(field:DungeonPlayerDieNotify.die_type)
    pub die_type: ::protobuf::EnumOrUnknown<super::PlayerDieType::PlayerDieType>,
    // @@protoc_insertion_point(field:DungeonPlayerDieNotify.revive_count)
    pub revive_count: u32,
    // @@protoc_insertion_point(field:DungeonPlayerDieNotify.wait_time)
    pub wait_time: u32,
    // @@protoc_insertion_point(field:DungeonPlayerDieNotify.dungeon_id)
    pub dungeon_id: u32,
    // @@protoc_insertion_point(field:DungeonPlayerDieNotify.murderer_entity_id)
    pub murderer_entity_id: u32,
    // @@protoc_insertion_point(field:DungeonPlayerDieNotify.strengthen_point_data_map)
    pub strengthen_point_data_map: ::std::collections::HashMap<u32, super::StrengthenPointData::StrengthenPointData>,
    // @@protoc_insertion_point(field:DungeonPlayerDieNotify.GKHNLKAADNG)
    pub GKHNLKAADNG: u32,
    // message oneof groups
    pub entity: ::std::option::Option<dungeon_player_die_notify::Entity>,
    // special fields
    // @@protoc_insertion_point(special_field:DungeonPlayerDieNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DungeonPlayerDieNotify {
    fn default() -> &'a DungeonPlayerDieNotify {
        <DungeonPlayerDieNotify as ::protobuf::Message>::default_instance()
    }
}

impl DungeonPlayerDieNotify {
    pub fn new() -> DungeonPlayerDieNotify {
        ::std::default::Default::default()
    }

    // uint32 monster_id = 2;

    pub fn monster_id(&self) -> u32 {
        match self.entity {
            ::std::option::Option::Some(dungeon_player_die_notify::Entity::MonsterId(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_monster_id(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_monster_id(&self) -> bool {
        match self.entity {
            ::std::option::Option::Some(dungeon_player_die_notify::Entity::MonsterId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_monster_id(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(dungeon_player_die_notify::Entity::MonsterId(v))
    }

    // uint32 gadget_id = 8;

    pub fn gadget_id(&self) -> u32 {
        match self.entity {
            ::std::option::Option::Some(dungeon_player_die_notify::Entity::GadgetId(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_gadget_id(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_gadget_id(&self) -> bool {
        match self.entity {
            ::std::option::Option::Some(dungeon_player_die_notify::Entity::GadgetId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_gadget_id(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(dungeon_player_die_notify::Entity::GadgetId(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "die_type",
            |m: &DungeonPlayerDieNotify| { &m.die_type },
            |m: &mut DungeonPlayerDieNotify| { &mut m.die_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "revive_count",
            |m: &DungeonPlayerDieNotify| { &m.revive_count },
            |m: &mut DungeonPlayerDieNotify| { &mut m.revive_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wait_time",
            |m: &DungeonPlayerDieNotify| { &m.wait_time },
            |m: &mut DungeonPlayerDieNotify| { &mut m.wait_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dungeon_id",
            |m: &DungeonPlayerDieNotify| { &m.dungeon_id },
            |m: &mut DungeonPlayerDieNotify| { &mut m.dungeon_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "murderer_entity_id",
            |m: &DungeonPlayerDieNotify| { &m.murderer_entity_id },
            |m: &mut DungeonPlayerDieNotify| { &mut m.murderer_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "strengthen_point_data_map",
            |m: &DungeonPlayerDieNotify| { &m.strengthen_point_data_map },
            |m: &mut DungeonPlayerDieNotify| { &mut m.strengthen_point_data_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GKHNLKAADNG",
            |m: &DungeonPlayerDieNotify| { &m.GKHNLKAADNG },
            |m: &mut DungeonPlayerDieNotify| { &mut m.GKHNLKAADNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "monster_id",
            DungeonPlayerDieNotify::has_monster_id,
            DungeonPlayerDieNotify::monster_id,
            DungeonPlayerDieNotify::set_monster_id,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "gadget_id",
            DungeonPlayerDieNotify::has_gadget_id,
            DungeonPlayerDieNotify::gadget_id,
            DungeonPlayerDieNotify::set_gadget_id,
        ));
        oneofs.push(dungeon_player_die_notify::Entity::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DungeonPlayerDieNotify>(
            "DungeonPlayerDieNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DungeonPlayerDieNotify {
    const NAME: &'static str = "DungeonPlayerDieNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.die_type = is.read_enum_or_unknown()?;
                },
                24 => {
                    self.revive_count = is.read_uint32()?;
                },
                56 => {
                    self.wait_time = is.read_uint32()?;
                },
                72 => {
                    self.dungeon_id = is.read_uint32()?;
                },
                88 => {
                    self.murderer_entity_id = is.read_uint32()?;
                },
                98 => {
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
                    self.strengthen_point_data_map.insert(key, value);
                },
                104 => {
                    self.GKHNLKAADNG = is.read_uint32()?;
                },
                16 => {
                    self.entity = ::std::option::Option::Some(dungeon_player_die_notify::Entity::MonsterId(is.read_uint32()?));
                },
                64 => {
                    self.entity = ::std::option::Option::Some(dungeon_player_die_notify::Entity::GadgetId(is.read_uint32()?));
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
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.die_type.value());
        }
        if self.revive_count != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.revive_count);
        }
        if self.wait_time != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.wait_time);
        }
        if self.dungeon_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.dungeon_id);
        }
        if self.murderer_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.murderer_entity_id);
        }
        for (k, v) in &self.strengthen_point_data_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.GKHNLKAADNG != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.GKHNLKAADNG);
        }
        if let ::std::option::Option::Some(ref v) = self.entity {
            match v {
                &dungeon_player_die_notify::Entity::MonsterId(v) => {
                    my_size += ::protobuf::rt::uint32_size(2, v);
                },
                &dungeon_player_die_notify::Entity::GadgetId(v) => {
                    my_size += ::protobuf::rt::uint32_size(8, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.die_type))?;
        }
        if self.revive_count != 0 {
            os.write_uint32(3, self.revive_count)?;
        }
        if self.wait_time != 0 {
            os.write_uint32(7, self.wait_time)?;
        }
        if self.dungeon_id != 0 {
            os.write_uint32(9, self.dungeon_id)?;
        }
        if self.murderer_entity_id != 0 {
            os.write_uint32(11, self.murderer_entity_id)?;
        }
        for (k, v) in &self.strengthen_point_data_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(98)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.GKHNLKAADNG != 0 {
            os.write_uint32(13, self.GKHNLKAADNG)?;
        }
        if let ::std::option::Option::Some(ref v) = self.entity {
            match v {
                &dungeon_player_die_notify::Entity::MonsterId(v) => {
                    os.write_uint32(2, v)?;
                },
                &dungeon_player_die_notify::Entity::GadgetId(v) => {
                    os.write_uint32(8, v)?;
                },
            };
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

    fn new() -> DungeonPlayerDieNotify {
        DungeonPlayerDieNotify::new()
    }

    fn clear(&mut self) {
        self.die_type = ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE);
        self.revive_count = 0;
        self.wait_time = 0;
        self.dungeon_id = 0;
        self.murderer_entity_id = 0;
        self.strengthen_point_data_map.clear();
        self.GKHNLKAADNG = 0;
        self.entity = ::std::option::Option::None;
        self.entity = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DungeonPlayerDieNotify {
        static instance: ::protobuf::rt::Lazy<DungeonPlayerDieNotify> = ::protobuf::rt::Lazy::new();
        instance.get(DungeonPlayerDieNotify::new)
    }
}

impl ::protobuf::MessageFull for DungeonPlayerDieNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DungeonPlayerDieNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DungeonPlayerDieNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DungeonPlayerDieNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `DungeonPlayerDieNotify`
pub mod dungeon_player_die_notify {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:DungeonPlayerDieNotify.entity)
    pub enum Entity {
        // @@protoc_insertion_point(oneof_field:DungeonPlayerDieNotify.monster_id)
        MonsterId(u32),
        // @@protoc_insertion_point(oneof_field:DungeonPlayerDieNotify.gadget_id)
        GadgetId(u32),
    }

    impl ::protobuf::Oneof for Entity {
    }

    impl ::protobuf::OneofFull for Entity {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::DungeonPlayerDieNotify as ::protobuf::MessageFull>::descriptor().oneof_by_name("entity").unwrap()).clone()
        }
    }

    impl Entity {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Entity>("entity")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cDungeonPlayerDieNotify.proto\x1a\x13PlayerDieType.proto\x1a\x19Str\
    engthenPointData.proto\"\x8d\x04\n\x16DungeonPlayerDieNotify\x12)\n\x08d\
    ie_type\x18\x01\x20\x01(\x0e2\x0e.PlayerDieTypeR\x07dieType\x12!\n\x0cre\
    vive_count\x18\x03\x20\x01(\rR\x0breviveCount\x12\x1b\n\twait_time\x18\
    \x07\x20\x01(\rR\x08waitTime\x12\x1d\n\ndungeon_id\x18\t\x20\x01(\rR\tdu\
    ngeonId\x12,\n\x12murderer_entity_id\x18\x0b\x20\x01(\rR\x10murdererEnti\
    tyId\x12n\n\x19strengthen_point_data_map\x18\x0c\x20\x03(\x0b23.DungeonP\
    layerDieNotify.StrengthenPointDataMapEntryR\x16strengthenPointDataMap\
    \x12\x20\n\x0bGKHNLKAADNG\x18\r\x20\x01(\rR\x0bGKHNLKAADNG\x12\x1f\n\nmo\
    nster_id\x18\x02\x20\x01(\rH\0R\tmonsterId\x12\x1d\n\tgadget_id\x18\x08\
    \x20\x01(\rH\0R\x08gadgetId\x1a_\n\x1bStrengthenPointDataMapEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12*\n\x05value\x18\x02\x20\x01\
    (\x0b2\x14.StrengthenPointDataR\x05value:\x028\x01B\x08\n\x06entityB\x1b\
    \n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::PlayerDieType::file_descriptor().clone());
            deps.push(super::StrengthenPointData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DungeonPlayerDieNotify::generated_message_descriptor_data());
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
