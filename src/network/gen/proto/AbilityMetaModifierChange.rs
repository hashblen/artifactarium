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

//! Generated file from `AbilityMetaModifierChange.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:AbilityMetaModifierChange)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AbilityMetaModifierChange {
    // message fields
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.server_buff_uid)
    pub server_buff_uid: u32,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.parent_ability_override)
    pub parent_ability_override: ::protobuf::MessageField<super::AbilityString::AbilityString>,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.properties)
    pub properties: ::std::vec::Vec<super::ModifierProperty::ModifierProperty>,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.apply_entity_id)
    pub apply_entity_id: u32,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.attached_instanced_modifier)
    pub attached_instanced_modifier: ::protobuf::MessageField<super::AbilityAttachedModifier::AbilityAttachedModifier>,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.parent_ability_name)
    pub parent_ability_name: ::protobuf::MessageField<super::AbilityString::AbilityString>,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.modifier_local_id)
    pub modifier_local_id: i32,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.is_attached_parent_ability)
    pub is_attached_parent_ability: bool,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.EFONMKFIJNA)
    pub EFONMKFIJNA: bool,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.MAPJDCOAIMG)
    pub MAPJDCOAIMG: bool,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.KKAAMMJBABH)
    pub KKAAMMJBABH: f32,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.KKFHAIPCCFA)
    pub KKFHAIPCCFA: u64,
    // @@protoc_insertion_point(field:AbilityMetaModifierChange.action)
    pub action: ::protobuf::EnumOrUnknown<super::ModifierAction::ModifierAction>,
    // special fields
    // @@protoc_insertion_point(special_field:AbilityMetaModifierChange.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AbilityMetaModifierChange {
    fn default() -> &'a AbilityMetaModifierChange {
        <AbilityMetaModifierChange as ::protobuf::Message>::default_instance()
    }
}

impl AbilityMetaModifierChange {
    pub fn new() -> AbilityMetaModifierChange {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "server_buff_uid",
            |m: &AbilityMetaModifierChange| { &m.server_buff_uid },
            |m: &mut AbilityMetaModifierChange| { &mut m.server_buff_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilityString::AbilityString>(
            "parent_ability_override",
            |m: &AbilityMetaModifierChange| { &m.parent_ability_override },
            |m: &mut AbilityMetaModifierChange| { &mut m.parent_ability_override },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "properties",
            |m: &AbilityMetaModifierChange| { &m.properties },
            |m: &mut AbilityMetaModifierChange| { &mut m.properties },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "apply_entity_id",
            |m: &AbilityMetaModifierChange| { &m.apply_entity_id },
            |m: &mut AbilityMetaModifierChange| { &mut m.apply_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilityAttachedModifier::AbilityAttachedModifier>(
            "attached_instanced_modifier",
            |m: &AbilityMetaModifierChange| { &m.attached_instanced_modifier },
            |m: &mut AbilityMetaModifierChange| { &mut m.attached_instanced_modifier },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilityString::AbilityString>(
            "parent_ability_name",
            |m: &AbilityMetaModifierChange| { &m.parent_ability_name },
            |m: &mut AbilityMetaModifierChange| { &mut m.parent_ability_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "modifier_local_id",
            |m: &AbilityMetaModifierChange| { &m.modifier_local_id },
            |m: &mut AbilityMetaModifierChange| { &mut m.modifier_local_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_attached_parent_ability",
            |m: &AbilityMetaModifierChange| { &m.is_attached_parent_ability },
            |m: &mut AbilityMetaModifierChange| { &mut m.is_attached_parent_ability },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFONMKFIJNA",
            |m: &AbilityMetaModifierChange| { &m.EFONMKFIJNA },
            |m: &mut AbilityMetaModifierChange| { &mut m.EFONMKFIJNA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MAPJDCOAIMG",
            |m: &AbilityMetaModifierChange| { &m.MAPJDCOAIMG },
            |m: &mut AbilityMetaModifierChange| { &mut m.MAPJDCOAIMG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKAAMMJBABH",
            |m: &AbilityMetaModifierChange| { &m.KKAAMMJBABH },
            |m: &mut AbilityMetaModifierChange| { &mut m.KKAAMMJBABH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKFHAIPCCFA",
            |m: &AbilityMetaModifierChange| { &m.KKFHAIPCCFA },
            |m: &mut AbilityMetaModifierChange| { &mut m.KKFHAIPCCFA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "action",
            |m: &AbilityMetaModifierChange| { &m.action },
            |m: &mut AbilityMetaModifierChange| { &mut m.action },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AbilityMetaModifierChange>(
            "AbilityMetaModifierChange",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AbilityMetaModifierChange {
    const NAME: &'static str = "AbilityMetaModifierChange";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.server_buff_uid = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.parent_ability_override)?;
                },
                34 => {
                    self.properties.push(is.read_message()?);
                },
                40 => {
                    self.apply_entity_id = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.attached_instanced_modifier)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.parent_ability_name)?;
                },
                64 => {
                    self.modifier_local_id = is.read_int32()?;
                },
                80 => {
                    self.is_attached_parent_ability = is.read_bool()?;
                },
                88 => {
                    self.EFONMKFIJNA = is.read_bool()?;
                },
                96 => {
                    self.MAPJDCOAIMG = is.read_bool()?;
                },
                109 => {
                    self.KKAAMMJBABH = is.read_float()?;
                },
                112 => {
                    self.KKFHAIPCCFA = is.read_uint64()?;
                },
                120 => {
                    self.action = is.read_enum_or_unknown()?;
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
        if self.server_buff_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.server_buff_uid);
        }
        if let Some(v) = self.parent_ability_override.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.properties {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.apply_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.apply_entity_id);
        }
        if let Some(v) = self.attached_instanced_modifier.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.parent_ability_name.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.modifier_local_id != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.modifier_local_id);
        }
        if self.is_attached_parent_ability != false {
            my_size += 1 + 1;
        }
        if self.EFONMKFIJNA != false {
            my_size += 1 + 1;
        }
        if self.MAPJDCOAIMG != false {
            my_size += 1 + 1;
        }
        if self.KKAAMMJBABH != 0. {
            my_size += 1 + 4;
        }
        if self.KKFHAIPCCFA != 0 {
            my_size += ::protobuf::rt::uint64_size(14, self.KKFHAIPCCFA);
        }
        if self.action != ::protobuf::EnumOrUnknown::new(super::ModifierAction::ModifierAction::MODIFIER_ACTION_ADDED) {
            my_size += ::protobuf::rt::int32_size(15, self.action.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.server_buff_uid != 0 {
            os.write_uint32(2, self.server_buff_uid)?;
        }
        if let Some(v) = self.parent_ability_override.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        for v in &self.properties {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.apply_entity_id != 0 {
            os.write_uint32(5, self.apply_entity_id)?;
        }
        if let Some(v) = self.attached_instanced_modifier.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.parent_ability_name.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.modifier_local_id != 0 {
            os.write_int32(8, self.modifier_local_id)?;
        }
        if self.is_attached_parent_ability != false {
            os.write_bool(10, self.is_attached_parent_ability)?;
        }
        if self.EFONMKFIJNA != false {
            os.write_bool(11, self.EFONMKFIJNA)?;
        }
        if self.MAPJDCOAIMG != false {
            os.write_bool(12, self.MAPJDCOAIMG)?;
        }
        if self.KKAAMMJBABH != 0. {
            os.write_float(13, self.KKAAMMJBABH)?;
        }
        if self.KKFHAIPCCFA != 0 {
            os.write_uint64(14, self.KKFHAIPCCFA)?;
        }
        if self.action != ::protobuf::EnumOrUnknown::new(super::ModifierAction::ModifierAction::MODIFIER_ACTION_ADDED) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.action))?;
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

    fn new() -> AbilityMetaModifierChange {
        AbilityMetaModifierChange::new()
    }

    fn clear(&mut self) {
        self.server_buff_uid = 0;
        self.parent_ability_override.clear();
        self.properties.clear();
        self.apply_entity_id = 0;
        self.attached_instanced_modifier.clear();
        self.parent_ability_name.clear();
        self.modifier_local_id = 0;
        self.is_attached_parent_ability = false;
        self.EFONMKFIJNA = false;
        self.MAPJDCOAIMG = false;
        self.KKAAMMJBABH = 0.;
        self.KKFHAIPCCFA = 0;
        self.action = ::protobuf::EnumOrUnknown::new(super::ModifierAction::ModifierAction::MODIFIER_ACTION_ADDED);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AbilityMetaModifierChange {
        static instance: AbilityMetaModifierChange = AbilityMetaModifierChange {
            server_buff_uid: 0,
            parent_ability_override: ::protobuf::MessageField::none(),
            properties: ::std::vec::Vec::new(),
            apply_entity_id: 0,
            attached_instanced_modifier: ::protobuf::MessageField::none(),
            parent_ability_name: ::protobuf::MessageField::none(),
            modifier_local_id: 0,
            is_attached_parent_ability: false,
            EFONMKFIJNA: false,
            MAPJDCOAIMG: false,
            KKAAMMJBABH: 0.,
            KKFHAIPCCFA: 0,
            action: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AbilityMetaModifierChange {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AbilityMetaModifierChange").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AbilityMetaModifierChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AbilityMetaModifierChange {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fAbilityMetaModifierChange.proto\x1a\x13AbilityString.proto\x1a\x16\
    ModifierProperty.proto\x1a\x1dAbilityAttachedModifier.proto\x1a\x14Modif\
    ierAction.proto\"\x9a\x05\n\x19AbilityMetaModifierChange\x12&\n\x0fserve\
    r_buff_uid\x18\x02\x20\x01(\rR\rserverBuffUid\x12F\n\x17parent_ability_o\
    verride\x18\x03\x20\x01(\x0b2\x0e.AbilityStringR\x15parentAbilityOverrid\
    e\x121\n\nproperties\x18\x04\x20\x03(\x0b2\x11.ModifierPropertyR\nproper\
    ties\x12&\n\x0fapply_entity_id\x18\x05\x20\x01(\rR\rapplyEntityId\x12X\n\
    \x1battached_instanced_modifier\x18\x06\x20\x01(\x0b2\x18.AbilityAttache\
    dModifierR\x19attachedInstancedModifier\x12>\n\x13parent_ability_name\
    \x18\x07\x20\x01(\x0b2\x0e.AbilityStringR\x11parentAbilityName\x12*\n\
    \x11modifier_local_id\x18\x08\x20\x01(\x05R\x0fmodifierLocalId\x12;\n\
    \x1ais_attached_parent_ability\x18\n\x20\x01(\x08R\x17isAttachedParentAb\
    ility\x12\x20\n\x0bEFONMKFIJNA\x18\x0b\x20\x01(\x08R\x0bEFONMKFIJNA\x12\
    \x20\n\x0bMAPJDCOAIMG\x18\x0c\x20\x01(\x08R\x0bMAPJDCOAIMG\x12\x20\n\x0b\
    KKAAMMJBABH\x18\r\x20\x01(\x02R\x0bKKAAMMJBABH\x12\x20\n\x0bKKFHAIPCCFA\
    \x18\x0e\x20\x01(\x04R\x0bKKFHAIPCCFA\x12'\n\x06action\x18\x0f\x20\x01(\
    \x0e2\x0f.ModifierActionR\x06actionB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::AbilityString::file_descriptor().clone());
            deps.push(super::ModifierProperty::file_descriptor().clone());
            deps.push(super::AbilityAttachedModifier::file_descriptor().clone());
            deps.push(super::ModifierAction::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AbilityMetaModifierChange::generated_message_descriptor_data());
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
