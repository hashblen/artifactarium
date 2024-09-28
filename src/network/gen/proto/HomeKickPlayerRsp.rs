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

//! Generated file from `HomeKickPlayerRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:HomeKickPlayerRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeKickPlayerRsp {
    // message fields
    // @@protoc_insertion_point(field:HomeKickPlayerRsp.is_kick_all)
    pub is_kick_all: bool,
    // @@protoc_insertion_point(field:HomeKickPlayerRsp.target_uid)
    pub target_uid: u32,
    // @@protoc_insertion_point(field:HomeKickPlayerRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:HomeKickPlayerRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeKickPlayerRsp {
    fn default() -> &'a HomeKickPlayerRsp {
        <HomeKickPlayerRsp as ::protobuf::Message>::default_instance()
    }
}

impl HomeKickPlayerRsp {
    pub fn new() -> HomeKickPlayerRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_kick_all",
            |m: &HomeKickPlayerRsp| { &m.is_kick_all },
            |m: &mut HomeKickPlayerRsp| { &mut m.is_kick_all },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_uid",
            |m: &HomeKickPlayerRsp| { &m.target_uid },
            |m: &mut HomeKickPlayerRsp| { &mut m.target_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &HomeKickPlayerRsp| { &m.retcode },
            |m: &mut HomeKickPlayerRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeKickPlayerRsp>(
            "HomeKickPlayerRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeKickPlayerRsp {
    const NAME: &'static str = "HomeKickPlayerRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.is_kick_all = is.read_bool()?;
                },
                72 => {
                    self.target_uid = is.read_uint32()?;
                },
                64 => {
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
        if self.is_kick_all != false {
            my_size += 1 + 1;
        }
        if self.target_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.target_uid);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_kick_all != false {
            os.write_bool(10, self.is_kick_all)?;
        }
        if self.target_uid != 0 {
            os.write_uint32(9, self.target_uid)?;
        }
        if self.retcode != 0 {
            os.write_int32(8, self.retcode)?;
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

    fn new() -> HomeKickPlayerRsp {
        HomeKickPlayerRsp::new()
    }

    fn clear(&mut self) {
        self.is_kick_all = false;
        self.target_uid = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeKickPlayerRsp {
        static instance: HomeKickPlayerRsp = HomeKickPlayerRsp {
            is_kick_all: false,
            target_uid: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeKickPlayerRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeKickPlayerRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeKickPlayerRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeKickPlayerRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17HomeKickPlayerRsp.proto\"l\n\x11HomeKickPlayerRsp\x12\x1e\n\x0bis_\
    kick_all\x18\n\x20\x01(\x08R\tisKickAll\x12\x1d\n\ntarget_uid\x18\t\x20\
    \x01(\rR\ttargetUid\x12\x18\n\x07retcode\x18\x08\x20\x01(\x05R\x07retcod\
    eB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(HomeKickPlayerRsp::generated_message_descriptor_data());
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
