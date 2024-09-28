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

//! Generated file from `BreakoutAction.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:BreakoutAction)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BreakoutAction {
    // message fields
    // @@protoc_insertion_point(field:BreakoutAction.action_type)
    pub action_type: ::protobuf::EnumOrUnknown<super::BreakoutActionType::BreakoutActionType>,
    // @@protoc_insertion_point(field:BreakoutAction.client_game_time)
    pub client_game_time: u64,
    // @@protoc_insertion_point(field:BreakoutAction.server_game_time)
    pub server_game_time: u64,
    // @@protoc_insertion_point(field:BreakoutAction.is_failed)
    pub is_failed: bool,
    // @@protoc_insertion_point(field:BreakoutAction.pre_index)
    pub pre_index: u32,
    // @@protoc_insertion_point(field:BreakoutAction.new_index)
    pub new_index: u32,
    // @@protoc_insertion_point(field:BreakoutAction.pos)
    pub pos: ::protobuf::MessageField<super::BreakoutVector2::BreakoutVector2>,
    // @@protoc_insertion_point(field:BreakoutAction.move_dir)
    pub move_dir: ::protobuf::MessageField<super::BreakoutVector2::BreakoutVector2>,
    // @@protoc_insertion_point(field:BreakoutAction.speed)
    pub speed: i32,
    // @@protoc_insertion_point(field:BreakoutAction.peer_id)
    pub peer_id: u32,
    // @@protoc_insertion_point(field:BreakoutAction.element_type)
    pub element_type: u32,
    // @@protoc_insertion_point(field:BreakoutAction.element_reaction_buff)
    pub element_reaction_buff: u32,
    // @@protoc_insertion_point(field:BreakoutAction.speed_increase_count)
    pub speed_increase_count: u32,
    // @@protoc_insertion_point(field:BreakoutAction.has_extra_ball)
    pub has_extra_ball: bool,
    // @@protoc_insertion_point(field:BreakoutAction.extra_ball_dir)
    pub extra_ball_dir: ::protobuf::MessageField<super::BreakoutVector2::BreakoutVector2>,
    // @@protoc_insertion_point(field:BreakoutAction.extra_ball_index)
    pub extra_ball_index: u32,
    // @@protoc_insertion_point(field:BreakoutAction.offset)
    pub offset: i32,
    // @@protoc_insertion_point(field:BreakoutAction.CLKEPICNJJD)
    pub CLKEPICNJJD: u64,
    // special fields
    // @@protoc_insertion_point(special_field:BreakoutAction.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BreakoutAction {
    fn default() -> &'a BreakoutAction {
        <BreakoutAction as ::protobuf::Message>::default_instance()
    }
}

impl BreakoutAction {
    pub fn new() -> BreakoutAction {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(18);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "action_type",
            |m: &BreakoutAction| { &m.action_type },
            |m: &mut BreakoutAction| { &mut m.action_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_game_time",
            |m: &BreakoutAction| { &m.client_game_time },
            |m: &mut BreakoutAction| { &mut m.client_game_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "server_game_time",
            |m: &BreakoutAction| { &m.server_game_time },
            |m: &mut BreakoutAction| { &mut m.server_game_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_failed",
            |m: &BreakoutAction| { &m.is_failed },
            |m: &mut BreakoutAction| { &mut m.is_failed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pre_index",
            |m: &BreakoutAction| { &m.pre_index },
            |m: &mut BreakoutAction| { &mut m.pre_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "new_index",
            |m: &BreakoutAction| { &m.new_index },
            |m: &mut BreakoutAction| { &mut m.new_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BreakoutVector2::BreakoutVector2>(
            "pos",
            |m: &BreakoutAction| { &m.pos },
            |m: &mut BreakoutAction| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BreakoutVector2::BreakoutVector2>(
            "move_dir",
            |m: &BreakoutAction| { &m.move_dir },
            |m: &mut BreakoutAction| { &mut m.move_dir },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "speed",
            |m: &BreakoutAction| { &m.speed },
            |m: &mut BreakoutAction| { &mut m.speed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "peer_id",
            |m: &BreakoutAction| { &m.peer_id },
            |m: &mut BreakoutAction| { &mut m.peer_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "element_type",
            |m: &BreakoutAction| { &m.element_type },
            |m: &mut BreakoutAction| { &mut m.element_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "element_reaction_buff",
            |m: &BreakoutAction| { &m.element_reaction_buff },
            |m: &mut BreakoutAction| { &mut m.element_reaction_buff },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "speed_increase_count",
            |m: &BreakoutAction| { &m.speed_increase_count },
            |m: &mut BreakoutAction| { &mut m.speed_increase_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "has_extra_ball",
            |m: &BreakoutAction| { &m.has_extra_ball },
            |m: &mut BreakoutAction| { &mut m.has_extra_ball },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BreakoutVector2::BreakoutVector2>(
            "extra_ball_dir",
            |m: &BreakoutAction| { &m.extra_ball_dir },
            |m: &mut BreakoutAction| { &mut m.extra_ball_dir },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_ball_index",
            |m: &BreakoutAction| { &m.extra_ball_index },
            |m: &mut BreakoutAction| { &mut m.extra_ball_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "offset",
            |m: &BreakoutAction| { &m.offset },
            |m: &mut BreakoutAction| { &mut m.offset },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLKEPICNJJD",
            |m: &BreakoutAction| { &m.CLKEPICNJJD },
            |m: &mut BreakoutAction| { &mut m.CLKEPICNJJD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BreakoutAction>(
            "BreakoutAction",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BreakoutAction {
    const NAME: &'static str = "BreakoutAction";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.action_type = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.client_game_time = is.read_uint64()?;
                },
                24 => {
                    self.server_game_time = is.read_uint64()?;
                },
                32 => {
                    self.is_failed = is.read_bool()?;
                },
                40 => {
                    self.pre_index = is.read_uint32()?;
                },
                48 => {
                    self.new_index = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pos)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.move_dir)?;
                },
                72 => {
                    self.speed = is.read_int32()?;
                },
                80 => {
                    self.peer_id = is.read_uint32()?;
                },
                88 => {
                    self.element_type = is.read_uint32()?;
                },
                96 => {
                    self.element_reaction_buff = is.read_uint32()?;
                },
                104 => {
                    self.speed_increase_count = is.read_uint32()?;
                },
                112 => {
                    self.has_extra_ball = is.read_bool()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.extra_ball_dir)?;
                },
                128 => {
                    self.extra_ball_index = is.read_uint32()?;
                },
                136 => {
                    self.offset = is.read_int32()?;
                },
                144 => {
                    self.CLKEPICNJJD = is.read_uint64()?;
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
        if self.action_type != ::protobuf::EnumOrUnknown::new(super::BreakoutActionType::BreakoutActionType::BREAKOUT_ACTION_TYPE_ACTION_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.action_type.value());
        }
        if self.client_game_time != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.client_game_time);
        }
        if self.server_game_time != 0 {
            my_size += ::protobuf::rt::uint64_size(3, self.server_game_time);
        }
        if self.is_failed != false {
            my_size += 1 + 1;
        }
        if self.pre_index != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.pre_index);
        }
        if self.new_index != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.new_index);
        }
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.move_dir.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.speed != 0 {
            my_size += ::protobuf::rt::int32_size(9, self.speed);
        }
        if self.peer_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.peer_id);
        }
        if self.element_type != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.element_type);
        }
        if self.element_reaction_buff != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.element_reaction_buff);
        }
        if self.speed_increase_count != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.speed_increase_count);
        }
        if self.has_extra_ball != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.extra_ball_dir.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.extra_ball_index != 0 {
            my_size += ::protobuf::rt::uint32_size(16, self.extra_ball_index);
        }
        if self.offset != 0 {
            my_size += ::protobuf::rt::int32_size(17, self.offset);
        }
        if self.CLKEPICNJJD != 0 {
            my_size += ::protobuf::rt::uint64_size(18, self.CLKEPICNJJD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.action_type != ::protobuf::EnumOrUnknown::new(super::BreakoutActionType::BreakoutActionType::BREAKOUT_ACTION_TYPE_ACTION_TYPE_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.action_type))?;
        }
        if self.client_game_time != 0 {
            os.write_uint64(2, self.client_game_time)?;
        }
        if self.server_game_time != 0 {
            os.write_uint64(3, self.server_game_time)?;
        }
        if self.is_failed != false {
            os.write_bool(4, self.is_failed)?;
        }
        if self.pre_index != 0 {
            os.write_uint32(5, self.pre_index)?;
        }
        if self.new_index != 0 {
            os.write_uint32(6, self.new_index)?;
        }
        if let Some(v) = self.pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.move_dir.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.speed != 0 {
            os.write_int32(9, self.speed)?;
        }
        if self.peer_id != 0 {
            os.write_uint32(10, self.peer_id)?;
        }
        if self.element_type != 0 {
            os.write_uint32(11, self.element_type)?;
        }
        if self.element_reaction_buff != 0 {
            os.write_uint32(12, self.element_reaction_buff)?;
        }
        if self.speed_increase_count != 0 {
            os.write_uint32(13, self.speed_increase_count)?;
        }
        if self.has_extra_ball != false {
            os.write_bool(14, self.has_extra_ball)?;
        }
        if let Some(v) = self.extra_ball_dir.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.extra_ball_index != 0 {
            os.write_uint32(16, self.extra_ball_index)?;
        }
        if self.offset != 0 {
            os.write_int32(17, self.offset)?;
        }
        if self.CLKEPICNJJD != 0 {
            os.write_uint64(18, self.CLKEPICNJJD)?;
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

    fn new() -> BreakoutAction {
        BreakoutAction::new()
    }

    fn clear(&mut self) {
        self.action_type = ::protobuf::EnumOrUnknown::new(super::BreakoutActionType::BreakoutActionType::BREAKOUT_ACTION_TYPE_ACTION_TYPE_NONE);
        self.client_game_time = 0;
        self.server_game_time = 0;
        self.is_failed = false;
        self.pre_index = 0;
        self.new_index = 0;
        self.pos.clear();
        self.move_dir.clear();
        self.speed = 0;
        self.peer_id = 0;
        self.element_type = 0;
        self.element_reaction_buff = 0;
        self.speed_increase_count = 0;
        self.has_extra_ball = false;
        self.extra_ball_dir.clear();
        self.extra_ball_index = 0;
        self.offset = 0;
        self.CLKEPICNJJD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BreakoutAction {
        static instance: BreakoutAction = BreakoutAction {
            action_type: ::protobuf::EnumOrUnknown::from_i32(0),
            client_game_time: 0,
            server_game_time: 0,
            is_failed: false,
            pre_index: 0,
            new_index: 0,
            pos: ::protobuf::MessageField::none(),
            move_dir: ::protobuf::MessageField::none(),
            speed: 0,
            peer_id: 0,
            element_type: 0,
            element_reaction_buff: 0,
            speed_increase_count: 0,
            has_extra_ball: false,
            extra_ball_dir: ::protobuf::MessageField::none(),
            extra_ball_index: 0,
            offset: 0,
            CLKEPICNJJD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BreakoutAction {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BreakoutAction").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BreakoutAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BreakoutAction {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14BreakoutAction.proto\x1a\x18BreakoutActionType.proto\x1a\x15Breako\
    utVector2.proto\"\xbc\x05\n\x0eBreakoutAction\x124\n\x0baction_type\x18\
    \x01\x20\x01(\x0e2\x13.BreakoutActionTypeR\nactionType\x12(\n\x10client_\
    game_time\x18\x02\x20\x01(\x04R\x0eclientGameTime\x12(\n\x10server_game_\
    time\x18\x03\x20\x01(\x04R\x0eserverGameTime\x12\x1b\n\tis_failed\x18\
    \x04\x20\x01(\x08R\x08isFailed\x12\x1b\n\tpre_index\x18\x05\x20\x01(\rR\
    \x08preIndex\x12\x1b\n\tnew_index\x18\x06\x20\x01(\rR\x08newIndex\x12\"\
    \n\x03pos\x18\x07\x20\x01(\x0b2\x10.BreakoutVector2R\x03pos\x12+\n\x08mo\
    ve_dir\x18\x08\x20\x01(\x0b2\x10.BreakoutVector2R\x07moveDir\x12\x14\n\
    \x05speed\x18\t\x20\x01(\x05R\x05speed\x12\x17\n\x07peer_id\x18\n\x20\
    \x01(\rR\x06peerId\x12!\n\x0celement_type\x18\x0b\x20\x01(\rR\x0belement\
    Type\x122\n\x15element_reaction_buff\x18\x0c\x20\x01(\rR\x13elementReact\
    ionBuff\x120\n\x14speed_increase_count\x18\r\x20\x01(\rR\x12speedIncreas\
    eCount\x12$\n\x0ehas_extra_ball\x18\x0e\x20\x01(\x08R\x0chasExtraBall\
    \x126\n\x0eextra_ball_dir\x18\x0f\x20\x01(\x0b2\x10.BreakoutVector2R\x0c\
    extraBallDir\x12(\n\x10extra_ball_index\x18\x10\x20\x01(\rR\x0eextraBall\
    Index\x12\x16\n\x06offset\x18\x11\x20\x01(\x05R\x06offset\x12\x20\n\x0bC\
    LKEPICNJJD\x18\x12\x20\x01(\x04R\x0bCLKEPICNJJDB\x1b\n\x19emu.grasscutte\
    r.net.protob\x06proto3\
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
            deps.push(super::BreakoutActionType::file_descriptor().clone());
            deps.push(super::BreakoutVector2::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BreakoutAction::generated_message_descriptor_data());
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
