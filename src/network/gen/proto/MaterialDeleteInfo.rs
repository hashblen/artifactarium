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

//! Generated file from `MaterialDeleteInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:MaterialDeleteInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MaterialDeleteInfo {
    // message fields
    // @@protoc_insertion_point(field:MaterialDeleteInfo.delete_time_num_map)
    pub delete_time_num_map: bool,
    // message oneof groups
    pub delete_info: ::std::option::Option<material_delete_info::Delete_info>,
    // special fields
    // @@protoc_insertion_point(special_field:MaterialDeleteInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MaterialDeleteInfo {
    fn default() -> &'a MaterialDeleteInfo {
        <MaterialDeleteInfo as ::protobuf::Message>::default_instance()
    }
}

impl MaterialDeleteInfo {
    pub fn new() -> MaterialDeleteInfo {
        ::std::default::Default::default()
    }

    // .CountDownDelete count_down_delete = 2;

    pub fn count_down_delete(&self) -> &super::CountDownDelete::CountDownDelete {
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(ref v)) => v,
            _ => <super::CountDownDelete::CountDownDelete as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_count_down_delete(&mut self) {
        self.delete_info = ::std::option::Option::None;
    }

    pub fn has_count_down_delete(&self) -> bool {
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_count_down_delete(&mut self, v: super::CountDownDelete::CountDownDelete) {
        self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(v))
    }

    // Mutable pointer to the field.
    pub fn mut_count_down_delete(&mut self) -> &mut super::CountDownDelete::CountDownDelete {
        if let ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(_)) = self.delete_info {
        } else {
            self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(super::CountDownDelete::CountDownDelete::new()));
        }
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_count_down_delete(&mut self) -> super::CountDownDelete::CountDownDelete {
        if self.has_count_down_delete() {
            match self.delete_info.take() {
                ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CountDownDelete::CountDownDelete::new()
        }
    }

    // .DateTimeDelete date_delete = 3;

    pub fn date_delete(&self) -> &super::DateTimeDelete::DateTimeDelete {
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(ref v)) => v,
            _ => <super::DateTimeDelete::DateTimeDelete as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_date_delete(&mut self) {
        self.delete_info = ::std::option::Option::None;
    }

    pub fn has_date_delete(&self) -> bool {
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_date_delete(&mut self, v: super::DateTimeDelete::DateTimeDelete) {
        self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(v))
    }

    // Mutable pointer to the field.
    pub fn mut_date_delete(&mut self) -> &mut super::DateTimeDelete::DateTimeDelete {
        if let ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(_)) = self.delete_info {
        } else {
            self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(super::DateTimeDelete::DateTimeDelete::new()));
        }
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_date_delete(&mut self) -> super::DateTimeDelete::DateTimeDelete {
        if self.has_date_delete() {
            match self.delete_info.take() {
                ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DateTimeDelete::DateTimeDelete::new()
        }
    }

    // .DelayWeekCountDownDelete delay_week_count_down_delete = 4;

    pub fn delay_week_count_down_delete(&self) -> &super::DelayWeekCountDownDelete::DelayWeekCountDownDelete {
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(ref v)) => v,
            _ => <super::DelayWeekCountDownDelete::DelayWeekCountDownDelete as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_delay_week_count_down_delete(&mut self) {
        self.delete_info = ::std::option::Option::None;
    }

    pub fn has_delay_week_count_down_delete(&self) -> bool {
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delay_week_count_down_delete(&mut self, v: super::DelayWeekCountDownDelete::DelayWeekCountDownDelete) {
        self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delay_week_count_down_delete(&mut self) -> &mut super::DelayWeekCountDownDelete::DelayWeekCountDownDelete {
        if let ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(_)) = self.delete_info {
        } else {
            self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(super::DelayWeekCountDownDelete::DelayWeekCountDownDelete::new()));
        }
        match self.delete_info {
            ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delay_week_count_down_delete(&mut self) -> super::DelayWeekCountDownDelete::DelayWeekCountDownDelete {
        if self.has_delay_week_count_down_delete() {
            match self.delete_info.take() {
                ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DelayWeekCountDownDelete::DelayWeekCountDownDelete::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "delete_time_num_map",
            |m: &MaterialDeleteInfo| { &m.delete_time_num_map },
            |m: &mut MaterialDeleteInfo| { &mut m.delete_time_num_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CountDownDelete::CountDownDelete>(
            "count_down_delete",
            MaterialDeleteInfo::has_count_down_delete,
            MaterialDeleteInfo::count_down_delete,
            MaterialDeleteInfo::mut_count_down_delete,
            MaterialDeleteInfo::set_count_down_delete,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DateTimeDelete::DateTimeDelete>(
            "date_delete",
            MaterialDeleteInfo::has_date_delete,
            MaterialDeleteInfo::date_delete,
            MaterialDeleteInfo::mut_date_delete,
            MaterialDeleteInfo::set_date_delete,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DelayWeekCountDownDelete::DelayWeekCountDownDelete>(
            "delay_week_count_down_delete",
            MaterialDeleteInfo::has_delay_week_count_down_delete,
            MaterialDeleteInfo::delay_week_count_down_delete,
            MaterialDeleteInfo::mut_delay_week_count_down_delete,
            MaterialDeleteInfo::set_delay_week_count_down_delete,
        ));
        oneofs.push(material_delete_info::Delete_info::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MaterialDeleteInfo>(
            "MaterialDeleteInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MaterialDeleteInfo {
    const NAME: &'static str = "MaterialDeleteInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.delete_time_num_map = is.read_bool()?;
                },
                18 => {
                    self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::CountDownDelete(is.read_message()?));
                },
                26 => {
                    self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::DateDelete(is.read_message()?));
                },
                34 => {
                    self.delete_info = ::std::option::Option::Some(material_delete_info::Delete_info::DelayWeekCountDownDelete(is.read_message()?));
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
        if self.delete_time_num_map != false {
            my_size += 1 + 1;
        }
        if let ::std::option::Option::Some(ref v) = self.delete_info {
            match v {
                &material_delete_info::Delete_info::CountDownDelete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &material_delete_info::Delete_info::DateDelete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &material_delete_info::Delete_info::DelayWeekCountDownDelete(ref v) => {
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
        if self.delete_time_num_map != false {
            os.write_bool(1, self.delete_time_num_map)?;
        }
        if let ::std::option::Option::Some(ref v) = self.delete_info {
            match v {
                &material_delete_info::Delete_info::CountDownDelete(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &material_delete_info::Delete_info::DateDelete(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &material_delete_info::Delete_info::DelayWeekCountDownDelete(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> MaterialDeleteInfo {
        MaterialDeleteInfo::new()
    }

    fn clear(&mut self) {
        self.delete_time_num_map = false;
        self.delete_info = ::std::option::Option::None;
        self.delete_info = ::std::option::Option::None;
        self.delete_info = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MaterialDeleteInfo {
        static instance: MaterialDeleteInfo = MaterialDeleteInfo {
            delete_time_num_map: false,
            delete_info: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MaterialDeleteInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MaterialDeleteInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MaterialDeleteInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MaterialDeleteInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `MaterialDeleteInfo`
pub mod material_delete_info {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:MaterialDeleteInfo.delete_info)
    pub enum Delete_info {
        // @@protoc_insertion_point(oneof_field:MaterialDeleteInfo.count_down_delete)
        CountDownDelete(super::super::CountDownDelete::CountDownDelete),
        // @@protoc_insertion_point(oneof_field:MaterialDeleteInfo.date_delete)
        DateDelete(super::super::DateTimeDelete::DateTimeDelete),
        // @@protoc_insertion_point(oneof_field:MaterialDeleteInfo.delay_week_count_down_delete)
        DelayWeekCountDownDelete(super::super::DelayWeekCountDownDelete::DelayWeekCountDownDelete),
    }

    impl ::protobuf::Oneof for Delete_info {
    }

    impl ::protobuf::OneofFull for Delete_info {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::MaterialDeleteInfo as ::protobuf::MessageFull>::descriptor().oneof_by_name("delete_info").unwrap()).clone()
        }
    }

    impl Delete_info {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Delete_info>("delete_info")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18MaterialDeleteInfo.proto\x1a\x15CountDownDelete.proto\x1a\x14DateT\
    imeDelete.proto\x1a\x1eDelayWeekCountDownDelete.proto\"\xa3\x02\n\x12Mat\
    erialDeleteInfo\x12-\n\x13delete_time_num_map\x18\x01\x20\x01(\x08R\x10d\
    eleteTimeNumMap\x12>\n\x11count_down_delete\x18\x02\x20\x01(\x0b2\x10.Co\
    untDownDeleteH\0R\x0fcountDownDelete\x122\n\x0bdate_delete\x18\x03\x20\
    \x01(\x0b2\x0f.DateTimeDeleteH\0R\ndateDelete\x12[\n\x1cdelay_week_count\
    _down_delete\x18\x04\x20\x01(\x0b2\x19.DelayWeekCountDownDeleteH\0R\x18d\
    elayWeekCountDownDeleteB\r\n\x0bdelete_infoB\x1b\n\x19emu.grasscutter.ne\
    t.protob\x06proto3\
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
            deps.push(super::CountDownDelete::file_descriptor().clone());
            deps.push(super::DateTimeDelete::file_descriptor().clone());
            deps.push(super::DelayWeekCountDownDelete::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MaterialDeleteInfo::generated_message_descriptor_data());
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
