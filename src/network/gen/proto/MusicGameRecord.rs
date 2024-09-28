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

//! Generated file from `MusicGameRecord.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:MusicGameRecord)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MusicGameRecord {
    // message fields
    // @@protoc_insertion_point(field:MusicGameRecord.max_combo)
    pub max_combo: u32,
    // @@protoc_insertion_point(field:MusicGameRecord.is_unlock)
    pub is_unlock: bool,
    // @@protoc_insertion_point(field:MusicGameRecord.max_score)
    pub max_score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MusicGameRecord.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MusicGameRecord {
    fn default() -> &'a MusicGameRecord {
        <MusicGameRecord as ::protobuf::Message>::default_instance()
    }
}

impl MusicGameRecord {
    pub fn new() -> MusicGameRecord {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_combo",
            |m: &MusicGameRecord| { &m.max_combo },
            |m: &mut MusicGameRecord| { &mut m.max_combo },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_unlock",
            |m: &MusicGameRecord| { &m.is_unlock },
            |m: &mut MusicGameRecord| { &mut m.is_unlock },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_score",
            |m: &MusicGameRecord| { &m.max_score },
            |m: &mut MusicGameRecord| { &mut m.max_score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MusicGameRecord>(
            "MusicGameRecord",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MusicGameRecord {
    const NAME: &'static str = "MusicGameRecord";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.max_combo = is.read_uint32()?;
                },
                56 => {
                    self.is_unlock = is.read_bool()?;
                },
                32 => {
                    self.max_score = is.read_uint32()?;
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
        if self.max_combo != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.max_combo);
        }
        if self.is_unlock != false {
            my_size += 1 + 1;
        }
        if self.max_score != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.max_score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.max_combo != 0 {
            os.write_uint32(15, self.max_combo)?;
        }
        if self.is_unlock != false {
            os.write_bool(7, self.is_unlock)?;
        }
        if self.max_score != 0 {
            os.write_uint32(4, self.max_score)?;
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

    fn new() -> MusicGameRecord {
        MusicGameRecord::new()
    }

    fn clear(&mut self) {
        self.max_combo = 0;
        self.is_unlock = false;
        self.max_score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MusicGameRecord {
        static instance: MusicGameRecord = MusicGameRecord {
            max_combo: 0,
            is_unlock: false,
            max_score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MusicGameRecord {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MusicGameRecord").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MusicGameRecord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MusicGameRecord {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15MusicGameRecord.proto\"h\n\x0fMusicGameRecord\x12\x1b\n\tmax_combo\
    \x18\x0f\x20\x01(\rR\x08maxCombo\x12\x1b\n\tis_unlock\x18\x07\x20\x01(\
    \x08R\x08isUnlock\x12\x1b\n\tmax_score\x18\x04\x20\x01(\rR\x08maxScoreB\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(MusicGameRecord::generated_message_descriptor_data());
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
