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

//! Generated file from `QuickUseWidgetReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:QuickUseWidgetReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QuickUseWidgetReq {
    // message fields
    // @@protoc_insertion_point(field:QuickUseWidgetReq.EEJNPDEBBIK)
    pub EEJNPDEBBIK: bool,
    // message oneof groups
    pub param: ::std::option::Option<quick_use_widget_req::Param>,
    // special fields
    // @@protoc_insertion_point(special_field:QuickUseWidgetReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QuickUseWidgetReq {
    fn default() -> &'a QuickUseWidgetReq {
        <QuickUseWidgetReq as ::protobuf::Message>::default_instance()
    }
}

impl QuickUseWidgetReq {
    pub fn new() -> QuickUseWidgetReq {
        ::std::default::Default::default()
    }

    // .WidgetCreateLocationInfo location_info = 615;

    pub fn location_info(&self) -> &super::WidgetCreateLocationInfo::WidgetCreateLocationInfo {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(ref v)) => v,
            _ => <super::WidgetCreateLocationInfo::WidgetCreateLocationInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_location_info(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_location_info(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_location_info(&mut self, v: super::WidgetCreateLocationInfo::WidgetCreateLocationInfo) {
        self.param = ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_location_info(&mut self) -> &mut super::WidgetCreateLocationInfo::WidgetCreateLocationInfo {
        if let ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(_)) = self.param {
        } else {
            self.param = ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(super::WidgetCreateLocationInfo::WidgetCreateLocationInfo::new()));
        }
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_location_info(&mut self) -> super::WidgetCreateLocationInfo::WidgetCreateLocationInfo {
        if self.has_location_info() {
            match self.param.take() {
                ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::WidgetCreateLocationInfo::WidgetCreateLocationInfo::new()
        }
    }

    // .WidgetCameraInfo camera_info = 564;

    pub fn camera_info(&self) -> &super::WidgetCameraInfo::WidgetCameraInfo {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(ref v)) => v,
            _ => <super::WidgetCameraInfo::WidgetCameraInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_camera_info(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_camera_info(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_camera_info(&mut self, v: super::WidgetCameraInfo::WidgetCameraInfo) {
        self.param = ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_camera_info(&mut self) -> &mut super::WidgetCameraInfo::WidgetCameraInfo {
        if let ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(_)) = self.param {
        } else {
            self.param = ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(super::WidgetCameraInfo::WidgetCameraInfo::new()));
        }
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_camera_info(&mut self) -> super::WidgetCameraInfo::WidgetCameraInfo {
        if self.has_camera_info() {
            match self.param.take() {
                ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::WidgetCameraInfo::WidgetCameraInfo::new()
        }
    }

    // .WidgetCreatorInfo creator_info = 777;

    pub fn creator_info(&self) -> &super::WidgetCreatorInfo::WidgetCreatorInfo {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(ref v)) => v,
            _ => <super::WidgetCreatorInfo::WidgetCreatorInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_creator_info(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_creator_info(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_creator_info(&mut self, v: super::WidgetCreatorInfo::WidgetCreatorInfo) {
        self.param = ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_creator_info(&mut self) -> &mut super::WidgetCreatorInfo::WidgetCreatorInfo {
        if let ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(_)) = self.param {
        } else {
            self.param = ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(super::WidgetCreatorInfo::WidgetCreatorInfo::new()));
        }
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_creator_info(&mut self) -> super::WidgetCreatorInfo::WidgetCreatorInfo {
        if self.has_creator_info() {
            match self.param.take() {
                ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::WidgetCreatorInfo::WidgetCreatorInfo::new()
        }
    }

    // .WidgetThunderBirdFeatherInfo thunder_bird_feather_info = 320;

    pub fn thunder_bird_feather_info(&self) -> &super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(ref v)) => v,
            _ => <super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_thunder_bird_feather_info(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_thunder_bird_feather_info(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_thunder_bird_feather_info(&mut self, v: super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo) {
        self.param = ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_thunder_bird_feather_info(&mut self) -> &mut super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo {
        if let ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(_)) = self.param {
        } else {
            self.param = ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo::new()));
        }
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_thunder_bird_feather_info(&mut self) -> super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo {
        if self.has_thunder_bird_feather_info() {
            match self.param.take() {
                ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo::new()
        }
    }

    // .WidgetSorushInfo sorush_info = 141;

    pub fn sorush_info(&self) -> &super::WidgetSorushInfo::WidgetSorushInfo {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(ref v)) => v,
            _ => <super::WidgetSorushInfo::WidgetSorushInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_sorush_info(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_sorush_info(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sorush_info(&mut self, v: super::WidgetSorushInfo::WidgetSorushInfo) {
        self.param = ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_sorush_info(&mut self) -> &mut super::WidgetSorushInfo::WidgetSorushInfo {
        if let ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(_)) = self.param {
        } else {
            self.param = ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(super::WidgetSorushInfo::WidgetSorushInfo::new()));
        }
        match self.param {
            ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_sorush_info(&mut self) -> super::WidgetSorushInfo::WidgetSorushInfo {
        if self.has_sorush_info() {
            match self.param.take() {
                ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(v)) => v,
                _ => panic!(),
            }
        } else {
            super::WidgetSorushInfo::WidgetSorushInfo::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::WidgetCreateLocationInfo::WidgetCreateLocationInfo>(
            "location_info",
            QuickUseWidgetReq::has_location_info,
            QuickUseWidgetReq::location_info,
            QuickUseWidgetReq::mut_location_info,
            QuickUseWidgetReq::set_location_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::WidgetCameraInfo::WidgetCameraInfo>(
            "camera_info",
            QuickUseWidgetReq::has_camera_info,
            QuickUseWidgetReq::camera_info,
            QuickUseWidgetReq::mut_camera_info,
            QuickUseWidgetReq::set_camera_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::WidgetCreatorInfo::WidgetCreatorInfo>(
            "creator_info",
            QuickUseWidgetReq::has_creator_info,
            QuickUseWidgetReq::creator_info,
            QuickUseWidgetReq::mut_creator_info,
            QuickUseWidgetReq::set_creator_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo>(
            "thunder_bird_feather_info",
            QuickUseWidgetReq::has_thunder_bird_feather_info,
            QuickUseWidgetReq::thunder_bird_feather_info,
            QuickUseWidgetReq::mut_thunder_bird_feather_info,
            QuickUseWidgetReq::set_thunder_bird_feather_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::WidgetSorushInfo::WidgetSorushInfo>(
            "sorush_info",
            QuickUseWidgetReq::has_sorush_info,
            QuickUseWidgetReq::sorush_info,
            QuickUseWidgetReq::mut_sorush_info,
            QuickUseWidgetReq::set_sorush_info,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EEJNPDEBBIK",
            |m: &QuickUseWidgetReq| { &m.EEJNPDEBBIK },
            |m: &mut QuickUseWidgetReq| { &mut m.EEJNPDEBBIK },
        ));
        oneofs.push(quick_use_widget_req::Param::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuickUseWidgetReq>(
            "QuickUseWidgetReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QuickUseWidgetReq {
    const NAME: &'static str = "QuickUseWidgetReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                4922 => {
                    self.param = ::std::option::Option::Some(quick_use_widget_req::Param::LocationInfo(is.read_message()?));
                },
                4514 => {
                    self.param = ::std::option::Option::Some(quick_use_widget_req::Param::CameraInfo(is.read_message()?));
                },
                6218 => {
                    self.param = ::std::option::Option::Some(quick_use_widget_req::Param::CreatorInfo(is.read_message()?));
                },
                2562 => {
                    self.param = ::std::option::Option::Some(quick_use_widget_req::Param::ThunderBirdFeatherInfo(is.read_message()?));
                },
                1130 => {
                    self.param = ::std::option::Option::Some(quick_use_widget_req::Param::SorushInfo(is.read_message()?));
                },
                6448 => {
                    self.EEJNPDEBBIK = is.read_bool()?;
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
        if self.EEJNPDEBBIK != false {
            my_size += 2 + 1;
        }
        if let ::std::option::Option::Some(ref v) = self.param {
            match v {
                &quick_use_widget_req::Param::LocationInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &quick_use_widget_req::Param::CameraInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &quick_use_widget_req::Param::CreatorInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &quick_use_widget_req::Param::ThunderBirdFeatherInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &quick_use_widget_req::Param::SorushInfo(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EEJNPDEBBIK != false {
            os.write_bool(806, self.EEJNPDEBBIK)?;
        }
        if let ::std::option::Option::Some(ref v) = self.param {
            match v {
                &quick_use_widget_req::Param::LocationInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(615, v, os)?;
                },
                &quick_use_widget_req::Param::CameraInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(564, v, os)?;
                },
                &quick_use_widget_req::Param::CreatorInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(777, v, os)?;
                },
                &quick_use_widget_req::Param::ThunderBirdFeatherInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(320, v, os)?;
                },
                &quick_use_widget_req::Param::SorushInfo(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(141, v, os)?;
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

    fn new() -> QuickUseWidgetReq {
        QuickUseWidgetReq::new()
    }

    fn clear(&mut self) {
        self.param = ::std::option::Option::None;
        self.param = ::std::option::Option::None;
        self.param = ::std::option::Option::None;
        self.param = ::std::option::Option::None;
        self.param = ::std::option::Option::None;
        self.EEJNPDEBBIK = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QuickUseWidgetReq {
        static instance: QuickUseWidgetReq = QuickUseWidgetReq {
            EEJNPDEBBIK: false,
            param: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QuickUseWidgetReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QuickUseWidgetReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QuickUseWidgetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuickUseWidgetReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `QuickUseWidgetReq`
pub mod quick_use_widget_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:QuickUseWidgetReq.param)
    pub enum Param {
        // @@protoc_insertion_point(oneof_field:QuickUseWidgetReq.location_info)
        LocationInfo(super::super::WidgetCreateLocationInfo::WidgetCreateLocationInfo),
        // @@protoc_insertion_point(oneof_field:QuickUseWidgetReq.camera_info)
        CameraInfo(super::super::WidgetCameraInfo::WidgetCameraInfo),
        // @@protoc_insertion_point(oneof_field:QuickUseWidgetReq.creator_info)
        CreatorInfo(super::super::WidgetCreatorInfo::WidgetCreatorInfo),
        // @@protoc_insertion_point(oneof_field:QuickUseWidgetReq.thunder_bird_feather_info)
        ThunderBirdFeatherInfo(super::super::WidgetThunderBirdFeatherInfo::WidgetThunderBirdFeatherInfo),
        // @@protoc_insertion_point(oneof_field:QuickUseWidgetReq.sorush_info)
        SorushInfo(super::super::WidgetSorushInfo::WidgetSorushInfo),
    }

    impl ::protobuf::Oneof for Param {
    }

    impl ::protobuf::OneofFull for Param {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::QuickUseWidgetReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("param").unwrap()).clone()
        }
    }

    impl Param {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Param>("param")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17QuickUseWidgetReq.proto\x1a\x1eWidgetCreateLocationInfo.proto\x1a\
    \x16WidgetCameraInfo.proto\x1a\x17WidgetCreatorInfo.proto\x1a\"WidgetThu\
    nderBirdFeatherInfo.proto\x1a\x16WidgetSorushInfo.proto\"\x87\x03\n\x11Q\
    uickUseWidgetReq\x12A\n\rlocation_info\x18\xe7\x04\x20\x01(\x0b2\x19.Wid\
    getCreateLocationInfoH\0R\x0clocationInfo\x125\n\x0bcamera_info\x18\xb4\
    \x04\x20\x01(\x0b2\x11.WidgetCameraInfoH\0R\ncameraInfo\x128\n\x0ccreato\
    r_info\x18\x89\x06\x20\x01(\x0b2\x12.WidgetCreatorInfoH\0R\x0bcreatorInf\
    o\x12[\n\x19thunder_bird_feather_info\x18\xc0\x02\x20\x01(\x0b2\x1d.Widg\
    etThunderBirdFeatherInfoH\0R\x16thunderBirdFeatherInfo\x125\n\x0bsorush_\
    info\x18\x8d\x01\x20\x01(\x0b2\x11.WidgetSorushInfoH\0R\nsorushInfo\x12!\
    \n\x0bEEJNPDEBBIK\x18\xa6\x06\x20\x01(\x08R\x0bEEJNPDEBBIKB\x07\n\x05par\
    amB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::WidgetCreateLocationInfo::file_descriptor().clone());
            deps.push(super::WidgetCameraInfo::file_descriptor().clone());
            deps.push(super::WidgetCreatorInfo::file_descriptor().clone());
            deps.push(super::WidgetThunderBirdFeatherInfo::file_descriptor().clone());
            deps.push(super::WidgetSorushInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(QuickUseWidgetReq::generated_message_descriptor_data());
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
