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

//! Generated file from `WorldOwnerBlossomBriefInfoNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:WorldOwnerBlossomBriefInfoNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct WorldOwnerBlossomBriefInfoNotify {
    // message fields
    // @@protoc_insertion_point(field:WorldOwnerBlossomBriefInfoNotify.brief_info_list)
    pub brief_info_list: ::std::vec::Vec<super::BlossomBriefInfo::BlossomBriefInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:WorldOwnerBlossomBriefInfoNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WorldOwnerBlossomBriefInfoNotify {
    fn default() -> &'a WorldOwnerBlossomBriefInfoNotify {
        <WorldOwnerBlossomBriefInfoNotify as ::protobuf::Message>::default_instance()
    }
}

impl WorldOwnerBlossomBriefInfoNotify {
    pub fn new() -> WorldOwnerBlossomBriefInfoNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "brief_info_list",
            |m: &WorldOwnerBlossomBriefInfoNotify| { &m.brief_info_list },
            |m: &mut WorldOwnerBlossomBriefInfoNotify| { &mut m.brief_info_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WorldOwnerBlossomBriefInfoNotify>(
            "WorldOwnerBlossomBriefInfoNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WorldOwnerBlossomBriefInfoNotify {
    const NAME: &'static str = "WorldOwnerBlossomBriefInfoNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.brief_info_list.push(is.read_message()?);
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
        for value in &self.brief_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.brief_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> WorldOwnerBlossomBriefInfoNotify {
        WorldOwnerBlossomBriefInfoNotify::new()
    }

    fn clear(&mut self) {
        self.brief_info_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WorldOwnerBlossomBriefInfoNotify {
        static instance: WorldOwnerBlossomBriefInfoNotify = WorldOwnerBlossomBriefInfoNotify {
            brief_info_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WorldOwnerBlossomBriefInfoNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WorldOwnerBlossomBriefInfoNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WorldOwnerBlossomBriefInfoNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WorldOwnerBlossomBriefInfoNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&WorldOwnerBlossomBriefInfoNotify.proto\x1a\x16BlossomBriefInfo.proto\
    \"]\n\x20WorldOwnerBlossomBriefInfoNotify\x129\n\x0fbrief_info_list\x18\
    \x0b\x20\x03(\x0b2\x11.BlossomBriefInfoR\rbriefInfoListB\x1b\n\x19emu.gr\
    asscutter.net.protob\x06proto3\
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
            deps.push(super::BlossomBriefInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(WorldOwnerBlossomBriefInfoNotify::generated_message_descriptor_data());
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
