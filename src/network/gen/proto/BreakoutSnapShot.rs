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

//! Generated file from `BreakoutSnapShot.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:BreakoutSnapShot)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BreakoutSnapShot {
    // message fields
    // @@protoc_insertion_point(field:BreakoutSnapShot.client_game_time)
    pub client_game_time: u64,
    // @@protoc_insertion_point(field:BreakoutSnapShot.server_game_time)
    pub server_game_time: u64,
    // @@protoc_insertion_point(field:BreakoutSnapShot.ball_list)
    pub ball_list: ::std::vec::Vec<super::BreakoutPhysicalObject::BreakoutPhysicalObject>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.physical_object_list)
    pub physical_object_list: ::std::vec::Vec<super::BreakoutPhysicalObject::BreakoutPhysicalObject>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.action_list)
    pub action_list: ::std::vec::Vec<super::BreakoutAction::BreakoutAction>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.wave_index)
    pub wave_index: u32,
    // @@protoc_insertion_point(field:BreakoutSnapShot.is_finish)
    pub is_finish: bool,
    // @@protoc_insertion_point(field:BreakoutSnapShot.score)
    pub score: u32,
    // @@protoc_insertion_point(field:BreakoutSnapShot.combo)
    pub combo: u32,
    // @@protoc_insertion_point(field:BreakoutSnapShot.max_combo)
    pub max_combo: u32,
    // @@protoc_insertion_point(field:BreakoutSnapShot.life_count)
    pub life_count: u32,
    // @@protoc_insertion_point(field:BreakoutSnapShot.wave_suite_index)
    pub wave_suite_index: u32,
    // @@protoc_insertion_point(field:BreakoutSnapShot.spawn_point_list)
    pub spawn_point_list: ::std::vec::Vec<super::BreakoutSpawnPoint::BreakoutSpawnPoint>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.remaining_boss_hp)
    pub remaining_boss_hp: u32,
    // @@protoc_insertion_point(field:BreakoutSnapShot.brick_element_reaction_list)
    pub brick_element_reaction_list: ::std::vec::Vec<super::BreakoutElementReactionCounter::BreakoutElementReactionCounter>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.ball_element_reaction_list)
    pub ball_element_reaction_list: ::std::vec::Vec<super::BreakoutElementReactionCounter::BreakoutElementReactionCounter>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.uid_info_list)
    pub uid_info_list: ::std::vec::Vec<super::BreakoutSyncConnectUidInfo::BreakoutSyncConnectUidInfo>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.dynamic_object_list)
    pub dynamic_object_list: ::std::vec::Vec<super::BreakoutPhysicalObject::BreakoutPhysicalObject>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.id_index_list)
    pub id_index_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BreakoutSnapShot.raw_client_game_time)
    pub raw_client_game_time: i32,
    // special fields
    // @@protoc_insertion_point(special_field:BreakoutSnapShot.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BreakoutSnapShot {
    fn default() -> &'a BreakoutSnapShot {
        <BreakoutSnapShot as ::protobuf::Message>::default_instance()
    }
}

impl BreakoutSnapShot {
    pub fn new() -> BreakoutSnapShot {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(20);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_game_time",
            |m: &BreakoutSnapShot| { &m.client_game_time },
            |m: &mut BreakoutSnapShot| { &mut m.client_game_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "server_game_time",
            |m: &BreakoutSnapShot| { &m.server_game_time },
            |m: &mut BreakoutSnapShot| { &mut m.server_game_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ball_list",
            |m: &BreakoutSnapShot| { &m.ball_list },
            |m: &mut BreakoutSnapShot| { &mut m.ball_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "physical_object_list",
            |m: &BreakoutSnapShot| { &m.physical_object_list },
            |m: &mut BreakoutSnapShot| { &mut m.physical_object_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "action_list",
            |m: &BreakoutSnapShot| { &m.action_list },
            |m: &mut BreakoutSnapShot| { &mut m.action_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wave_index",
            |m: &BreakoutSnapShot| { &m.wave_index },
            |m: &mut BreakoutSnapShot| { &mut m.wave_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_finish",
            |m: &BreakoutSnapShot| { &m.is_finish },
            |m: &mut BreakoutSnapShot| { &mut m.is_finish },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &BreakoutSnapShot| { &m.score },
            |m: &mut BreakoutSnapShot| { &mut m.score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "combo",
            |m: &BreakoutSnapShot| { &m.combo },
            |m: &mut BreakoutSnapShot| { &mut m.combo },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_combo",
            |m: &BreakoutSnapShot| { &m.max_combo },
            |m: &mut BreakoutSnapShot| { &mut m.max_combo },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "life_count",
            |m: &BreakoutSnapShot| { &m.life_count },
            |m: &mut BreakoutSnapShot| { &mut m.life_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wave_suite_index",
            |m: &BreakoutSnapShot| { &m.wave_suite_index },
            |m: &mut BreakoutSnapShot| { &mut m.wave_suite_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "spawn_point_list",
            |m: &BreakoutSnapShot| { &m.spawn_point_list },
            |m: &mut BreakoutSnapShot| { &mut m.spawn_point_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "remaining_boss_hp",
            |m: &BreakoutSnapShot| { &m.remaining_boss_hp },
            |m: &mut BreakoutSnapShot| { &mut m.remaining_boss_hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "brick_element_reaction_list",
            |m: &BreakoutSnapShot| { &m.brick_element_reaction_list },
            |m: &mut BreakoutSnapShot| { &mut m.brick_element_reaction_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ball_element_reaction_list",
            |m: &BreakoutSnapShot| { &m.ball_element_reaction_list },
            |m: &mut BreakoutSnapShot| { &mut m.ball_element_reaction_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "uid_info_list",
            |m: &BreakoutSnapShot| { &m.uid_info_list },
            |m: &mut BreakoutSnapShot| { &mut m.uid_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "dynamic_object_list",
            |m: &BreakoutSnapShot| { &m.dynamic_object_list },
            |m: &mut BreakoutSnapShot| { &mut m.dynamic_object_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "id_index_list",
            |m: &BreakoutSnapShot| { &m.id_index_list },
            |m: &mut BreakoutSnapShot| { &mut m.id_index_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "raw_client_game_time",
            |m: &BreakoutSnapShot| { &m.raw_client_game_time },
            |m: &mut BreakoutSnapShot| { &mut m.raw_client_game_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BreakoutSnapShot>(
            "BreakoutSnapShot",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BreakoutSnapShot {
    const NAME: &'static str = "BreakoutSnapShot";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.client_game_time = is.read_uint64()?;
                },
                16 => {
                    self.server_game_time = is.read_uint64()?;
                },
                26 => {
                    self.ball_list.push(is.read_message()?);
                },
                34 => {
                    self.physical_object_list.push(is.read_message()?);
                },
                42 => {
                    self.action_list.push(is.read_message()?);
                },
                48 => {
                    self.wave_index = is.read_uint32()?;
                },
                56 => {
                    self.is_finish = is.read_bool()?;
                },
                64 => {
                    self.score = is.read_uint32()?;
                },
                72 => {
                    self.combo = is.read_uint32()?;
                },
                80 => {
                    self.max_combo = is.read_uint32()?;
                },
                88 => {
                    self.life_count = is.read_uint32()?;
                },
                96 => {
                    self.wave_suite_index = is.read_uint32()?;
                },
                106 => {
                    self.spawn_point_list.push(is.read_message()?);
                },
                112 => {
                    self.remaining_boss_hp = is.read_uint32()?;
                },
                122 => {
                    self.brick_element_reaction_list.push(is.read_message()?);
                },
                130 => {
                    self.ball_element_reaction_list.push(is.read_message()?);
                },
                138 => {
                    self.uid_info_list.push(is.read_message()?);
                },
                146 => {
                    self.dynamic_object_list.push(is.read_message()?);
                },
                154 => {
                    is.read_repeated_packed_uint32_into(&mut self.id_index_list)?;
                },
                152 => {
                    self.id_index_list.push(is.read_uint32()?);
                },
                160 => {
                    self.raw_client_game_time = is.read_int32()?;
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
        if self.client_game_time != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.client_game_time);
        }
        if self.server_game_time != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.server_game_time);
        }
        for value in &self.ball_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.physical_object_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.action_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.wave_index != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.wave_index);
        }
        if self.is_finish != false {
            my_size += 1 + 1;
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.score);
        }
        if self.combo != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.combo);
        }
        if self.max_combo != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.max_combo);
        }
        if self.life_count != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.life_count);
        }
        if self.wave_suite_index != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.wave_suite_index);
        }
        for value in &self.spawn_point_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.remaining_boss_hp != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.remaining_boss_hp);
        }
        for value in &self.brick_element_reaction_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.ball_element_reaction_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.uid_info_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.dynamic_object_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(19, &self.id_index_list);
        if self.raw_client_game_time != 0 {
            my_size += ::protobuf::rt::int32_size(20, self.raw_client_game_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.client_game_time != 0 {
            os.write_uint64(1, self.client_game_time)?;
        }
        if self.server_game_time != 0 {
            os.write_uint64(2, self.server_game_time)?;
        }
        for v in &self.ball_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.physical_object_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.action_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.wave_index != 0 {
            os.write_uint32(6, self.wave_index)?;
        }
        if self.is_finish != false {
            os.write_bool(7, self.is_finish)?;
        }
        if self.score != 0 {
            os.write_uint32(8, self.score)?;
        }
        if self.combo != 0 {
            os.write_uint32(9, self.combo)?;
        }
        if self.max_combo != 0 {
            os.write_uint32(10, self.max_combo)?;
        }
        if self.life_count != 0 {
            os.write_uint32(11, self.life_count)?;
        }
        if self.wave_suite_index != 0 {
            os.write_uint32(12, self.wave_suite_index)?;
        }
        for v in &self.spawn_point_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.remaining_boss_hp != 0 {
            os.write_uint32(14, self.remaining_boss_hp)?;
        }
        for v in &self.brick_element_reaction_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        for v in &self.ball_element_reaction_list {
            ::protobuf::rt::write_message_field_with_cached_size(16, v, os)?;
        };
        for v in &self.uid_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(17, v, os)?;
        };
        for v in &self.dynamic_object_list {
            ::protobuf::rt::write_message_field_with_cached_size(18, v, os)?;
        };
        os.write_repeated_packed_uint32(19, &self.id_index_list)?;
        if self.raw_client_game_time != 0 {
            os.write_int32(20, self.raw_client_game_time)?;
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

    fn new() -> BreakoutSnapShot {
        BreakoutSnapShot::new()
    }

    fn clear(&mut self) {
        self.client_game_time = 0;
        self.server_game_time = 0;
        self.ball_list.clear();
        self.physical_object_list.clear();
        self.action_list.clear();
        self.wave_index = 0;
        self.is_finish = false;
        self.score = 0;
        self.combo = 0;
        self.max_combo = 0;
        self.life_count = 0;
        self.wave_suite_index = 0;
        self.spawn_point_list.clear();
        self.remaining_boss_hp = 0;
        self.brick_element_reaction_list.clear();
        self.ball_element_reaction_list.clear();
        self.uid_info_list.clear();
        self.dynamic_object_list.clear();
        self.id_index_list.clear();
        self.raw_client_game_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BreakoutSnapShot {
        static instance: BreakoutSnapShot = BreakoutSnapShot {
            client_game_time: 0,
            server_game_time: 0,
            ball_list: ::std::vec::Vec::new(),
            physical_object_list: ::std::vec::Vec::new(),
            action_list: ::std::vec::Vec::new(),
            wave_index: 0,
            is_finish: false,
            score: 0,
            combo: 0,
            max_combo: 0,
            life_count: 0,
            wave_suite_index: 0,
            spawn_point_list: ::std::vec::Vec::new(),
            remaining_boss_hp: 0,
            brick_element_reaction_list: ::std::vec::Vec::new(),
            ball_element_reaction_list: ::std::vec::Vec::new(),
            uid_info_list: ::std::vec::Vec::new(),
            dynamic_object_list: ::std::vec::Vec::new(),
            id_index_list: ::std::vec::Vec::new(),
            raw_client_game_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BreakoutSnapShot {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BreakoutSnapShot").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BreakoutSnapShot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BreakoutSnapShot {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16BreakoutSnapShot.proto\x1a\x1cBreakoutPhysicalObject.proto\x1a\x14\
    BreakoutAction.proto\x1a\x18BreakoutSpawnPoint.proto\x1a$BreakoutElement\
    ReactionCounter.proto\x1a\x20BreakoutSyncConnectUidInfo.proto\"\xef\x07\
    \n\x10BreakoutSnapShot\x12(\n\x10client_game_time\x18\x01\x20\x01(\x04R\
    \x0eclientGameTime\x12(\n\x10server_game_time\x18\x02\x20\x01(\x04R\x0es\
    erverGameTime\x124\n\tball_list\x18\x03\x20\x03(\x0b2\x17.BreakoutPhysic\
    alObjectR\x08ballList\x12I\n\x14physical_object_list\x18\x04\x20\x03(\
    \x0b2\x17.BreakoutPhysicalObjectR\x12physicalObjectList\x120\n\x0baction\
    _list\x18\x05\x20\x03(\x0b2\x0f.BreakoutActionR\nactionList\x12\x1d\n\nw\
    ave_index\x18\x06\x20\x01(\rR\twaveIndex\x12\x1b\n\tis_finish\x18\x07\
    \x20\x01(\x08R\x08isFinish\x12\x14\n\x05score\x18\x08\x20\x01(\rR\x05sco\
    re\x12\x14\n\x05combo\x18\t\x20\x01(\rR\x05combo\x12\x1b\n\tmax_combo\
    \x18\n\x20\x01(\rR\x08maxCombo\x12\x1d\n\nlife_count\x18\x0b\x20\x01(\rR\
    \tlifeCount\x12(\n\x10wave_suite_index\x18\x0c\x20\x01(\rR\x0ewaveSuiteI\
    ndex\x12=\n\x10spawn_point_list\x18\r\x20\x03(\x0b2\x13.BreakoutSpawnPoi\
    ntR\x0espawnPointList\x12*\n\x11remaining_boss_hp\x18\x0e\x20\x01(\rR\
    \x0fremainingBossHp\x12^\n\x1bbrick_element_reaction_list\x18\x0f\x20\
    \x03(\x0b2\x1f.BreakoutElementReactionCounterR\x18brickElementReactionLi\
    st\x12\\\n\x1aball_element_reaction_list\x18\x10\x20\x03(\x0b2\x1f.Break\
    outElementReactionCounterR\x17ballElementReactionList\x12?\n\ruid_info_l\
    ist\x18\x11\x20\x03(\x0b2\x1b.BreakoutSyncConnectUidInfoR\x0buidInfoList\
    \x12G\n\x13dynamic_object_list\x18\x12\x20\x03(\x0b2\x17.BreakoutPhysica\
    lObjectR\x11dynamicObjectList\x12\"\n\rid_index_list\x18\x13\x20\x03(\rR\
    \x0bidIndexList\x12/\n\x14raw_client_game_time\x18\x14\x20\x01(\x05R\x11\
    rawClientGameTimeB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::BreakoutPhysicalObject::file_descriptor().clone());
            deps.push(super::BreakoutAction::file_descriptor().clone());
            deps.push(super::BreakoutSpawnPoint::file_descriptor().clone());
            deps.push(super::BreakoutElementReactionCounter::file_descriptor().clone());
            deps.push(super::BreakoutSyncConnectUidInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BreakoutSnapShot::generated_message_descriptor_data());
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
