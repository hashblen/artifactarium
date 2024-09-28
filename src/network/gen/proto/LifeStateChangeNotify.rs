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

//! Generated file from `LifeStateChangeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:LifeStateChangeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LifeStateChangeNotify {
    // message fields
    // @@protoc_insertion_point(field:LifeStateChangeNotify.attack_tag)
    pub attack_tag: ::std::string::String,
    // @@protoc_insertion_point(field:LifeStateChangeNotify.move_reliable_seq)
    pub move_reliable_seq: u32,
    // @@protoc_insertion_point(field:LifeStateChangeNotify.EPGOBHIEDOI)
    pub EPGOBHIEDOI: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:LifeStateChangeNotify.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:LifeStateChangeNotify.life_state)
    pub life_state: u32,
    // @@protoc_insertion_point(field:LifeStateChangeNotify.die_type)
    pub die_type: ::protobuf::EnumOrUnknown<super::PlayerDieType::PlayerDieType>,
    // @@protoc_insertion_point(field:LifeStateChangeNotify.server_buff_list)
    pub server_buff_list: ::std::vec::Vec<super::ServerBuff::ServerBuff>,
    // @@protoc_insertion_point(field:LifeStateChangeNotify.source_entity_id)
    pub source_entity_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LifeStateChangeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LifeStateChangeNotify {
    fn default() -> &'a LifeStateChangeNotify {
        <LifeStateChangeNotify as ::protobuf::Message>::default_instance()
    }
}

impl LifeStateChangeNotify {
    pub fn new() -> LifeStateChangeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attack_tag",
            |m: &LifeStateChangeNotify| { &m.attack_tag },
            |m: &mut LifeStateChangeNotify| { &mut m.attack_tag },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "move_reliable_seq",
            |m: &LifeStateChangeNotify| { &m.move_reliable_seq },
            |m: &mut LifeStateChangeNotify| { &mut m.move_reliable_seq },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EPGOBHIEDOI",
            |m: &LifeStateChangeNotify| { &m.EPGOBHIEDOI },
            |m: &mut LifeStateChangeNotify| { &mut m.EPGOBHIEDOI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &LifeStateChangeNotify| { &m.entity_id },
            |m: &mut LifeStateChangeNotify| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "life_state",
            |m: &LifeStateChangeNotify| { &m.life_state },
            |m: &mut LifeStateChangeNotify| { &mut m.life_state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "die_type",
            |m: &LifeStateChangeNotify| { &m.die_type },
            |m: &mut LifeStateChangeNotify| { &mut m.die_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "server_buff_list",
            |m: &LifeStateChangeNotify| { &m.server_buff_list },
            |m: &mut LifeStateChangeNotify| { &mut m.server_buff_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "source_entity_id",
            |m: &LifeStateChangeNotify| { &m.source_entity_id },
            |m: &mut LifeStateChangeNotify| { &mut m.source_entity_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LifeStateChangeNotify>(
            "LifeStateChangeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LifeStateChangeNotify {
    const NAME: &'static str = "LifeStateChangeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.attack_tag = is.read_string()?;
                },
                24 => {
                    self.move_reliable_seq = is.read_uint32()?;
                },
                42 => {
                    self.EPGOBHIEDOI.push(is.read_string()?);
                },
                48 => {
                    self.entity_id = is.read_uint32()?;
                },
                64 => {
                    self.life_state = is.read_uint32()?;
                },
                80 => {
                    self.die_type = is.read_enum_or_unknown()?;
                },
                106 => {
                    self.server_buff_list.push(is.read_message()?);
                },
                112 => {
                    self.source_entity_id = is.read_uint32()?;
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
        if !self.attack_tag.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.attack_tag);
        }
        if self.move_reliable_seq != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.move_reliable_seq);
        }
        for value in &self.EPGOBHIEDOI {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.entity_id);
        }
        if self.life_state != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.life_state);
        }
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.die_type.value());
        }
        for value in &self.server_buff_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.source_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.source_entity_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.attack_tag.is_empty() {
            os.write_string(2, &self.attack_tag)?;
        }
        if self.move_reliable_seq != 0 {
            os.write_uint32(3, self.move_reliable_seq)?;
        }
        for v in &self.EPGOBHIEDOI {
            os.write_string(5, &v)?;
        };
        if self.entity_id != 0 {
            os.write_uint32(6, self.entity_id)?;
        }
        if self.life_state != 0 {
            os.write_uint32(8, self.life_state)?;
        }
        if self.die_type != ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.die_type))?;
        }
        for v in &self.server_buff_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.source_entity_id != 0 {
            os.write_uint32(14, self.source_entity_id)?;
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

    fn new() -> LifeStateChangeNotify {
        LifeStateChangeNotify::new()
    }

    fn clear(&mut self) {
        self.attack_tag.clear();
        self.move_reliable_seq = 0;
        self.EPGOBHIEDOI.clear();
        self.entity_id = 0;
        self.life_state = 0;
        self.die_type = ::protobuf::EnumOrUnknown::new(super::PlayerDieType::PlayerDieType::PLAYER_DIE_TYPE_NONE);
        self.server_buff_list.clear();
        self.source_entity_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LifeStateChangeNotify {
        static instance: LifeStateChangeNotify = LifeStateChangeNotify {
            attack_tag: ::std::string::String::new(),
            move_reliable_seq: 0,
            EPGOBHIEDOI: ::std::vec::Vec::new(),
            entity_id: 0,
            life_state: 0,
            die_type: ::protobuf::EnumOrUnknown::from_i32(0),
            server_buff_list: ::std::vec::Vec::new(),
            source_entity_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LifeStateChangeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LifeStateChangeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LifeStateChangeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LifeStateChangeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bLifeStateChangeNotify.proto\x1a\x13PlayerDieType.proto\x1a\x10Serv\
    erBuff.proto\"\xcc\x02\n\x15LifeStateChangeNotify\x12\x1d\n\nattack_tag\
    \x18\x02\x20\x01(\tR\tattackTag\x12*\n\x11move_reliable_seq\x18\x03\x20\
    \x01(\rR\x0fmoveReliableSeq\x12\x20\n\x0bEPGOBHIEDOI\x18\x05\x20\x03(\tR\
    \x0bEPGOBHIEDOI\x12\x1b\n\tentity_id\x18\x06\x20\x01(\rR\x08entityId\x12\
    \x1d\n\nlife_state\x18\x08\x20\x01(\rR\tlifeState\x12)\n\x08die_type\x18\
    \n\x20\x01(\x0e2\x0e.PlayerDieTypeR\x07dieType\x125\n\x10server_buff_lis\
    t\x18\r\x20\x03(\x0b2\x0b.ServerBuffR\x0eserverBuffList\x12(\n\x10source\
    _entity_id\x18\x0e\x20\x01(\rR\x0esourceEntityIdB\x1b\n\x19emu.grasscutt\
    er.net.protob\x06proto3\
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
            deps.push(super::ServerBuff::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LifeStateChangeNotify::generated_message_descriptor_data());
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
