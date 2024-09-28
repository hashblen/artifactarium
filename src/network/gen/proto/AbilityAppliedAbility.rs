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

//! Generated file from `AbilityAppliedAbility.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:AbilityAppliedAbility)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AbilityAppliedAbility {
    // message fields
    // @@protoc_insertion_point(field:AbilityAppliedAbility.ability_name)
    pub ability_name: ::protobuf::MessageField<super::AbilityString::AbilityString>,
    // @@protoc_insertion_point(field:AbilityAppliedAbility.ability_override)
    pub ability_override: ::protobuf::MessageField<super::AbilityString::AbilityString>,
    // @@protoc_insertion_point(field:AbilityAppliedAbility.override_map)
    pub override_map: ::std::vec::Vec<super::AbilityScalarValueEntry::AbilityScalarValueEntry>,
    // @@protoc_insertion_point(field:AbilityAppliedAbility.instanced_ability_id)
    pub instanced_ability_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AbilityAppliedAbility.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AbilityAppliedAbility {
    fn default() -> &'a AbilityAppliedAbility {
        <AbilityAppliedAbility as ::protobuf::Message>::default_instance()
    }
}

impl AbilityAppliedAbility {
    pub fn new() -> AbilityAppliedAbility {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilityString::AbilityString>(
            "ability_name",
            |m: &AbilityAppliedAbility| { &m.ability_name },
            |m: &mut AbilityAppliedAbility| { &mut m.ability_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilityString::AbilityString>(
            "ability_override",
            |m: &AbilityAppliedAbility| { &m.ability_override },
            |m: &mut AbilityAppliedAbility| { &mut m.ability_override },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "override_map",
            |m: &AbilityAppliedAbility| { &m.override_map },
            |m: &mut AbilityAppliedAbility| { &mut m.override_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "instanced_ability_id",
            |m: &AbilityAppliedAbility| { &m.instanced_ability_id },
            |m: &mut AbilityAppliedAbility| { &mut m.instanced_ability_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AbilityAppliedAbility>(
            "AbilityAppliedAbility",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AbilityAppliedAbility {
    const NAME: &'static str = "AbilityAppliedAbility";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ability_name)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ability_override)?;
                },
                26 => {
                    self.override_map.push(is.read_message()?);
                },
                32 => {
                    self.instanced_ability_id = is.read_uint32()?;
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
        if let Some(v) = self.ability_name.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ability_override.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.override_map {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.instanced_ability_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.instanced_ability_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ability_name.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.ability_override.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.override_map {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.instanced_ability_id != 0 {
            os.write_uint32(4, self.instanced_ability_id)?;
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

    fn new() -> AbilityAppliedAbility {
        AbilityAppliedAbility::new()
    }

    fn clear(&mut self) {
        self.ability_name.clear();
        self.ability_override.clear();
        self.override_map.clear();
        self.instanced_ability_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AbilityAppliedAbility {
        static instance: AbilityAppliedAbility = AbilityAppliedAbility {
            ability_name: ::protobuf::MessageField::none(),
            ability_override: ::protobuf::MessageField::none(),
            override_map: ::std::vec::Vec::new(),
            instanced_ability_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AbilityAppliedAbility {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AbilityAppliedAbility").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AbilityAppliedAbility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AbilityAppliedAbility {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bAbilityAppliedAbility.proto\x1a\x13AbilityString.proto\x1a\x1dAbil\
    ityScalarValueEntry.proto\"\xf4\x01\n\x15AbilityAppliedAbility\x121\n\
    \x0cability_name\x18\x01\x20\x01(\x0b2\x0e.AbilityStringR\x0babilityName\
    \x129\n\x10ability_override\x18\x02\x20\x01(\x0b2\x0e.AbilityStringR\x0f\
    abilityOverride\x12;\n\x0coverride_map\x18\x03\x20\x03(\x0b2\x18.Ability\
    ScalarValueEntryR\x0boverrideMap\x120\n\x14instanced_ability_id\x18\x04\
    \x20\x01(\rR\x12instancedAbilityIdB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::AbilityString::file_descriptor().clone());
            deps.push(super::AbilityScalarValueEntry::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AbilityAppliedAbility::generated_message_descriptor_data());
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
