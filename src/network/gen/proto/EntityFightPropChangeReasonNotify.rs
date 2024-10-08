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

//! Generated file from `EntityFightPropChangeReasonNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:EntityFightPropChangeReasonNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EntityFightPropChangeReasonNotify {
    // message fields
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.detail_info)
    pub detail_info: ::protobuf::MessageField<super::DetailInfo::DetailInfo>,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.reason)
    pub reason: ::protobuf::EnumOrUnknown<super::PropChangeReason::PropChangeReason>,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.change_hp_debts_reason)
    pub change_hp_debts_reason: ::protobuf::EnumOrUnknown<super::ChangeHpDebtsReason::ChangeHpDebtsReason>,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.param_list)
    pub param_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.prop_delta)
    pub prop_delta: f32,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.prop_type)
    pub prop_type: u32,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.NNFONEOHGHE)
    pub NNFONEOHGHE: f32,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.change_hp_reason)
    pub change_hp_reason: ::protobuf::EnumOrUnknown<super::ChangeHpReason::ChangeHpReason>,
    // @@protoc_insertion_point(field:EntityFightPropChangeReasonNotify.change_energy_reason)
    pub change_energy_reason: ::protobuf::EnumOrUnknown<super::ChangeEnergyReason::ChangeEnergyReason>,
    // special fields
    // @@protoc_insertion_point(special_field:EntityFightPropChangeReasonNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EntityFightPropChangeReasonNotify {
    fn default() -> &'a EntityFightPropChangeReasonNotify {
        <EntityFightPropChangeReasonNotify as ::protobuf::Message>::default_instance()
    }
}

impl EntityFightPropChangeReasonNotify {
    pub fn new() -> EntityFightPropChangeReasonNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DetailInfo::DetailInfo>(
            "detail_info",
            |m: &EntityFightPropChangeReasonNotify| { &m.detail_info },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.detail_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &EntityFightPropChangeReasonNotify| { &m.entity_id },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reason",
            |m: &EntityFightPropChangeReasonNotify| { &m.reason },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "change_hp_debts_reason",
            |m: &EntityFightPropChangeReasonNotify| { &m.change_hp_debts_reason },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.change_hp_debts_reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "param_list",
            |m: &EntityFightPropChangeReasonNotify| { &m.param_list },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.param_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prop_delta",
            |m: &EntityFightPropChangeReasonNotify| { &m.prop_delta },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.prop_delta },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prop_type",
            |m: &EntityFightPropChangeReasonNotify| { &m.prop_type },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.prop_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNFONEOHGHE",
            |m: &EntityFightPropChangeReasonNotify| { &m.NNFONEOHGHE },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.NNFONEOHGHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "change_hp_reason",
            |m: &EntityFightPropChangeReasonNotify| { &m.change_hp_reason },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.change_hp_reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "change_energy_reason",
            |m: &EntityFightPropChangeReasonNotify| { &m.change_energy_reason },
            |m: &mut EntityFightPropChangeReasonNotify| { &mut m.change_energy_reason },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EntityFightPropChangeReasonNotify>(
            "EntityFightPropChangeReasonNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EntityFightPropChangeReasonNotify {
    const NAME: &'static str = "EntityFightPropChangeReasonNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.detail_info)?;
                },
                16 => {
                    self.entity_id = is.read_uint32()?;
                },
                24 => {
                    self.reason = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.change_hp_debts_reason = is.read_enum_or_unknown()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.param_list)?;
                },
                64 => {
                    self.param_list.push(is.read_uint32()?);
                },
                85 => {
                    self.prop_delta = is.read_float()?;
                },
                88 => {
                    self.prop_type = is.read_uint32()?;
                },
                101 => {
                    self.NNFONEOHGHE = is.read_float()?;
                },
                104 => {
                    self.change_hp_reason = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.change_energy_reason = is.read_enum_or_unknown()?;
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
        if let Some(v) = self.detail_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.entity_id);
        }
        if self.reason != ::protobuf::EnumOrUnknown::new(super::PropChangeReason::PropChangeReason::PROP_CHANGE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.reason.value());
        }
        if self.change_hp_debts_reason != ::protobuf::EnumOrUnknown::new(super::ChangeHpDebtsReason::ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.change_hp_debts_reason.value());
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(8, &self.param_list);
        if self.prop_delta != 0. {
            my_size += 1 + 4;
        }
        if self.prop_type != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.prop_type);
        }
        if self.NNFONEOHGHE != 0. {
            my_size += 1 + 4;
        }
        if self.change_hp_reason != ::protobuf::EnumOrUnknown::new(super::ChangeHpReason::ChangeHpReason::CHANGE_HP_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.change_hp_reason.value());
        }
        if self.change_energy_reason != ::protobuf::EnumOrUnknown::new(super::ChangeEnergyReason::ChangeEnergyReason::CHANGE_ENERGY_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.change_energy_reason.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.detail_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.entity_id != 0 {
            os.write_uint32(2, self.entity_id)?;
        }
        if self.reason != ::protobuf::EnumOrUnknown::new(super::PropChangeReason::PropChangeReason::PROP_CHANGE_REASON_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.reason))?;
        }
        if self.change_hp_debts_reason != ::protobuf::EnumOrUnknown::new(super::ChangeHpDebtsReason::ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.change_hp_debts_reason))?;
        }
        os.write_repeated_packed_uint32(8, &self.param_list)?;
        if self.prop_delta != 0. {
            os.write_float(10, self.prop_delta)?;
        }
        if self.prop_type != 0 {
            os.write_uint32(11, self.prop_type)?;
        }
        if self.NNFONEOHGHE != 0. {
            os.write_float(12, self.NNFONEOHGHE)?;
        }
        if self.change_hp_reason != ::protobuf::EnumOrUnknown::new(super::ChangeHpReason::ChangeHpReason::CHANGE_HP_REASON_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.change_hp_reason))?;
        }
        if self.change_energy_reason != ::protobuf::EnumOrUnknown::new(super::ChangeEnergyReason::ChangeEnergyReason::CHANGE_ENERGY_REASON_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.change_energy_reason))?;
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

    fn new() -> EntityFightPropChangeReasonNotify {
        EntityFightPropChangeReasonNotify::new()
    }

    fn clear(&mut self) {
        self.detail_info.clear();
        self.entity_id = 0;
        self.reason = ::protobuf::EnumOrUnknown::new(super::PropChangeReason::PropChangeReason::PROP_CHANGE_REASON_NONE);
        self.change_hp_debts_reason = ::protobuf::EnumOrUnknown::new(super::ChangeHpDebtsReason::ChangeHpDebtsReason::CHANGE_HP_DEBTS_NONE);
        self.param_list.clear();
        self.prop_delta = 0.;
        self.prop_type = 0;
        self.NNFONEOHGHE = 0.;
        self.change_hp_reason = ::protobuf::EnumOrUnknown::new(super::ChangeHpReason::ChangeHpReason::CHANGE_HP_REASON_NONE);
        self.change_energy_reason = ::protobuf::EnumOrUnknown::new(super::ChangeEnergyReason::ChangeEnergyReason::CHANGE_ENERGY_REASON_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EntityFightPropChangeReasonNotify {
        static instance: EntityFightPropChangeReasonNotify = EntityFightPropChangeReasonNotify {
            detail_info: ::protobuf::MessageField::none(),
            entity_id: 0,
            reason: ::protobuf::EnumOrUnknown::from_i32(0),
            change_hp_debts_reason: ::protobuf::EnumOrUnknown::from_i32(0),
            param_list: ::std::vec::Vec::new(),
            prop_delta: 0.,
            prop_type: 0,
            NNFONEOHGHE: 0.,
            change_hp_reason: ::protobuf::EnumOrUnknown::from_i32(0),
            change_energy_reason: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EntityFightPropChangeReasonNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EntityFightPropChangeReasonNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EntityFightPropChangeReasonNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EntityFightPropChangeReasonNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'EntityFightPropChangeReasonNotify.proto\x1a\x10DetailInfo.proto\x1a\
    \x16PropChangeReason.proto\x1a\x19ChangeHpDebtsReason.proto\x1a\x14Chang\
    eHpReason.proto\x1a\x18ChangeEnergyReason.proto\"\xe3\x03\n!EntityFightP\
    ropChangeReasonNotify\x12,\n\x0bdetail_info\x18\x01\x20\x01(\x0b2\x0b.De\
    tailInfoR\ndetailInfo\x12\x1b\n\tentity_id\x18\x02\x20\x01(\rR\x08entity\
    Id\x12)\n\x06reason\x18\x03\x20\x01(\x0e2\x11.PropChangeReasonR\x06reaso\
    n\x12I\n\x16change_hp_debts_reason\x18\x07\x20\x01(\x0e2\x14.ChangeHpDeb\
    tsReasonR\x13changeHpDebtsReason\x12\x1d\n\nparam_list\x18\x08\x20\x03(\
    \rR\tparamList\x12\x1d\n\nprop_delta\x18\n\x20\x01(\x02R\tpropDelta\x12\
    \x1b\n\tprop_type\x18\x0b\x20\x01(\rR\x08propType\x12\x20\n\x0bNNFONEOHG\
    HE\x18\x0c\x20\x01(\x02R\x0bNNFONEOHGHE\x129\n\x10change_hp_reason\x18\r\
    \x20\x01(\x0e2\x0f.ChangeHpReasonR\x0echangeHpReason\x12E\n\x14change_en\
    ergy_reason\x18\x0f\x20\x01(\x0e2\x13.ChangeEnergyReasonR\x12changeEnerg\
    yReasonB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::DetailInfo::file_descriptor().clone());
            deps.push(super::PropChangeReason::file_descriptor().clone());
            deps.push(super::ChangeHpDebtsReason::file_descriptor().clone());
            deps.push(super::ChangeHpReason::file_descriptor().clone());
            deps.push(super::ChangeEnergyReason::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EntityFightPropChangeReasonNotify::generated_message_descriptor_data());
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
