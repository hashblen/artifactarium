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

//! Generated file from `HitCollision.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:HitCollision)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HitCollision {
    // message fields
    // @@protoc_insertion_point(field:HitCollision.DDLIENJBAKB)
    pub DDLIENJBAKB: f32,
    // @@protoc_insertion_point(field:HitCollision.hit_point)
    pub hit_point: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:HitCollision.hit_collider_type)
    pub hit_collider_type: ::protobuf::EnumOrUnknown<super::HitColliderType::HitColliderType>,
    // @@protoc_insertion_point(field:HitCollision.GGHMAHBOMKE)
    pub GGHMAHBOMKE: f32,
    // @@protoc_insertion_point(field:HitCollision.EDOEECEACHH)
    pub EDOEECEACHH: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:HitCollision.hit_box_index)
    pub hit_box_index: i32,
    // special fields
    // @@protoc_insertion_point(special_field:HitCollision.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HitCollision {
    fn default() -> &'a HitCollision {
        <HitCollision as ::protobuf::Message>::default_instance()
    }
}

impl HitCollision {
    pub fn new() -> HitCollision {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDLIENJBAKB",
            |m: &HitCollision| { &m.DDLIENJBAKB },
            |m: &mut HitCollision| { &mut m.DDLIENJBAKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "hit_point",
            |m: &HitCollision| { &m.hit_point },
            |m: &mut HitCollision| { &mut m.hit_point },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hit_collider_type",
            |m: &HitCollision| { &m.hit_collider_type },
            |m: &mut HitCollision| { &mut m.hit_collider_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GGHMAHBOMKE",
            |m: &HitCollision| { &m.GGHMAHBOMKE },
            |m: &mut HitCollision| { &mut m.GGHMAHBOMKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "EDOEECEACHH",
            |m: &HitCollision| { &m.EDOEECEACHH },
            |m: &mut HitCollision| { &mut m.EDOEECEACHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hit_box_index",
            |m: &HitCollision| { &m.hit_box_index },
            |m: &mut HitCollision| { &mut m.hit_box_index },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HitCollision>(
            "HitCollision",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HitCollision {
    const NAME: &'static str = "HitCollision";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                29 => {
                    self.DDLIENJBAKB = is.read_float()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.hit_point)?;
                },
                48 => {
                    self.hit_collider_type = is.read_enum_or_unknown()?;
                },
                61 => {
                    self.GGHMAHBOMKE = is.read_float()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EDOEECEACHH)?;
                },
                96 => {
                    self.hit_box_index = is.read_int32()?;
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
        if self.DDLIENJBAKB != 0. {
            my_size += 1 + 4;
        }
        if let Some(v) = self.hit_point.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.hit_collider_type != ::protobuf::EnumOrUnknown::new(super::HitColliderType::HitColliderType::HIT_COLLIDER_INVALID) {
            my_size += ::protobuf::rt::int32_size(6, self.hit_collider_type.value());
        }
        if self.GGHMAHBOMKE != 0. {
            my_size += 1 + 4;
        }
        if let Some(v) = self.EDOEECEACHH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.hit_box_index != 0 {
            my_size += ::protobuf::rt::int32_size(12, self.hit_box_index);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DDLIENJBAKB != 0. {
            os.write_float(3, self.DDLIENJBAKB)?;
        }
        if let Some(v) = self.hit_point.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.hit_collider_type != ::protobuf::EnumOrUnknown::new(super::HitColliderType::HitColliderType::HIT_COLLIDER_INVALID) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.hit_collider_type))?;
        }
        if self.GGHMAHBOMKE != 0. {
            os.write_float(7, self.GGHMAHBOMKE)?;
        }
        if let Some(v) = self.EDOEECEACHH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.hit_box_index != 0 {
            os.write_int32(12, self.hit_box_index)?;
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

    fn new() -> HitCollision {
        HitCollision::new()
    }

    fn clear(&mut self) {
        self.DDLIENJBAKB = 0.;
        self.hit_point.clear();
        self.hit_collider_type = ::protobuf::EnumOrUnknown::new(super::HitColliderType::HitColliderType::HIT_COLLIDER_INVALID);
        self.GGHMAHBOMKE = 0.;
        self.EDOEECEACHH.clear();
        self.hit_box_index = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HitCollision {
        static instance: HitCollision = HitCollision {
            DDLIENJBAKB: 0.,
            hit_point: ::protobuf::MessageField::none(),
            hit_collider_type: ::protobuf::EnumOrUnknown::from_i32(0),
            GGHMAHBOMKE: 0.,
            EDOEECEACHH: ::protobuf::MessageField::none(),
            hit_box_index: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HitCollision {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HitCollision").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HitCollision {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HitCollision {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12HitCollision.proto\x1a\x0cVector.proto\x1a\x15HitColliderType.prot\
    o\"\x85\x02\n\x0cHitCollision\x12\x20\n\x0bDDLIENJBAKB\x18\x03\x20\x01(\
    \x02R\x0bDDLIENJBAKB\x12$\n\thit_point\x18\x05\x20\x01(\x0b2\x07.VectorR\
    \x08hitPoint\x12<\n\x11hit_collider_type\x18\x06\x20\x01(\x0e2\x10.HitCo\
    lliderTypeR\x0fhitColliderType\x12\x20\n\x0bGGHMAHBOMKE\x18\x07\x20\x01(\
    \x02R\x0bGGHMAHBOMKE\x12)\n\x0bEDOEECEACHH\x18\n\x20\x01(\x0b2\x07.Vecto\
    rR\x0bEDOEECEACHH\x12\"\n\rhit_box_index\x18\x0c\x20\x01(\x05R\x0bhitBox\
    IndexB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::Vector::file_descriptor().clone());
            deps.push(super::HitColliderType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HitCollision::generated_message_descriptor_data());
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
