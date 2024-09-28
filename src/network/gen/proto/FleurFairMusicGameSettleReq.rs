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

//! Generated file from `FleurFairMusicGameSettleReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:FleurFairMusicGameSettleReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FleurFairMusicGameSettleReq {
    // message fields
    // @@protoc_insertion_point(field:FleurFairMusicGameSettleReq.combo)
    pub combo: u32,
    // @@protoc_insertion_point(field:FleurFairMusicGameSettleReq.music_basic_id)
    pub music_basic_id: u32,
    // @@protoc_insertion_point(field:FleurFairMusicGameSettleReq.score)
    pub score: u32,
    // @@protoc_insertion_point(field:FleurFairMusicGameSettleReq.correct_hit)
    pub correct_hit: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FleurFairMusicGameSettleReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FleurFairMusicGameSettleReq {
    fn default() -> &'a FleurFairMusicGameSettleReq {
        <FleurFairMusicGameSettleReq as ::protobuf::Message>::default_instance()
    }
}

impl FleurFairMusicGameSettleReq {
    pub fn new() -> FleurFairMusicGameSettleReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "combo",
            |m: &FleurFairMusicGameSettleReq| { &m.combo },
            |m: &mut FleurFairMusicGameSettleReq| { &mut m.combo },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "music_basic_id",
            |m: &FleurFairMusicGameSettleReq| { &m.music_basic_id },
            |m: &mut FleurFairMusicGameSettleReq| { &mut m.music_basic_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &FleurFairMusicGameSettleReq| { &m.score },
            |m: &mut FleurFairMusicGameSettleReq| { &mut m.score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "correct_hit",
            |m: &FleurFairMusicGameSettleReq| { &m.correct_hit },
            |m: &mut FleurFairMusicGameSettleReq| { &mut m.correct_hit },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FleurFairMusicGameSettleReq>(
            "FleurFairMusicGameSettleReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FleurFairMusicGameSettleReq {
    const NAME: &'static str = "FleurFairMusicGameSettleReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.combo = is.read_uint32()?;
                },
                80 => {
                    self.music_basic_id = is.read_uint32()?;
                },
                32 => {
                    self.score = is.read_uint32()?;
                },
                88 => {
                    self.correct_hit = is.read_uint32()?;
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
        if self.combo != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.combo);
        }
        if self.music_basic_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.music_basic_id);
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.score);
        }
        if self.correct_hit != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.correct_hit);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.combo != 0 {
            os.write_uint32(13, self.combo)?;
        }
        if self.music_basic_id != 0 {
            os.write_uint32(10, self.music_basic_id)?;
        }
        if self.score != 0 {
            os.write_uint32(4, self.score)?;
        }
        if self.correct_hit != 0 {
            os.write_uint32(11, self.correct_hit)?;
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

    fn new() -> FleurFairMusicGameSettleReq {
        FleurFairMusicGameSettleReq::new()
    }

    fn clear(&mut self) {
        self.combo = 0;
        self.music_basic_id = 0;
        self.score = 0;
        self.correct_hit = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FleurFairMusicGameSettleReq {
        static instance: FleurFairMusicGameSettleReq = FleurFairMusicGameSettleReq {
            combo: 0,
            music_basic_id: 0,
            score: 0,
            correct_hit: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FleurFairMusicGameSettleReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FleurFairMusicGameSettleReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FleurFairMusicGameSettleReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FleurFairMusicGameSettleReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!FleurFairMusicGameSettleReq.proto\"\x90\x01\n\x1bFleurFairMusicGameSe\
    ttleReq\x12\x14\n\x05combo\x18\r\x20\x01(\rR\x05combo\x12$\n\x0emusic_ba\
    sic_id\x18\n\x20\x01(\rR\x0cmusicBasicId\x12\x14\n\x05score\x18\x04\x20\
    \x01(\rR\x05score\x12\x1f\n\x0bcorrect_hit\x18\x0b\x20\x01(\rR\ncorrectH\
    itB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(FleurFairMusicGameSettleReq::generated_message_descriptor_data());
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
