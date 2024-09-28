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

//! Generated file from `MassiveEntityElementOpBatchNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:MassiveEntityElementOpBatchNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MassiveEntityElementOpBatchNotify {
    // message fields
    // @@protoc_insertion_point(field:MassiveEntityElementOpBatchNotify.op_idx)
    pub op_idx: u32,
    // @@protoc_insertion_point(field:MassiveEntityElementOpBatchNotify.user_id)
    pub user_id: u32,
    // @@protoc_insertion_point(field:MassiveEntityElementOpBatchNotify.attacker_id)
    pub attacker_id: u32,
    // @@protoc_insertion_point(field:MassiveEntityElementOpBatchNotify.entity_type)
    pub entity_type: i32,
    // @@protoc_insertion_point(field:MassiveEntityElementOpBatchNotify.attack_element_durability)
    pub attack_element_durability: f32,
    // @@protoc_insertion_point(field:MassiveEntityElementOpBatchNotify.uk1)
    pub uk1: i32,
    // @@protoc_insertion_point(field:MassiveEntityElementOpBatchNotify.uk2)
    pub uk2: i32,
    // message oneof groups
    pub shape: ::std::option::Option<massive_entity_element_op_batch_notify::Shape>,
    // special fields
    // @@protoc_insertion_point(special_field:MassiveEntityElementOpBatchNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MassiveEntityElementOpBatchNotify {
    fn default() -> &'a MassiveEntityElementOpBatchNotify {
        <MassiveEntityElementOpBatchNotify as ::protobuf::Message>::default_instance()
    }
}

impl MassiveEntityElementOpBatchNotify {
    pub fn new() -> MassiveEntityElementOpBatchNotify {
        ::std::default::Default::default()
    }

    // .ShapeSphere shape_sphere = 4;

    pub fn shape_sphere(&self) -> &super::ShapeSphere::ShapeSphere {
        match self.shape {
            ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(ref v)) => v,
            _ => <super::ShapeSphere::ShapeSphere as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_shape_sphere(&mut self) {
        self.shape = ::std::option::Option::None;
    }

    pub fn has_shape_sphere(&self) -> bool {
        match self.shape {
            ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_shape_sphere(&mut self, v: super::ShapeSphere::ShapeSphere) {
        self.shape = ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(v))
    }

    // Mutable pointer to the field.
    pub fn mut_shape_sphere(&mut self) -> &mut super::ShapeSphere::ShapeSphere {
        if let ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(_)) = self.shape {
        } else {
            self.shape = ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(super::ShapeSphere::ShapeSphere::new()));
        }
        match self.shape {
            ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_shape_sphere(&mut self) -> super::ShapeSphere::ShapeSphere {
        if self.has_shape_sphere() {
            match self.shape.take() {
                ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ShapeSphere::ShapeSphere::new()
        }
    }

    // .ShapeBox shape_box = 8;

    pub fn shape_box(&self) -> &super::ShapeBox::ShapeBox {
        match self.shape {
            ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(ref v)) => v,
            _ => <super::ShapeBox::ShapeBox as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_shape_box(&mut self) {
        self.shape = ::std::option::Option::None;
    }

    pub fn has_shape_box(&self) -> bool {
        match self.shape {
            ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_shape_box(&mut self, v: super::ShapeBox::ShapeBox) {
        self.shape = ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(v))
    }

    // Mutable pointer to the field.
    pub fn mut_shape_box(&mut self) -> &mut super::ShapeBox::ShapeBox {
        if let ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(_)) = self.shape {
        } else {
            self.shape = ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(super::ShapeBox::ShapeBox::new()));
        }
        match self.shape {
            ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_shape_box(&mut self) -> super::ShapeBox::ShapeBox {
        if self.has_shape_box() {
            match self.shape.take() {
                ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ShapeBox::ShapeBox::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "op_idx",
            |m: &MassiveEntityElementOpBatchNotify| { &m.op_idx },
            |m: &mut MassiveEntityElementOpBatchNotify| { &mut m.op_idx },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_id",
            |m: &MassiveEntityElementOpBatchNotify| { &m.user_id },
            |m: &mut MassiveEntityElementOpBatchNotify| { &mut m.user_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attacker_id",
            |m: &MassiveEntityElementOpBatchNotify| { &m.attacker_id },
            |m: &mut MassiveEntityElementOpBatchNotify| { &mut m.attacker_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_type",
            |m: &MassiveEntityElementOpBatchNotify| { &m.entity_type },
            |m: &mut MassiveEntityElementOpBatchNotify| { &mut m.entity_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attack_element_durability",
            |m: &MassiveEntityElementOpBatchNotify| { &m.attack_element_durability },
            |m: &mut MassiveEntityElementOpBatchNotify| { &mut m.attack_element_durability },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uk1",
            |m: &MassiveEntityElementOpBatchNotify| { &m.uk1 },
            |m: &mut MassiveEntityElementOpBatchNotify| { &mut m.uk1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uk2",
            |m: &MassiveEntityElementOpBatchNotify| { &m.uk2 },
            |m: &mut MassiveEntityElementOpBatchNotify| { &mut m.uk2 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ShapeSphere::ShapeSphere>(
            "shape_sphere",
            MassiveEntityElementOpBatchNotify::has_shape_sphere,
            MassiveEntityElementOpBatchNotify::shape_sphere,
            MassiveEntityElementOpBatchNotify::mut_shape_sphere,
            MassiveEntityElementOpBatchNotify::set_shape_sphere,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ShapeBox::ShapeBox>(
            "shape_box",
            MassiveEntityElementOpBatchNotify::has_shape_box,
            MassiveEntityElementOpBatchNotify::shape_box,
            MassiveEntityElementOpBatchNotify::mut_shape_box,
            MassiveEntityElementOpBatchNotify::set_shape_box,
        ));
        oneofs.push(massive_entity_element_op_batch_notify::Shape::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MassiveEntityElementOpBatchNotify>(
            "MassiveEntityElementOpBatchNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MassiveEntityElementOpBatchNotify {
    const NAME: &'static str = "MassiveEntityElementOpBatchNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.op_idx = is.read_uint32()?;
                },
                56 => {
                    self.user_id = is.read_uint32()?;
                },
                16 => {
                    self.attacker_id = is.read_uint32()?;
                },
                24 => {
                    self.entity_type = is.read_int32()?;
                },
                53 => {
                    self.attack_element_durability = is.read_float()?;
                },
                40 => {
                    self.uk1 = is.read_int32()?;
                },
                88 => {
                    self.uk2 = is.read_int32()?;
                },
                34 => {
                    self.shape = ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeSphere(is.read_message()?));
                },
                66 => {
                    self.shape = ::std::option::Option::Some(massive_entity_element_op_batch_notify::Shape::ShapeBox(is.read_message()?));
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
        if self.op_idx != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.op_idx);
        }
        if self.user_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.user_id);
        }
        if self.attacker_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.attacker_id);
        }
        if self.entity_type != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.entity_type);
        }
        if self.attack_element_durability != 0. {
            my_size += 1 + 4;
        }
        if self.uk1 != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.uk1);
        }
        if self.uk2 != 0 {
            my_size += ::protobuf::rt::int32_size(11, self.uk2);
        }
        if let ::std::option::Option::Some(ref v) = self.shape {
            match v {
                &massive_entity_element_op_batch_notify::Shape::ShapeSphere(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &massive_entity_element_op_batch_notify::Shape::ShapeBox(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.op_idx != 0 {
            os.write_uint32(12, self.op_idx)?;
        }
        if self.user_id != 0 {
            os.write_uint32(7, self.user_id)?;
        }
        if self.attacker_id != 0 {
            os.write_uint32(2, self.attacker_id)?;
        }
        if self.entity_type != 0 {
            os.write_int32(3, self.entity_type)?;
        }
        if self.attack_element_durability != 0. {
            os.write_float(6, self.attack_element_durability)?;
        }
        if self.uk1 != 0 {
            os.write_int32(5, self.uk1)?;
        }
        if self.uk2 != 0 {
            os.write_int32(11, self.uk2)?;
        }
        if let ::std::option::Option::Some(ref v) = self.shape {
            match v {
                &massive_entity_element_op_batch_notify::Shape::ShapeSphere(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &massive_entity_element_op_batch_notify::Shape::ShapeBox(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
            };
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

    fn new() -> MassiveEntityElementOpBatchNotify {
        MassiveEntityElementOpBatchNotify::new()
    }

    fn clear(&mut self) {
        self.op_idx = 0;
        self.user_id = 0;
        self.attacker_id = 0;
        self.entity_type = 0;
        self.attack_element_durability = 0.;
        self.uk1 = 0;
        self.uk2 = 0;
        self.shape = ::std::option::Option::None;
        self.shape = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MassiveEntityElementOpBatchNotify {
        static instance: MassiveEntityElementOpBatchNotify = MassiveEntityElementOpBatchNotify {
            op_idx: 0,
            user_id: 0,
            attacker_id: 0,
            entity_type: 0,
            attack_element_durability: 0.,
            uk1: 0,
            uk2: 0,
            shape: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MassiveEntityElementOpBatchNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MassiveEntityElementOpBatchNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MassiveEntityElementOpBatchNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MassiveEntityElementOpBatchNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `MassiveEntityElementOpBatchNotify`
pub mod massive_entity_element_op_batch_notify {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:MassiveEntityElementOpBatchNotify.shape)
    pub enum Shape {
        // @@protoc_insertion_point(oneof_field:MassiveEntityElementOpBatchNotify.shape_sphere)
        ShapeSphere(super::super::ShapeSphere::ShapeSphere),
        // @@protoc_insertion_point(oneof_field:MassiveEntityElementOpBatchNotify.shape_box)
        ShapeBox(super::super::ShapeBox::ShapeBox),
    }

    impl ::protobuf::Oneof for Shape {
    }

    impl ::protobuf::OneofFull for Shape {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::MassiveEntityElementOpBatchNotify as ::protobuf::MessageFull>::descriptor().oneof_by_name("shape").unwrap()).clone()
        }
    }

    impl Shape {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Shape>("shape")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'MassiveEntityElementOpBatchNotify.proto\x1a\x11ShapeSphere.proto\x1a\
    \x0eShapeBox.proto\"\xdb\x02\n!MassiveEntityElementOpBatchNotify\x12\x15\
    \n\x06op_idx\x18\x0c\x20\x01(\rR\x05opIdx\x12\x17\n\x07user_id\x18\x07\
    \x20\x01(\rR\x06userId\x12\x1f\n\x0battacker_id\x18\x02\x20\x01(\rR\natt\
    ackerId\x12\x1f\n\x0bentity_type\x18\x03\x20\x01(\x05R\nentityType\x12:\
    \n\x19attack_element_durability\x18\x06\x20\x01(\x02R\x17attackElementDu\
    rability\x12\x10\n\x03uk1\x18\x05\x20\x01(\x05R\x03uk1\x12\x10\n\x03uk2\
    \x18\x0b\x20\x01(\x05R\x03uk2\x121\n\x0cshape_sphere\x18\x04\x20\x01(\
    \x0b2\x0c.ShapeSphereH\0R\x0bshapeSphere\x12(\n\tshape_box\x18\x08\x20\
    \x01(\x0b2\t.ShapeBoxH\0R\x08shapeBoxB\x07\n\x05shapeB\x1b\n\x19emu.gras\
    scutter.net.protob\x06proto3\
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
            deps.push(super::ShapeSphere::file_descriptor().clone());
            deps.push(super::ShapeBox::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MassiveEntityElementOpBatchNotify::generated_message_descriptor_data());
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
