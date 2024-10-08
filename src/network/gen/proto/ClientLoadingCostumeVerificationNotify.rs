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

//! Generated file from `ClientLoadingCostumeVerificationNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:ClientLoadingCostumeVerificationNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientLoadingCostumeVerificationNotify {
    // message fields
    // @@protoc_insertion_point(field:ClientLoadingCostumeVerificationNotify.prefab_hash)
    pub prefab_hash: u64,
    // @@protoc_insertion_point(field:ClientLoadingCostumeVerificationNotify.costume_id)
    pub costume_id: u32,
    // @@protoc_insertion_point(field:ClientLoadingCostumeVerificationNotify.guid)
    pub guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:ClientLoadingCostumeVerificationNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientLoadingCostumeVerificationNotify {
    fn default() -> &'a ClientLoadingCostumeVerificationNotify {
        <ClientLoadingCostumeVerificationNotify as ::protobuf::Message>::default_instance()
    }
}

impl ClientLoadingCostumeVerificationNotify {
    pub fn new() -> ClientLoadingCostumeVerificationNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prefab_hash",
            |m: &ClientLoadingCostumeVerificationNotify| { &m.prefab_hash },
            |m: &mut ClientLoadingCostumeVerificationNotify| { &mut m.prefab_hash },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "costume_id",
            |m: &ClientLoadingCostumeVerificationNotify| { &m.costume_id },
            |m: &mut ClientLoadingCostumeVerificationNotify| { &mut m.costume_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "guid",
            |m: &ClientLoadingCostumeVerificationNotify| { &m.guid },
            |m: &mut ClientLoadingCostumeVerificationNotify| { &mut m.guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientLoadingCostumeVerificationNotify>(
            "ClientLoadingCostumeVerificationNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientLoadingCostumeVerificationNotify {
    const NAME: &'static str = "ClientLoadingCostumeVerificationNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.prefab_hash = is.read_uint64()?;
                },
                16 => {
                    self.costume_id = is.read_uint32()?;
                },
                80 => {
                    self.guid = is.read_uint64()?;
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
        if self.prefab_hash != 0 {
            my_size += ::protobuf::rt::uint64_size(15, self.prefab_hash);
        }
        if self.costume_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.costume_id);
        }
        if self.guid != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.prefab_hash != 0 {
            os.write_uint64(15, self.prefab_hash)?;
        }
        if self.costume_id != 0 {
            os.write_uint32(2, self.costume_id)?;
        }
        if self.guid != 0 {
            os.write_uint64(10, self.guid)?;
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

    fn new() -> ClientLoadingCostumeVerificationNotify {
        ClientLoadingCostumeVerificationNotify::new()
    }

    fn clear(&mut self) {
        self.prefab_hash = 0;
        self.costume_id = 0;
        self.guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientLoadingCostumeVerificationNotify {
        static instance: ClientLoadingCostumeVerificationNotify = ClientLoadingCostumeVerificationNotify {
            prefab_hash: 0,
            costume_id: 0,
            guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientLoadingCostumeVerificationNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientLoadingCostumeVerificationNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientLoadingCostumeVerificationNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientLoadingCostumeVerificationNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,ClientLoadingCostumeVerificationNotify.proto\"|\n&ClientLoadingCostum\
    eVerificationNotify\x12\x1f\n\x0bprefab_hash\x18\x0f\x20\x01(\x04R\npref\
    abHash\x12\x1d\n\ncostume_id\x18\x02\x20\x01(\rR\tcostumeId\x12\x12\n\
    \x04guid\x18\n\x20\x01(\x04R\x04guidB\x1b\n\x19emu.grasscutter.net.proto\
    b\x06proto3\
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
            messages.push(ClientLoadingCostumeVerificationNotify::generated_message_descriptor_data());
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
