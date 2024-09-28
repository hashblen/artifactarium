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

//! Generated file from `ForgeFormulaDataNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:ForgeFormulaDataNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ForgeFormulaDataNotify {
    // message fields
    // @@protoc_insertion_point(field:ForgeFormulaDataNotify.is_locked)
    pub is_locked: bool,
    // @@protoc_insertion_point(field:ForgeFormulaDataNotify.forge_id)
    pub forge_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ForgeFormulaDataNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ForgeFormulaDataNotify {
    fn default() -> &'a ForgeFormulaDataNotify {
        <ForgeFormulaDataNotify as ::protobuf::Message>::default_instance()
    }
}

impl ForgeFormulaDataNotify {
    pub fn new() -> ForgeFormulaDataNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_locked",
            |m: &ForgeFormulaDataNotify| { &m.is_locked },
            |m: &mut ForgeFormulaDataNotify| { &mut m.is_locked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "forge_id",
            |m: &ForgeFormulaDataNotify| { &m.forge_id },
            |m: &mut ForgeFormulaDataNotify| { &mut m.forge_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ForgeFormulaDataNotify>(
            "ForgeFormulaDataNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ForgeFormulaDataNotify {
    const NAME: &'static str = "ForgeFormulaDataNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.is_locked = is.read_bool()?;
                },
                56 => {
                    self.forge_id = is.read_uint32()?;
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
        if self.is_locked != false {
            my_size += 1 + 1;
        }
        if self.forge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.forge_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_locked != false {
            os.write_bool(15, self.is_locked)?;
        }
        if self.forge_id != 0 {
            os.write_uint32(7, self.forge_id)?;
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

    fn new() -> ForgeFormulaDataNotify {
        ForgeFormulaDataNotify::new()
    }

    fn clear(&mut self) {
        self.is_locked = false;
        self.forge_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ForgeFormulaDataNotify {
        static instance: ForgeFormulaDataNotify = ForgeFormulaDataNotify {
            is_locked: false,
            forge_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ForgeFormulaDataNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ForgeFormulaDataNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ForgeFormulaDataNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ForgeFormulaDataNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cForgeFormulaDataNotify.proto\"P\n\x16ForgeFormulaDataNotify\x12\
    \x1b\n\tis_locked\x18\x0f\x20\x01(\x08R\x08isLocked\x12\x19\n\x08forge_i\
    d\x18\x07\x20\x01(\rR\x07forgeIdB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ForgeFormulaDataNotify::generated_message_descriptor_data());
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
