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

//! Generated file from `TryEnterHomeReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:TryEnterHomeReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TryEnterHomeReq {
    // message fields
    // @@protoc_insertion_point(field:TryEnterHomeReq.target_uid)
    pub target_uid: u32,
    // @@protoc_insertion_point(field:TryEnterHomeReq.is_transfer_to_main_house_point)
    pub is_transfer_to_main_house_point: bool,
    // @@protoc_insertion_point(field:TryEnterHomeReq.is_transfer_to_safe_point)
    pub is_transfer_to_safe_point: bool,
    // @@protoc_insertion_point(field:TryEnterHomeReq.target_point)
    pub target_point: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TryEnterHomeReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TryEnterHomeReq {
    fn default() -> &'a TryEnterHomeReq {
        <TryEnterHomeReq as ::protobuf::Message>::default_instance()
    }
}

impl TryEnterHomeReq {
    pub fn new() -> TryEnterHomeReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_uid",
            |m: &TryEnterHomeReq| { &m.target_uid },
            |m: &mut TryEnterHomeReq| { &mut m.target_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_transfer_to_main_house_point",
            |m: &TryEnterHomeReq| { &m.is_transfer_to_main_house_point },
            |m: &mut TryEnterHomeReq| { &mut m.is_transfer_to_main_house_point },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_transfer_to_safe_point",
            |m: &TryEnterHomeReq| { &m.is_transfer_to_safe_point },
            |m: &mut TryEnterHomeReq| { &mut m.is_transfer_to_safe_point },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_point",
            |m: &TryEnterHomeReq| { &m.target_point },
            |m: &mut TryEnterHomeReq| { &mut m.target_point },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TryEnterHomeReq>(
            "TryEnterHomeReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TryEnterHomeReq {
    const NAME: &'static str = "TryEnterHomeReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.target_uid = is.read_uint32()?;
                },
                8 => {
                    self.is_transfer_to_main_house_point = is.read_bool()?;
                },
                88 => {
                    self.is_transfer_to_safe_point = is.read_bool()?;
                },
                120 => {
                    self.target_point = is.read_uint32()?;
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
        if self.target_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.target_uid);
        }
        if self.is_transfer_to_main_house_point != false {
            my_size += 1 + 1;
        }
        if self.is_transfer_to_safe_point != false {
            my_size += 1 + 1;
        }
        if self.target_point != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.target_point);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.target_uid != 0 {
            os.write_uint32(7, self.target_uid)?;
        }
        if self.is_transfer_to_main_house_point != false {
            os.write_bool(1, self.is_transfer_to_main_house_point)?;
        }
        if self.is_transfer_to_safe_point != false {
            os.write_bool(11, self.is_transfer_to_safe_point)?;
        }
        if self.target_point != 0 {
            os.write_uint32(15, self.target_point)?;
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

    fn new() -> TryEnterHomeReq {
        TryEnterHomeReq::new()
    }

    fn clear(&mut self) {
        self.target_uid = 0;
        self.is_transfer_to_main_house_point = false;
        self.is_transfer_to_safe_point = false;
        self.target_point = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TryEnterHomeReq {
        static instance: TryEnterHomeReq = TryEnterHomeReq {
            target_uid: 0,
            is_transfer_to_main_house_point: false,
            is_transfer_to_safe_point: false,
            target_point: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TryEnterHomeReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TryEnterHomeReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TryEnterHomeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TryEnterHomeReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15TryEnterHomeReq.proto\"\xd2\x01\n\x0fTryEnterHomeReq\x12\x1d\n\nta\
    rget_uid\x18\x07\x20\x01(\rR\ttargetUid\x12C\n\x1fis_transfer_to_main_ho\
    use_point\x18\x01\x20\x01(\x08R\x1aisTransferToMainHousePoint\x128\n\x19\
    is_transfer_to_safe_point\x18\x0b\x20\x01(\x08R\x15isTransferToSafePoint\
    \x12!\n\x0ctarget_point\x18\x0f\x20\x01(\rR\x0btargetPointB\x1b\n\x19emu\
    .grasscutter.net.protob\x06proto3\
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
            messages.push(TryEnterHomeReq::generated_message_descriptor_data());
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
