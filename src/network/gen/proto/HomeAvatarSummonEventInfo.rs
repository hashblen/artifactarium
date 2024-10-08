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

//! Generated file from `HomeAvatarSummonEventInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:HomeAvatarSummonEventInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeAvatarSummonEventInfo {
    // message fields
    // @@protoc_insertion_point(field:HomeAvatarSummonEventInfo.event_id)
    pub event_id: u32,
    // @@protoc_insertion_point(field:HomeAvatarSummonEventInfo.guid)
    pub guid: u32,
    // @@protoc_insertion_point(field:HomeAvatarSummonEventInfo.event_over_time)
    pub event_over_time: u32,
    // @@protoc_insertion_point(field:HomeAvatarSummonEventInfo.suit_id)
    pub suit_id: u32,
    // @@protoc_insertion_point(field:HomeAvatarSummonEventInfo.random_position)
    pub random_position: u32,
    // @@protoc_insertion_point(field:HomeAvatarSummonEventInfo.avatar_id)
    pub avatar_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HomeAvatarSummonEventInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeAvatarSummonEventInfo {
    fn default() -> &'a HomeAvatarSummonEventInfo {
        <HomeAvatarSummonEventInfo as ::protobuf::Message>::default_instance()
    }
}

impl HomeAvatarSummonEventInfo {
    pub fn new() -> HomeAvatarSummonEventInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_id",
            |m: &HomeAvatarSummonEventInfo| { &m.event_id },
            |m: &mut HomeAvatarSummonEventInfo| { &mut m.event_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "guid",
            |m: &HomeAvatarSummonEventInfo| { &m.guid },
            |m: &mut HomeAvatarSummonEventInfo| { &mut m.guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_over_time",
            |m: &HomeAvatarSummonEventInfo| { &m.event_over_time },
            |m: &mut HomeAvatarSummonEventInfo| { &mut m.event_over_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "suit_id",
            |m: &HomeAvatarSummonEventInfo| { &m.suit_id },
            |m: &mut HomeAvatarSummonEventInfo| { &mut m.suit_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "random_position",
            |m: &HomeAvatarSummonEventInfo| { &m.random_position },
            |m: &mut HomeAvatarSummonEventInfo| { &mut m.random_position },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &HomeAvatarSummonEventInfo| { &m.avatar_id },
            |m: &mut HomeAvatarSummonEventInfo| { &mut m.avatar_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeAvatarSummonEventInfo>(
            "HomeAvatarSummonEventInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeAvatarSummonEventInfo {
    const NAME: &'static str = "HomeAvatarSummonEventInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.event_id = is.read_uint32()?;
                },
                32 => {
                    self.guid = is.read_uint32()?;
                },
                56 => {
                    self.event_over_time = is.read_uint32()?;
                },
                72 => {
                    self.suit_id = is.read_uint32()?;
                },
                88 => {
                    self.random_position = is.read_uint32()?;
                },
                112 => {
                    self.avatar_id = is.read_uint32()?;
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
        if self.event_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.event_id);
        }
        if self.guid != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.guid);
        }
        if self.event_over_time != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.event_over_time);
        }
        if self.suit_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.suit_id);
        }
        if self.random_position != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.random_position);
        }
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.avatar_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.event_id != 0 {
            os.write_uint32(2, self.event_id)?;
        }
        if self.guid != 0 {
            os.write_uint32(4, self.guid)?;
        }
        if self.event_over_time != 0 {
            os.write_uint32(7, self.event_over_time)?;
        }
        if self.suit_id != 0 {
            os.write_uint32(9, self.suit_id)?;
        }
        if self.random_position != 0 {
            os.write_uint32(11, self.random_position)?;
        }
        if self.avatar_id != 0 {
            os.write_uint32(14, self.avatar_id)?;
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

    fn new() -> HomeAvatarSummonEventInfo {
        HomeAvatarSummonEventInfo::new()
    }

    fn clear(&mut self) {
        self.event_id = 0;
        self.guid = 0;
        self.event_over_time = 0;
        self.suit_id = 0;
        self.random_position = 0;
        self.avatar_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeAvatarSummonEventInfo {
        static instance: HomeAvatarSummonEventInfo = HomeAvatarSummonEventInfo {
            event_id: 0,
            guid: 0,
            event_over_time: 0,
            suit_id: 0,
            random_position: 0,
            avatar_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeAvatarSummonEventInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeAvatarSummonEventInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeAvatarSummonEventInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeAvatarSummonEventInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fHomeAvatarSummonEventInfo.proto\"\xd1\x01\n\x19HomeAvatarSummonEve\
    ntInfo\x12\x19\n\x08event_id\x18\x02\x20\x01(\rR\x07eventId\x12\x12\n\
    \x04guid\x18\x04\x20\x01(\rR\x04guid\x12&\n\x0fevent_over_time\x18\x07\
    \x20\x01(\rR\reventOverTime\x12\x17\n\x07suit_id\x18\t\x20\x01(\rR\x06su\
    itId\x12'\n\x0frandom_position\x18\x0b\x20\x01(\rR\x0erandomPosition\x12\
    \x1b\n\tavatar_id\x18\x0e\x20\x01(\rR\x08avatarIdB\x1b\n\x19emu.grasscut\
    ter.net.protob\x06proto3\
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
            messages.push(HomeAvatarSummonEventInfo::generated_message_descriptor_data());
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
