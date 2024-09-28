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

//! Generated file from `PersonalSceneJumpRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:PersonalSceneJumpRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PersonalSceneJumpRsp {
    // message fields
    // @@protoc_insertion_point(field:PersonalSceneJumpRsp.dest_pos)
    pub dest_pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:PersonalSceneJumpRsp.dest_scene_id)
    pub dest_scene_id: u32,
    // @@protoc_insertion_point(field:PersonalSceneJumpRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:PersonalSceneJumpRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PersonalSceneJumpRsp {
    fn default() -> &'a PersonalSceneJumpRsp {
        <PersonalSceneJumpRsp as ::protobuf::Message>::default_instance()
    }
}

impl PersonalSceneJumpRsp {
    pub fn new() -> PersonalSceneJumpRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "dest_pos",
            |m: &PersonalSceneJumpRsp| { &m.dest_pos },
            |m: &mut PersonalSceneJumpRsp| { &mut m.dest_pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dest_scene_id",
            |m: &PersonalSceneJumpRsp| { &m.dest_scene_id },
            |m: &mut PersonalSceneJumpRsp| { &mut m.dest_scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &PersonalSceneJumpRsp| { &m.retcode },
            |m: &mut PersonalSceneJumpRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PersonalSceneJumpRsp>(
            "PersonalSceneJumpRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PersonalSceneJumpRsp {
    const NAME: &'static str = "PersonalSceneJumpRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.dest_pos)?;
                },
                112 => {
                    self.dest_scene_id = is.read_uint32()?;
                },
                24 => {
                    self.retcode = is.read_int32()?;
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
        if let Some(v) = self.dest_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.dest_scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.dest_scene_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.dest_pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.dest_scene_id != 0 {
            os.write_uint32(14, self.dest_scene_id)?;
        }
        if self.retcode != 0 {
            os.write_int32(3, self.retcode)?;
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

    fn new() -> PersonalSceneJumpRsp {
        PersonalSceneJumpRsp::new()
    }

    fn clear(&mut self) {
        self.dest_pos.clear();
        self.dest_scene_id = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PersonalSceneJumpRsp {
        static instance: PersonalSceneJumpRsp = PersonalSceneJumpRsp {
            dest_pos: ::protobuf::MessageField::none(),
            dest_scene_id: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PersonalSceneJumpRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PersonalSceneJumpRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PersonalSceneJumpRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PersonalSceneJumpRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aPersonalSceneJumpRsp.proto\x1a\x0cVector.proto\"x\n\x14PersonalSce\
    neJumpRsp\x12\"\n\x08dest_pos\x18\x05\x20\x01(\x0b2\x07.VectorR\x07destP\
    os\x12\"\n\rdest_scene_id\x18\x0e\x20\x01(\rR\x0bdestSceneId\x12\x18\n\
    \x07retcode\x18\x03\x20\x01(\x05R\x07retcodeB\x1b\n\x19emu.grasscutter.n\
    et.protob\x06proto3\
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
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PersonalSceneJumpRsp::generated_message_descriptor_data());
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
