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

//! Generated file from `ToTheMoonQueryPathReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:ToTheMoonQueryPathReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ToTheMoonQueryPathReq {
    // message fields
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.filter_type)
    pub filter_type: ::protobuf::EnumOrUnknown<to_the_moon_query_path_req::FilterType>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.query_type)
    pub query_type: ::protobuf::EnumOrUnknown<to_the_moon_query_path_req::OptionType>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.PMELMGPKENE)
    pub PMELMGPKENE: bool,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.destination_pos)
    pub destination_pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.fuzzy_range)
    pub fuzzy_range: i32,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.EAHHCMBPGDJ)
    pub EAHHCMBPGDJ: bool,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.CCHHHAJICHB)
    pub CCHHHAJICHB: ::std::vec::Vec<i32>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.astar_method)
    pub astar_method: ::protobuf::EnumOrUnknown<to_the_moon_query_path_req::AStarMethod>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.source_pos)
    pub source_pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ToTheMoonQueryPathReq.query_id)
    pub query_id: i32,
    // special fields
    // @@protoc_insertion_point(special_field:ToTheMoonQueryPathReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ToTheMoonQueryPathReq {
    fn default() -> &'a ToTheMoonQueryPathReq {
        <ToTheMoonQueryPathReq as ::protobuf::Message>::default_instance()
    }
}

impl ToTheMoonQueryPathReq {
    pub fn new() -> ToTheMoonQueryPathReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "filter_type",
            |m: &ToTheMoonQueryPathReq| { &m.filter_type },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.filter_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "query_type",
            |m: &ToTheMoonQueryPathReq| { &m.query_type },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.query_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMELMGPKENE",
            |m: &ToTheMoonQueryPathReq| { &m.PMELMGPKENE },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.PMELMGPKENE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "destination_pos",
            |m: &ToTheMoonQueryPathReq| { &m.destination_pos },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.destination_pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &ToTheMoonQueryPathReq| { &m.scene_id },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fuzzy_range",
            |m: &ToTheMoonQueryPathReq| { &m.fuzzy_range },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.fuzzy_range },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EAHHCMBPGDJ",
            |m: &ToTheMoonQueryPathReq| { &m.EAHHCMBPGDJ },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.EAHHCMBPGDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CCHHHAJICHB",
            |m: &ToTheMoonQueryPathReq| { &m.CCHHHAJICHB },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.CCHHHAJICHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "astar_method",
            |m: &ToTheMoonQueryPathReq| { &m.astar_method },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.astar_method },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "source_pos",
            |m: &ToTheMoonQueryPathReq| { &m.source_pos },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.source_pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "query_id",
            |m: &ToTheMoonQueryPathReq| { &m.query_id },
            |m: &mut ToTheMoonQueryPathReq| { &mut m.query_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ToTheMoonQueryPathReq>(
            "ToTheMoonQueryPathReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ToTheMoonQueryPathReq {
    const NAME: &'static str = "ToTheMoonQueryPathReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.filter_type = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.query_type = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.PMELMGPKENE = is.read_bool()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.destination_pos)?;
                },
                72 => {
                    self.scene_id = is.read_uint32()?;
                },
                64 => {
                    self.fuzzy_range = is.read_int32()?;
                },
                40 => {
                    self.EAHHCMBPGDJ = is.read_bool()?;
                },
                106 => {
                    is.read_repeated_packed_int32_into(&mut self.CCHHHAJICHB)?;
                },
                104 => {
                    self.CCHHHAJICHB.push(is.read_int32()?);
                },
                24 => {
                    self.astar_method = is.read_enum_or_unknown()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.source_pos)?;
                },
                112 => {
                    self.query_id = is.read_int32()?;
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
        if self.filter_type != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::FilterType::FilterType_ALL) {
            my_size += ::protobuf::rt::int32_size(15, self.filter_type.value());
        }
        if self.query_type != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::OptionType::OPTION_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.query_type.value());
        }
        if self.PMELMGPKENE != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.destination_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.scene_id);
        }
        if self.fuzzy_range != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.fuzzy_range);
        }
        if self.EAHHCMBPGDJ != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_int32_size(13, &self.CCHHHAJICHB);
        if self.astar_method != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::AStarMethod::AStarMethod_CLASSIC) {
            my_size += ::protobuf::rt::int32_size(3, self.astar_method.value());
        }
        if let Some(v) = self.source_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.query_id != 0 {
            my_size += ::protobuf::rt::int32_size(14, self.query_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.filter_type != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::FilterType::FilterType_ALL) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.filter_type))?;
        }
        if self.query_type != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::OptionType::OPTION_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.query_type))?;
        }
        if self.PMELMGPKENE != false {
            os.write_bool(7, self.PMELMGPKENE)?;
        }
        if let Some(v) = self.destination_pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.scene_id != 0 {
            os.write_uint32(9, self.scene_id)?;
        }
        if self.fuzzy_range != 0 {
            os.write_int32(8, self.fuzzy_range)?;
        }
        if self.EAHHCMBPGDJ != false {
            os.write_bool(5, self.EAHHCMBPGDJ)?;
        }
        os.write_repeated_packed_int32(13, &self.CCHHHAJICHB)?;
        if self.astar_method != ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::AStarMethod::AStarMethod_CLASSIC) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.astar_method))?;
        }
        if let Some(v) = self.source_pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.query_id != 0 {
            os.write_int32(14, self.query_id)?;
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

    fn new() -> ToTheMoonQueryPathReq {
        ToTheMoonQueryPathReq::new()
    }

    fn clear(&mut self) {
        self.filter_type = ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::FilterType::FilterType_ALL);
        self.query_type = ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::OptionType::OPTION_NONE);
        self.PMELMGPKENE = false;
        self.destination_pos.clear();
        self.scene_id = 0;
        self.fuzzy_range = 0;
        self.EAHHCMBPGDJ = false;
        self.CCHHHAJICHB.clear();
        self.astar_method = ::protobuf::EnumOrUnknown::new(to_the_moon_query_path_req::AStarMethod::AStarMethod_CLASSIC);
        self.source_pos.clear();
        self.query_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ToTheMoonQueryPathReq {
        static instance: ToTheMoonQueryPathReq = ToTheMoonQueryPathReq {
            filter_type: ::protobuf::EnumOrUnknown::from_i32(0),
            query_type: ::protobuf::EnumOrUnknown::from_i32(0),
            PMELMGPKENE: false,
            destination_pos: ::protobuf::MessageField::none(),
            scene_id: 0,
            fuzzy_range: 0,
            EAHHCMBPGDJ: false,
            CCHHHAJICHB: ::std::vec::Vec::new(),
            astar_method: ::protobuf::EnumOrUnknown::from_i32(0),
            source_pos: ::protobuf::MessageField::none(),
            query_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ToTheMoonQueryPathReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ToTheMoonQueryPathReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ToTheMoonQueryPathReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToTheMoonQueryPathReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ToTheMoonQueryPathReq`
pub mod to_the_moon_query_path_req {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:ToTheMoonQueryPathReq.OptionType)
    pub enum OptionType {
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.OptionType.OPTION_NONE)
        OPTION_NONE = 0,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.OptionType.OPTION_NORMAL)
        OPTION_NORMAL = 1,
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
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<OptionType> {
            match str {
                "OPTION_NONE" => ::std::option::Option::Some(OptionType::OPTION_NONE),
                "OPTION_NORMAL" => ::std::option::Option::Some(OptionType::OPTION_NORMAL),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [OptionType] = &[
            OptionType::OPTION_NONE,
            OptionType::OPTION_NORMAL,
        ];
    }

    impl ::protobuf::EnumFull for OptionType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("ToTheMoonQueryPathReq.OptionType").unwrap()).clone()
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
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<OptionType>("ToTheMoonQueryPathReq.OptionType")
        }
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:ToTheMoonQueryPathReq.AStarMethod)
    pub enum AStarMethod {
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.AStarMethod.AStarMethod_CLASSIC)
        AStarMethod_CLASSIC = 0,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.AStarMethod.AStarMethod_TENDENCY)
        AStarMethod_TENDENCY = 1,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.AStarMethod.AStarMethod_ADAPTIVE)
        AStarMethod_ADAPTIVE = 2,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.AStarMethod.AStarMethod_INFLECTION)
        AStarMethod_INFLECTION = 3,
    }

    impl ::protobuf::Enum for AStarMethod {
        const NAME: &'static str = "AStarMethod";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<AStarMethod> {
            match value {
                0 => ::std::option::Option::Some(AStarMethod::AStarMethod_CLASSIC),
                1 => ::std::option::Option::Some(AStarMethod::AStarMethod_TENDENCY),
                2 => ::std::option::Option::Some(AStarMethod::AStarMethod_ADAPTIVE),
                3 => ::std::option::Option::Some(AStarMethod::AStarMethod_INFLECTION),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<AStarMethod> {
            match str {
                "AStarMethod_CLASSIC" => ::std::option::Option::Some(AStarMethod::AStarMethod_CLASSIC),
                "AStarMethod_TENDENCY" => ::std::option::Option::Some(AStarMethod::AStarMethod_TENDENCY),
                "AStarMethod_ADAPTIVE" => ::std::option::Option::Some(AStarMethod::AStarMethod_ADAPTIVE),
                "AStarMethod_INFLECTION" => ::std::option::Option::Some(AStarMethod::AStarMethod_INFLECTION),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [AStarMethod] = &[
            AStarMethod::AStarMethod_CLASSIC,
            AStarMethod::AStarMethod_TENDENCY,
            AStarMethod::AStarMethod_ADAPTIVE,
            AStarMethod::AStarMethod_INFLECTION,
        ];
    }

    impl ::protobuf::EnumFull for AStarMethod {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("ToTheMoonQueryPathReq.AStarMethod").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for AStarMethod {
        fn default() -> Self {
            AStarMethod::AStarMethod_CLASSIC
        }
    }

    impl AStarMethod {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<AStarMethod>("ToTheMoonQueryPathReq.AStarMethod")
        }
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:ToTheMoonQueryPathReq.FilterType)
    pub enum FilterType {
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.FilterType.FilterType_ALL)
        FilterType_ALL = 0,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.FilterType.FilterType_AIR)
        FilterType_AIR = 1,
        // @@protoc_insertion_point(enum_value:ToTheMoonQueryPathReq.FilterType.FilterType_WATER)
        FilterType_WATER = 2,
    }

    impl ::protobuf::Enum for FilterType {
        const NAME: &'static str = "FilterType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<FilterType> {
            match value {
                0 => ::std::option::Option::Some(FilterType::FilterType_ALL),
                1 => ::std::option::Option::Some(FilterType::FilterType_AIR),
                2 => ::std::option::Option::Some(FilterType::FilterType_WATER),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<FilterType> {
            match str {
                "FilterType_ALL" => ::std::option::Option::Some(FilterType::FilterType_ALL),
                "FilterType_AIR" => ::std::option::Option::Some(FilterType::FilterType_AIR),
                "FilterType_WATER" => ::std::option::Option::Some(FilterType::FilterType_WATER),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [FilterType] = &[
            FilterType::FilterType_ALL,
            FilterType::FilterType_AIR,
            FilterType::FilterType_WATER,
        ];
    }

    impl ::protobuf::EnumFull for FilterType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("ToTheMoonQueryPathReq.FilterType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for FilterType {
        fn default() -> Self {
            FilterType::FilterType_ALL
        }
    }

    impl FilterType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FilterType>("ToTheMoonQueryPathReq.FilterType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bToTheMoonQueryPathReq.proto\x1a\x0cVector.proto\"\xf1\x05\n\x15ToT\
    heMoonQueryPathReq\x12B\n\x0bfilter_type\x18\x0f\x20\x01(\x0e2!.ToTheMoo\
    nQueryPathReq.FilterTypeR\nfilterType\x12@\n\nquery_type\x18\n\x20\x01(\
    \x0e2!.ToTheMoonQueryPathReq.OptionTypeR\tqueryType\x12\x20\n\x0bPMELMGP\
    KENE\x18\x07\x20\x01(\x08R\x0bPMELMGPKENE\x120\n\x0fdestination_pos\x18\
    \x01\x20\x01(\x0b2\x07.VectorR\x0edestinationPos\x12\x19\n\x08scene_id\
    \x18\t\x20\x01(\rR\x07sceneId\x12\x1f\n\x0bfuzzy_range\x18\x08\x20\x01(\
    \x05R\nfuzzyRange\x12\x20\n\x0bEAHHCMBPGDJ\x18\x05\x20\x01(\x08R\x0bEAHH\
    CMBPGDJ\x12\x20\n\x0bCCHHHAJICHB\x18\r\x20\x03(\x05R\x0bCCHHHAJICHB\x12E\
    \n\x0castar_method\x18\x03\x20\x01(\x0e2\".ToTheMoonQueryPathReq.AStarMe\
    thodR\x0bastarMethod\x12&\n\nsource_pos\x18\x02\x20\x01(\x0b2\x07.Vector\
    R\tsourcePos\x12\x19\n\x08query_id\x18\x0e\x20\x01(\x05R\x07queryId\"0\n\
    \nOptionType\x12\x0f\n\x0bOPTION_NONE\x10\0\x12\x11\n\rOPTION_NORMAL\x10\
    \x01\"v\n\x0bAStarMethod\x12\x17\n\x13AStarMethod_CLASSIC\x10\0\x12\x18\
    \n\x14AStarMethod_TENDENCY\x10\x01\x12\x18\n\x14AStarMethod_ADAPTIVE\x10\
    \x02\x12\x1a\n\x16AStarMethod_INFLECTION\x10\x03\"J\n\nFilterType\x12\
    \x12\n\x0eFilterType_ALL\x10\0\x12\x12\n\x0eFilterType_AIR\x10\x01\x12\
    \x14\n\x10FilterType_WATER\x10\x02B\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ToTheMoonQueryPathReq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(3);
            enums.push(to_the_moon_query_path_req::OptionType::generated_enum_descriptor_data());
            enums.push(to_the_moon_query_path_req::AStarMethod::generated_enum_descriptor_data());
            enums.push(to_the_moon_query_path_req::FilterType::generated_enum_descriptor_data());
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
