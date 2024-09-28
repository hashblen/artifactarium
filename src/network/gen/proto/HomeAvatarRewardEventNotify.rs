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

//! Generated file from `HomeAvatarRewardEventNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:HomeAvatarRewardEventNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeAvatarRewardEventNotify {
    // message fields
    // @@protoc_insertion_point(field:HomeAvatarRewardEventNotify.is_event_trigger)
    pub is_event_trigger: bool,
    // @@protoc_insertion_point(field:HomeAvatarRewardEventNotify.reward_event)
    pub reward_event: ::protobuf::MessageField<super::HomeAvatarRewardEventInfo::HomeAvatarRewardEventInfo>,
    // @@protoc_insertion_point(field:HomeAvatarRewardEventNotify.pending_list)
    pub pending_list: ::std::vec::Vec<super::HomeAvatarRewardEventInfo::HomeAvatarRewardEventInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:HomeAvatarRewardEventNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeAvatarRewardEventNotify {
    fn default() -> &'a HomeAvatarRewardEventNotify {
        <HomeAvatarRewardEventNotify as ::protobuf::Message>::default_instance()
    }
}

impl HomeAvatarRewardEventNotify {
    pub fn new() -> HomeAvatarRewardEventNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_event_trigger",
            |m: &HomeAvatarRewardEventNotify| { &m.is_event_trigger },
            |m: &mut HomeAvatarRewardEventNotify| { &mut m.is_event_trigger },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HomeAvatarRewardEventInfo::HomeAvatarRewardEventInfo>(
            "reward_event",
            |m: &HomeAvatarRewardEventNotify| { &m.reward_event },
            |m: &mut HomeAvatarRewardEventNotify| { &mut m.reward_event },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "pending_list",
            |m: &HomeAvatarRewardEventNotify| { &m.pending_list },
            |m: &mut HomeAvatarRewardEventNotify| { &mut m.pending_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeAvatarRewardEventNotify>(
            "HomeAvatarRewardEventNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeAvatarRewardEventNotify {
    const NAME: &'static str = "HomeAvatarRewardEventNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.is_event_trigger = is.read_bool()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward_event)?;
                },
                50 => {
                    self.pending_list.push(is.read_message()?);
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
        if self.is_event_trigger != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.reward_event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.pending_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_event_trigger != false {
            os.write_bool(1, self.is_event_trigger)?;
        }
        if let Some(v) = self.reward_event.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        for v in &self.pending_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> HomeAvatarRewardEventNotify {
        HomeAvatarRewardEventNotify::new()
    }

    fn clear(&mut self) {
        self.is_event_trigger = false;
        self.reward_event.clear();
        self.pending_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeAvatarRewardEventNotify {
        static instance: HomeAvatarRewardEventNotify = HomeAvatarRewardEventNotify {
            is_event_trigger: false,
            reward_event: ::protobuf::MessageField::none(),
            pending_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeAvatarRewardEventNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeAvatarRewardEventNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeAvatarRewardEventNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeAvatarRewardEventNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!HomeAvatarRewardEventNotify.proto\x1a\x1fHomeAvatarRewardEventInfo.pr\
    oto\"\xc5\x01\n\x1bHomeAvatarRewardEventNotify\x12(\n\x10is_event_trigge\
    r\x18\x01\x20\x01(\x08R\x0eisEventTrigger\x12=\n\x0creward_event\x18\x04\
    \x20\x01(\x0b2\x1a.HomeAvatarRewardEventInfoR\x0brewardEvent\x12=\n\x0cp\
    ending_list\x18\x06\x20\x03(\x0b2\x1a.HomeAvatarRewardEventInfoR\x0bpend\
    ingListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::HomeAvatarRewardEventInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HomeAvatarRewardEventNotify::generated_message_descriptor_data());
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
