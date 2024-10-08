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

//! Generated file from `ClientAbilityChangeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:ClientAbilityChangeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientAbilityChangeNotify {
    // message fields
    // @@protoc_insertion_point(field:ClientAbilityChangeNotify.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:ClientAbilityChangeNotify.invokes)
    pub invokes: ::std::vec::Vec<super::AbilityInvokeEntry::AbilityInvokeEntry>,
    // @@protoc_insertion_point(field:ClientAbilityChangeNotify.is_init_hash)
    pub is_init_hash: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ClientAbilityChangeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientAbilityChangeNotify {
    fn default() -> &'a ClientAbilityChangeNotify {
        <ClientAbilityChangeNotify as ::protobuf::Message>::default_instance()
    }
}

impl ClientAbilityChangeNotify {
    pub fn new() -> ClientAbilityChangeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &ClientAbilityChangeNotify| { &m.entity_id },
            |m: &mut ClientAbilityChangeNotify| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "invokes",
            |m: &ClientAbilityChangeNotify| { &m.invokes },
            |m: &mut ClientAbilityChangeNotify| { &mut m.invokes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_init_hash",
            |m: &ClientAbilityChangeNotify| { &m.is_init_hash },
            |m: &mut ClientAbilityChangeNotify| { &mut m.is_init_hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientAbilityChangeNotify>(
            "ClientAbilityChangeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientAbilityChangeNotify {
    const NAME: &'static str = "ClientAbilityChangeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.entity_id = is.read_uint32()?;
                },
                26 => {
                    self.invokes.push(is.read_message()?);
                },
                32 => {
                    self.is_init_hash = is.read_bool()?;
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
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.entity_id);
        }
        for value in &self.invokes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.is_init_hash != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.entity_id != 0 {
            os.write_uint32(1, self.entity_id)?;
        }
        for v in &self.invokes {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.is_init_hash != false {
            os.write_bool(4, self.is_init_hash)?;
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

    fn new() -> ClientAbilityChangeNotify {
        ClientAbilityChangeNotify::new()
    }

    fn clear(&mut self) {
        self.entity_id = 0;
        self.invokes.clear();
        self.is_init_hash = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientAbilityChangeNotify {
        static instance: ClientAbilityChangeNotify = ClientAbilityChangeNotify {
            entity_id: 0,
            invokes: ::std::vec::Vec::new(),
            is_init_hash: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientAbilityChangeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientAbilityChangeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientAbilityChangeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientAbilityChangeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fClientAbilityChangeNotify.proto\x1a\x18AbilityInvokeEntry.proto\"\
    \x89\x01\n\x19ClientAbilityChangeNotify\x12\x1b\n\tentity_id\x18\x01\x20\
    \x01(\rR\x08entityId\x12-\n\x07invokes\x18\x03\x20\x03(\x0b2\x13.Ability\
    InvokeEntryR\x07invokes\x12\x20\n\x0cis_init_hash\x18\x04\x20\x01(\x08R\
    \nisInitHashB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::AbilityInvokeEntry::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClientAbilityChangeNotify::generated_message_descriptor_data());
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
