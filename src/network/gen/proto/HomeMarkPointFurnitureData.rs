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

//! Generated file from `HomeMarkPointFurnitureData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:HomeMarkPointFurnitureData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeMarkPointFurnitureData {
    // message fields
    // @@protoc_insertion_point(field:HomeMarkPointFurnitureData.guid)
    pub guid: u32,
    // @@protoc_insertion_point(field:HomeMarkPointFurnitureData.furniture_id)
    pub furniture_id: u32,
    // @@protoc_insertion_point(field:HomeMarkPointFurnitureData.furniture_type)
    pub furniture_type: u32,
    // @@protoc_insertion_point(field:HomeMarkPointFurnitureData.pos)
    pub pos: ::protobuf::MessageField<super::Vector::Vector>,
    // message oneof groups
    pub extra: ::std::option::Option<home_mark_point_furniture_data::Extra>,
    // special fields
    // @@protoc_insertion_point(special_field:HomeMarkPointFurnitureData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeMarkPointFurnitureData {
    fn default() -> &'a HomeMarkPointFurnitureData {
        <HomeMarkPointFurnitureData as ::protobuf::Message>::default_instance()
    }
}

impl HomeMarkPointFurnitureData {
    pub fn new() -> HomeMarkPointFurnitureData {
        ::std::default::Default::default()
    }

    // .HomeMarkPointNPCData npc_data = 6;

    pub fn npc_data(&self) -> &super::HomeMarkPointNPCData::HomeMarkPointNPCData {
        match self.extra {
            ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(ref v)) => v,
            _ => <super::HomeMarkPointNPCData::HomeMarkPointNPCData as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_npc_data(&mut self) {
        self.extra = ::std::option::Option::None;
    }

    pub fn has_npc_data(&self) -> bool {
        match self.extra {
            ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_npc_data(&mut self, v: super::HomeMarkPointNPCData::HomeMarkPointNPCData) {
        self.extra = ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(v))
    }

    // Mutable pointer to the field.
    pub fn mut_npc_data(&mut self) -> &mut super::HomeMarkPointNPCData::HomeMarkPointNPCData {
        if let ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(_)) = self.extra {
        } else {
            self.extra = ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(super::HomeMarkPointNPCData::HomeMarkPointNPCData::new()));
        }
        match self.extra {
            ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_npc_data(&mut self) -> super::HomeMarkPointNPCData::HomeMarkPointNPCData {
        if self.has_npc_data() {
            match self.extra.take() {
                ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HomeMarkPointNPCData::HomeMarkPointNPCData::new()
        }
    }

    // .HomeMarkPointSuiteData suite_data = 7;

    pub fn suite_data(&self) -> &super::HomeMarkPointSuiteData::HomeMarkPointSuiteData {
        match self.extra {
            ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(ref v)) => v,
            _ => <super::HomeMarkPointSuiteData::HomeMarkPointSuiteData as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_suite_data(&mut self) {
        self.extra = ::std::option::Option::None;
    }

    pub fn has_suite_data(&self) -> bool {
        match self.extra {
            ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_suite_data(&mut self, v: super::HomeMarkPointSuiteData::HomeMarkPointSuiteData) {
        self.extra = ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(v))
    }

    // Mutable pointer to the field.
    pub fn mut_suite_data(&mut self) -> &mut super::HomeMarkPointSuiteData::HomeMarkPointSuiteData {
        if let ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(_)) = self.extra {
        } else {
            self.extra = ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(super::HomeMarkPointSuiteData::HomeMarkPointSuiteData::new()));
        }
        match self.extra {
            ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_suite_data(&mut self) -> super::HomeMarkPointSuiteData::HomeMarkPointSuiteData {
        if self.has_suite_data() {
            match self.extra.take() {
                ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HomeMarkPointSuiteData::HomeMarkPointSuiteData::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "guid",
            |m: &HomeMarkPointFurnitureData| { &m.guid },
            |m: &mut HomeMarkPointFurnitureData| { &mut m.guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "furniture_id",
            |m: &HomeMarkPointFurnitureData| { &m.furniture_id },
            |m: &mut HomeMarkPointFurnitureData| { &mut m.furniture_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "furniture_type",
            |m: &HomeMarkPointFurnitureData| { &m.furniture_type },
            |m: &mut HomeMarkPointFurnitureData| { &mut m.furniture_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "pos",
            |m: &HomeMarkPointFurnitureData| { &m.pos },
            |m: &mut HomeMarkPointFurnitureData| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HomeMarkPointNPCData::HomeMarkPointNPCData>(
            "npc_data",
            HomeMarkPointFurnitureData::has_npc_data,
            HomeMarkPointFurnitureData::npc_data,
            HomeMarkPointFurnitureData::mut_npc_data,
            HomeMarkPointFurnitureData::set_npc_data,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HomeMarkPointSuiteData::HomeMarkPointSuiteData>(
            "suite_data",
            HomeMarkPointFurnitureData::has_suite_data,
            HomeMarkPointFurnitureData::suite_data,
            HomeMarkPointFurnitureData::mut_suite_data,
            HomeMarkPointFurnitureData::set_suite_data,
        ));
        oneofs.push(home_mark_point_furniture_data::Extra::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeMarkPointFurnitureData>(
            "HomeMarkPointFurnitureData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeMarkPointFurnitureData {
    const NAME: &'static str = "HomeMarkPointFurnitureData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.guid = is.read_uint32()?;
                },
                16 => {
                    self.furniture_id = is.read_uint32()?;
                },
                24 => {
                    self.furniture_type = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pos)?;
                },
                50 => {
                    self.extra = ::std::option::Option::Some(home_mark_point_furniture_data::Extra::NpcData(is.read_message()?));
                },
                58 => {
                    self.extra = ::std::option::Option::Some(home_mark_point_furniture_data::Extra::SuiteData(is.read_message()?));
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
        if self.guid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.guid);
        }
        if self.furniture_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.furniture_id);
        }
        if self.furniture_type != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.furniture_type);
        }
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.extra {
            match v {
                &home_mark_point_furniture_data::Extra::NpcData(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &home_mark_point_furniture_data::Extra::SuiteData(ref v) => {
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
        if self.guid != 0 {
            os.write_uint32(1, self.guid)?;
        }
        if self.furniture_id != 0 {
            os.write_uint32(2, self.furniture_id)?;
        }
        if self.furniture_type != 0 {
            os.write_uint32(3, self.furniture_type)?;
        }
        if let Some(v) = self.pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.extra {
            match v {
                &home_mark_point_furniture_data::Extra::NpcData(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
                },
                &home_mark_point_furniture_data::Extra::SuiteData(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> HomeMarkPointFurnitureData {
        HomeMarkPointFurnitureData::new()
    }

    fn clear(&mut self) {
        self.guid = 0;
        self.furniture_id = 0;
        self.furniture_type = 0;
        self.pos.clear();
        self.extra = ::std::option::Option::None;
        self.extra = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeMarkPointFurnitureData {
        static instance: HomeMarkPointFurnitureData = HomeMarkPointFurnitureData {
            guid: 0,
            furniture_id: 0,
            furniture_type: 0,
            pos: ::protobuf::MessageField::none(),
            extra: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeMarkPointFurnitureData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeMarkPointFurnitureData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeMarkPointFurnitureData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeMarkPointFurnitureData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `HomeMarkPointFurnitureData`
pub mod home_mark_point_furniture_data {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:HomeMarkPointFurnitureData.extra)
    pub enum Extra {
        // @@protoc_insertion_point(oneof_field:HomeMarkPointFurnitureData.npc_data)
        NpcData(super::super::HomeMarkPointNPCData::HomeMarkPointNPCData),
        // @@protoc_insertion_point(oneof_field:HomeMarkPointFurnitureData.suite_data)
        SuiteData(super::super::HomeMarkPointSuiteData::HomeMarkPointSuiteData),
    }

    impl ::protobuf::Oneof for Extra {
    }

    impl ::protobuf::OneofFull for Extra {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::HomeMarkPointFurnitureData as ::protobuf::MessageFull>::descriptor().oneof_by_name("extra").unwrap()).clone()
        }
    }

    impl Extra {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Extra>("extra")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20HomeMarkPointFurnitureData.proto\x1a\x0cVector.proto\x1a\x1aHomeMa\
    rkPointNPCData.proto\x1a\x1cHomeMarkPointSuiteData.proto\"\x8c\x02\n\x1a\
    HomeMarkPointFurnitureData\x12\x12\n\x04guid\x18\x01\x20\x01(\rR\x04guid\
    \x12!\n\x0cfurniture_id\x18\x02\x20\x01(\rR\x0bfurnitureId\x12%\n\x0efur\
    niture_type\x18\x03\x20\x01(\rR\rfurnitureType\x12\x19\n\x03pos\x18\x04\
    \x20\x01(\x0b2\x07.VectorR\x03pos\x122\n\x08npc_data\x18\x06\x20\x01(\
    \x0b2\x15.HomeMarkPointNPCDataH\0R\x07npcData\x128\n\nsuite_data\x18\x07\
    \x20\x01(\x0b2\x17.HomeMarkPointSuiteDataH\0R\tsuiteDataB\x07\n\x05extra\
    B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::Vector::file_descriptor().clone());
            deps.push(super::HomeMarkPointNPCData::file_descriptor().clone());
            deps.push(super::HomeMarkPointSuiteData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HomeMarkPointFurnitureData::generated_message_descriptor_data());
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
