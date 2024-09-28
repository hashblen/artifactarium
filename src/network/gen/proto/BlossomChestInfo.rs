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

//! Generated file from `BlossomChestInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:BlossomChestInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BlossomChestInfo {
    // message fields
    // @@protoc_insertion_point(field:BlossomChestInfo.resin)
    pub resin: u32,
    // @@protoc_insertion_point(field:BlossomChestInfo.qualify_uid_list)
    pub qualify_uid_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BlossomChestInfo.remain_uid_list)
    pub remain_uid_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BlossomChestInfo.dead_time)
    pub dead_time: u32,
    // @@protoc_insertion_point(field:BlossomChestInfo.blossom_refresh_type)
    pub blossom_refresh_type: u32,
    // @@protoc_insertion_point(field:BlossomChestInfo.refresh_id)
    pub refresh_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BlossomChestInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BlossomChestInfo {
    fn default() -> &'a BlossomChestInfo {
        <BlossomChestInfo as ::protobuf::Message>::default_instance()
    }
}

impl BlossomChestInfo {
    pub fn new() -> BlossomChestInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "resin",
            |m: &BlossomChestInfo| { &m.resin },
            |m: &mut BlossomChestInfo| { &mut m.resin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "qualify_uid_list",
            |m: &BlossomChestInfo| { &m.qualify_uid_list },
            |m: &mut BlossomChestInfo| { &mut m.qualify_uid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "remain_uid_list",
            |m: &BlossomChestInfo| { &m.remain_uid_list },
            |m: &mut BlossomChestInfo| { &mut m.remain_uid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dead_time",
            |m: &BlossomChestInfo| { &m.dead_time },
            |m: &mut BlossomChestInfo| { &mut m.dead_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "blossom_refresh_type",
            |m: &BlossomChestInfo| { &m.blossom_refresh_type },
            |m: &mut BlossomChestInfo| { &mut m.blossom_refresh_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "refresh_id",
            |m: &BlossomChestInfo| { &m.refresh_id },
            |m: &mut BlossomChestInfo| { &mut m.refresh_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BlossomChestInfo>(
            "BlossomChestInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BlossomChestInfo {
    const NAME: &'static str = "BlossomChestInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.resin = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.qualify_uid_list)?;
                },
                16 => {
                    self.qualify_uid_list.push(is.read_uint32()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.remain_uid_list)?;
                },
                24 => {
                    self.remain_uid_list.push(is.read_uint32()?);
                },
                32 => {
                    self.dead_time = is.read_uint32()?;
                },
                40 => {
                    self.blossom_refresh_type = is.read_uint32()?;
                },
                48 => {
                    self.refresh_id = is.read_uint32()?;
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
        if self.resin != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.resin);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.qualify_uid_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(3, &self.remain_uid_list);
        if self.dead_time != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.dead_time);
        }
        if self.blossom_refresh_type != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.blossom_refresh_type);
        }
        if self.refresh_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.refresh_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.resin != 0 {
            os.write_uint32(1, self.resin)?;
        }
        os.write_repeated_packed_uint32(2, &self.qualify_uid_list)?;
        os.write_repeated_packed_uint32(3, &self.remain_uid_list)?;
        if self.dead_time != 0 {
            os.write_uint32(4, self.dead_time)?;
        }
        if self.blossom_refresh_type != 0 {
            os.write_uint32(5, self.blossom_refresh_type)?;
        }
        if self.refresh_id != 0 {
            os.write_uint32(6, self.refresh_id)?;
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

    fn new() -> BlossomChestInfo {
        BlossomChestInfo::new()
    }

    fn clear(&mut self) {
        self.resin = 0;
        self.qualify_uid_list.clear();
        self.remain_uid_list.clear();
        self.dead_time = 0;
        self.blossom_refresh_type = 0;
        self.refresh_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BlossomChestInfo {
        static instance: BlossomChestInfo = BlossomChestInfo {
            resin: 0,
            qualify_uid_list: ::std::vec::Vec::new(),
            remain_uid_list: ::std::vec::Vec::new(),
            dead_time: 0,
            blossom_refresh_type: 0,
            refresh_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BlossomChestInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BlossomChestInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BlossomChestInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlossomChestInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16BlossomChestInfo.proto\"\xe8\x01\n\x10BlossomChestInfo\x12\x14\n\
    \x05resin\x18\x01\x20\x01(\rR\x05resin\x12(\n\x10qualify_uid_list\x18\
    \x02\x20\x03(\rR\x0equalifyUidList\x12&\n\x0fremain_uid_list\x18\x03\x20\
    \x03(\rR\rremainUidList\x12\x1b\n\tdead_time\x18\x04\x20\x01(\rR\x08dead\
    Time\x120\n\x14blossom_refresh_type\x18\x05\x20\x01(\rR\x12blossomRefres\
    hType\x12\x1d\n\nrefresh_id\x18\x06\x20\x01(\rR\trefreshIdB\x1b\n\x19emu\
    .grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BlossomChestInfo::generated_message_descriptor_data());
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
