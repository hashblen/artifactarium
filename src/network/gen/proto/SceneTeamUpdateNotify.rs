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

//! Generated file from `SceneTeamUpdateNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:SceneTeamUpdateNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneTeamUpdateNotify {
    // message fields
    // @@protoc_insertion_point(field:SceneTeamUpdateNotify.scene_team_avatar_list)
    pub scene_team_avatar_list: ::std::vec::Vec<super::SceneTeamAvatar::SceneTeamAvatar>,
    // @@protoc_insertion_point(field:SceneTeamUpdateNotify.is_in_mp)
    pub is_in_mp: bool,
    // special fields
    // @@protoc_insertion_point(special_field:SceneTeamUpdateNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneTeamUpdateNotify {
    fn default() -> &'a SceneTeamUpdateNotify {
        <SceneTeamUpdateNotify as ::protobuf::Message>::default_instance()
    }
}

impl SceneTeamUpdateNotify {
    pub fn new() -> SceneTeamUpdateNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "scene_team_avatar_list",
            |m: &SceneTeamUpdateNotify| { &m.scene_team_avatar_list },
            |m: &mut SceneTeamUpdateNotify| { &mut m.scene_team_avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_in_mp",
            |m: &SceneTeamUpdateNotify| { &m.is_in_mp },
            |m: &mut SceneTeamUpdateNotify| { &mut m.is_in_mp },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneTeamUpdateNotify>(
            "SceneTeamUpdateNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneTeamUpdateNotify {
    const NAME: &'static str = "SceneTeamUpdateNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.scene_team_avatar_list.push(is.read_message()?);
                },
                88 => {
                    self.is_in_mp = is.read_bool()?;
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
        for value in &self.scene_team_avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.is_in_mp != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.scene_team_avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.is_in_mp != false {
            os.write_bool(11, self.is_in_mp)?;
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

    fn new() -> SceneTeamUpdateNotify {
        SceneTeamUpdateNotify::new()
    }

    fn clear(&mut self) {
        self.scene_team_avatar_list.clear();
        self.is_in_mp = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneTeamUpdateNotify {
        static instance: SceneTeamUpdateNotify = SceneTeamUpdateNotify {
            scene_team_avatar_list: ::std::vec::Vec::new(),
            is_in_mp: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneTeamUpdateNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneTeamUpdateNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneTeamUpdateNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneTeamUpdateNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bSceneTeamUpdateNotify.proto\x1a\x15SceneTeamAvatar.proto\"x\n\x15S\
    ceneTeamUpdateNotify\x12E\n\x16scene_team_avatar_list\x18\x05\x20\x03(\
    \x0b2\x10.SceneTeamAvatarR\x13sceneTeamAvatarList\x12\x18\n\x08is_in_mp\
    \x18\x0b\x20\x01(\x08R\x06isInMpB\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            deps.push(super::SceneTeamAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneTeamUpdateNotify::generated_message_descriptor_data());
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
