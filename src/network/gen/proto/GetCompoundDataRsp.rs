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

//! Generated file from `GetCompoundDataRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:GetCompoundDataRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetCompoundDataRsp {
    // message fields
    // @@protoc_insertion_point(field:GetCompoundDataRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:GetCompoundDataRsp.unlockCompoundList)
    pub unlockCompoundList: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetCompoundDataRsp.compoundQueueDataList)
    pub compoundQueueDataList: ::std::vec::Vec<super::CompoundQueueData::CompoundQueueData>,
    // special fields
    // @@protoc_insertion_point(special_field:GetCompoundDataRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetCompoundDataRsp {
    fn default() -> &'a GetCompoundDataRsp {
        <GetCompoundDataRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetCompoundDataRsp {
    pub fn new() -> GetCompoundDataRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetCompoundDataRsp| { &m.retcode },
            |m: &mut GetCompoundDataRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlockCompoundList",
            |m: &GetCompoundDataRsp| { &m.unlockCompoundList },
            |m: &mut GetCompoundDataRsp| { &mut m.unlockCompoundList },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "compoundQueueDataList",
            |m: &GetCompoundDataRsp| { &m.compoundQueueDataList },
            |m: &mut GetCompoundDataRsp| { &mut m.compoundQueueDataList },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetCompoundDataRsp>(
            "GetCompoundDataRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetCompoundDataRsp {
    const NAME: &'static str = "GetCompoundDataRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.retcode = is.read_int32()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.unlockCompoundList)?;
                },
                104 => {
                    self.unlockCompoundList.push(is.read_uint32()?);
                },
                74 => {
                    self.compoundQueueDataList.push(is.read_message()?);
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.retcode);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.unlockCompoundList);
        for value in &self.compoundQueueDataList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_int32(6, self.retcode)?;
        }
        os.write_repeated_packed_uint32(13, &self.unlockCompoundList)?;
        for v in &self.compoundQueueDataList {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> GetCompoundDataRsp {
        GetCompoundDataRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.unlockCompoundList.clear();
        self.compoundQueueDataList.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetCompoundDataRsp {
        static instance: GetCompoundDataRsp = GetCompoundDataRsp {
            retcode: 0,
            unlockCompoundList: ::std::vec::Vec::new(),
            compoundQueueDataList: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetCompoundDataRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetCompoundDataRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetCompoundDataRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetCompoundDataRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18GetCompoundDataRsp.proto\x1a\x17CompoundQueueData.proto\"\xa8\x01\
    \n\x12GetCompoundDataRsp\x12\x18\n\x07retcode\x18\x06\x20\x01(\x05R\x07r\
    etcode\x12.\n\x12unlockCompoundList\x18\r\x20\x03(\rR\x12unlockCompoundL\
    ist\x12H\n\x15compoundQueueDataList\x18\t\x20\x03(\x0b2\x12.CompoundQueu\
    eDataR\x15compoundQueueDataListB\x1b\n\x19emu.grasscutter.net.protob\x06\
    proto3\
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
            deps.push(super::CompoundQueueData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetCompoundDataRsp::generated_message_descriptor_data());
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
