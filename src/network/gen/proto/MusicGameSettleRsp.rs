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

//! Generated file from `MusicGameSettleRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:MusicGameSettleRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MusicGameSettleRsp {
    // message fields
    // @@protoc_insertion_point(field:MusicGameSettleRsp.ugc_guid)
    pub ugc_guid: u64,
    // @@protoc_insertion_point(field:MusicGameSettleRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:MusicGameSettleRsp.music_basic_id)
    pub music_basic_id: u32,
    // @@protoc_insertion_point(field:MusicGameSettleRsp.is_unlock_next_level)
    pub is_unlock_next_level: bool,
    // @@protoc_insertion_point(field:MusicGameSettleRsp.is_new_record)
    pub is_new_record: bool,
    // special fields
    // @@protoc_insertion_point(special_field:MusicGameSettleRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MusicGameSettleRsp {
    fn default() -> &'a MusicGameSettleRsp {
        <MusicGameSettleRsp as ::protobuf::Message>::default_instance()
    }
}

impl MusicGameSettleRsp {
    pub fn new() -> MusicGameSettleRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ugc_guid",
            |m: &MusicGameSettleRsp| { &m.ugc_guid },
            |m: &mut MusicGameSettleRsp| { &mut m.ugc_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MusicGameSettleRsp| { &m.retcode },
            |m: &mut MusicGameSettleRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "music_basic_id",
            |m: &MusicGameSettleRsp| { &m.music_basic_id },
            |m: &mut MusicGameSettleRsp| { &mut m.music_basic_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_unlock_next_level",
            |m: &MusicGameSettleRsp| { &m.is_unlock_next_level },
            |m: &mut MusicGameSettleRsp| { &mut m.is_unlock_next_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_new_record",
            |m: &MusicGameSettleRsp| { &m.is_new_record },
            |m: &mut MusicGameSettleRsp| { &mut m.is_new_record },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MusicGameSettleRsp>(
            "MusicGameSettleRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MusicGameSettleRsp {
    const NAME: &'static str = "MusicGameSettleRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.ugc_guid = is.read_uint64()?;
                },
                96 => {
                    self.retcode = is.read_int32()?;
                },
                56 => {
                    self.music_basic_id = is.read_uint32()?;
                },
                24 => {
                    self.is_unlock_next_level = is.read_bool()?;
                },
                48 => {
                    self.is_new_record = is.read_bool()?;
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
        if self.ugc_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.ugc_guid);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(12, self.retcode);
        }
        if self.music_basic_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.music_basic_id);
        }
        if self.is_unlock_next_level != false {
            my_size += 1 + 1;
        }
        if self.is_new_record != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ugc_guid != 0 {
            os.write_uint64(9, self.ugc_guid)?;
        }
        if self.retcode != 0 {
            os.write_int32(12, self.retcode)?;
        }
        if self.music_basic_id != 0 {
            os.write_uint32(7, self.music_basic_id)?;
        }
        if self.is_unlock_next_level != false {
            os.write_bool(3, self.is_unlock_next_level)?;
        }
        if self.is_new_record != false {
            os.write_bool(6, self.is_new_record)?;
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

    fn new() -> MusicGameSettleRsp {
        MusicGameSettleRsp::new()
    }

    fn clear(&mut self) {
        self.ugc_guid = 0;
        self.retcode = 0;
        self.music_basic_id = 0;
        self.is_unlock_next_level = false;
        self.is_new_record = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MusicGameSettleRsp {
        static instance: MusicGameSettleRsp = MusicGameSettleRsp {
            ugc_guid: 0,
            retcode: 0,
            music_basic_id: 0,
            is_unlock_next_level: false,
            is_new_record: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MusicGameSettleRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MusicGameSettleRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MusicGameSettleRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MusicGameSettleRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18MusicGameSettleRsp.proto\"\xc4\x01\n\x12MusicGameSettleRsp\x12\x19\
    \n\x08ugc_guid\x18\t\x20\x01(\x04R\x07ugcGuid\x12\x18\n\x07retcode\x18\
    \x0c\x20\x01(\x05R\x07retcode\x12$\n\x0emusic_basic_id\x18\x07\x20\x01(\
    \rR\x0cmusicBasicId\x12/\n\x14is_unlock_next_level\x18\x03\x20\x01(\x08R\
    \x11isUnlockNextLevel\x12\"\n\ris_new_record\x18\x06\x20\x01(\x08R\x0bis\
    NewRecordB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(MusicGameSettleRsp::generated_message_descriptor_data());
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
