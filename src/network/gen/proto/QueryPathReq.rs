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

//! Generated file from `QueryPathReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:QueryPathReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QueryPathReq {
    // message fields
    // @@protoc_insertion_point(field:QueryPathReq.destination_pos)
    pub destination_pos: ::std::vec::Vec<super::Vector::Vector>,
    // @@protoc_insertion_point(field:QueryPathReq.query_type)
    pub query_type: ::protobuf::EnumOrUnknown<query_path_req::OptionType>,
    // @@protoc_insertion_point(field:QueryPathReq.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:QueryPathReq.source_pos)
    pub source_pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:QueryPathReq.ANCGPGGGOAJ)
    pub ANCGPGGGOAJ: ::protobuf::MessageField<super::Vector3Int::Vector3Int>,
    // @@protoc_insertion_point(field:QueryPathReq.IFMLKJBFKDK)
    pub IFMLKJBFKDK: ::protobuf::MessageField<super::Vector3Int::Vector3Int>,
    // @@protoc_insertion_point(field:QueryPathReq.query_id)
    pub query_id: i32,
    // @@protoc_insertion_point(field:QueryPathReq.filter)
    pub filter: ::protobuf::MessageField<super::QueryFilter::QueryFilter>,
    // special fields
    // @@protoc_insertion_point(special_field:QueryPathReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QueryPathReq {
    fn default() -> &'a QueryPathReq {
        <QueryPathReq as ::protobuf::Message>::default_instance()
    }
}

impl QueryPathReq {
    pub fn new() -> QueryPathReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "destination_pos",
            |m: &QueryPathReq| { &m.destination_pos },
            |m: &mut QueryPathReq| { &mut m.destination_pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "query_type",
            |m: &QueryPathReq| { &m.query_type },
            |m: &mut QueryPathReq| { &mut m.query_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &QueryPathReq| { &m.scene_id },
            |m: &mut QueryPathReq| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "source_pos",
            |m: &QueryPathReq| { &m.source_pos },
            |m: &mut QueryPathReq| { &mut m.source_pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector3Int::Vector3Int>(
            "ANCGPGGGOAJ",
            |m: &QueryPathReq| { &m.ANCGPGGGOAJ },
            |m: &mut QueryPathReq| { &mut m.ANCGPGGGOAJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector3Int::Vector3Int>(
            "IFMLKJBFKDK",
            |m: &QueryPathReq| { &m.IFMLKJBFKDK },
            |m: &mut QueryPathReq| { &mut m.IFMLKJBFKDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "query_id",
            |m: &QueryPathReq| { &m.query_id },
            |m: &mut QueryPathReq| { &mut m.query_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::QueryFilter::QueryFilter>(
            "filter",
            |m: &QueryPathReq| { &m.filter },
            |m: &mut QueryPathReq| { &mut m.filter },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QueryPathReq>(
            "QueryPathReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QueryPathReq {
    const NAME: &'static str = "QueryPathReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.destination_pos.push(is.read_message()?);
                },
                24 => {
                    self.query_type = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.scene_id = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.source_pos)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ANCGPGGGOAJ)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IFMLKJBFKDK)?;
                },
                112 => {
                    self.query_id = is.read_int32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.filter)?;
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
        for value in &self.destination_pos {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.query_type != ::protobuf::EnumOrUnknown::new(query_path_req::OptionType::OPTION_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.query_type.value());
        }
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.scene_id);
        }
        if let Some(v) = self.source_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ANCGPGGGOAJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IFMLKJBFKDK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.query_id != 0 {
            my_size += ::protobuf::rt::int32_size(14, self.query_id);
        }
        if let Some(v) = self.filter.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.destination_pos {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.query_type != ::protobuf::EnumOrUnknown::new(query_path_req::OptionType::OPTION_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.query_type))?;
        }
        if self.scene_id != 0 {
            os.write_uint32(4, self.scene_id)?;
        }
        if let Some(v) = self.source_pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.ANCGPGGGOAJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.IFMLKJBFKDK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.query_id != 0 {
            os.write_int32(14, self.query_id)?;
        }
        if let Some(v) = self.filter.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> QueryPathReq {
        QueryPathReq::new()
    }

    fn clear(&mut self) {
        self.destination_pos.clear();
        self.query_type = ::protobuf::EnumOrUnknown::new(query_path_req::OptionType::OPTION_NONE);
        self.scene_id = 0;
        self.source_pos.clear();
        self.ANCGPGGGOAJ.clear();
        self.IFMLKJBFKDK.clear();
        self.query_id = 0;
        self.filter.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QueryPathReq {
        static instance: QueryPathReq = QueryPathReq {
            destination_pos: ::std::vec::Vec::new(),
            query_type: ::protobuf::EnumOrUnknown::from_i32(0),
            scene_id: 0,
            source_pos: ::protobuf::MessageField::none(),
            ANCGPGGGOAJ: ::protobuf::MessageField::none(),
            IFMLKJBFKDK: ::protobuf::MessageField::none(),
            query_id: 0,
            filter: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QueryPathReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QueryPathReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QueryPathReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryPathReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `QueryPathReq`
pub mod query_path_req {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:QueryPathReq.OptionType)
    pub enum OptionType {
        // @@protoc_insertion_point(enum_value:QueryPathReq.OptionType.OPTION_NONE)
        OPTION_NONE = 0,
        // @@protoc_insertion_point(enum_value:QueryPathReq.OptionType.OPTION_NORMAL)
        OPTION_NORMAL = 1,
        // @@protoc_insertion_point(enum_value:QueryPathReq.OptionType.OPTION_FIRST_CAN_GO)
        OPTION_FIRST_CAN_GO = 2,
    }

    impl ::protobuf::Enum for OptionType {
        const NAME: &'static str = "OptionType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<OptionType> {
            match value {
                0 => ::std::option::Option::Some(OptionType::OPTION_NONE),
                1 => ::std::option::Option::Some(OptionType::OPTION_NORMAL),
                2 => ::std::option::Option::Some(OptionType::OPTION_FIRST_CAN_GO),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<OptionType> {
            match str {
                "OPTION_NONE" => ::std::option::Option::Some(OptionType::OPTION_NONE),
                "OPTION_NORMAL" => ::std::option::Option::Some(OptionType::OPTION_NORMAL),
                "OPTION_FIRST_CAN_GO" => ::std::option::Option::Some(OptionType::OPTION_FIRST_CAN_GO),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [OptionType] = &[
            OptionType::OPTION_NONE,
            OptionType::OPTION_NORMAL,
            OptionType::OPTION_FIRST_CAN_GO,
        ];
    }

    impl ::protobuf::EnumFull for OptionType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("QueryPathReq.OptionType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for OptionType {
        fn default() -> Self {
            OptionType::OPTION_NONE
        }
    }

    impl OptionType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<OptionType>("QueryPathReq.OptionType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12QueryPathReq.proto\x1a\x0cVector.proto\x1a\x10Vector3Int.proto\x1a\
    \x11QueryFilter.proto\"\xa6\x03\n\x0cQueryPathReq\x120\n\x0fdestination_\
    pos\x18\x02\x20\x03(\x0b2\x07.VectorR\x0edestinationPos\x127\n\nquery_ty\
    pe\x18\x03\x20\x01(\x0e2\x18.QueryPathReq.OptionTypeR\tqueryType\x12\x19\
    \n\x08scene_id\x18\x04\x20\x01(\rR\x07sceneId\x12&\n\nsource_pos\x18\x05\
    \x20\x01(\x0b2\x07.VectorR\tsourcePos\x12-\n\x0bANCGPGGGOAJ\x18\x08\x20\
    \x01(\x0b2\x0b.Vector3IntR\x0bANCGPGGGOAJ\x12-\n\x0bIFMLKJBFKDK\x18\t\
    \x20\x01(\x0b2\x0b.Vector3IntR\x0bIFMLKJBFKDK\x12\x19\n\x08query_id\x18\
    \x0e\x20\x01(\x05R\x07queryId\x12$\n\x06filter\x18\x0f\x20\x01(\x0b2\x0c\
    .QueryFilterR\x06filter\"I\n\nOptionType\x12\x0f\n\x0bOPTION_NONE\x10\0\
    \x12\x11\n\rOPTION_NORMAL\x10\x01\x12\x17\n\x13OPTION_FIRST_CAN_GO\x10\
    \x02B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::Vector3Int::file_descriptor().clone());
            deps.push(super::QueryFilter::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(QueryPathReq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(query_path_req::OptionType::generated_enum_descriptor_data());
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
