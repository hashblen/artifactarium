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

//! Generated file from `WeaponAwakenReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:WeaponAwakenReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct WeaponAwakenReq {
    // message fields
    // @@protoc_insertion_point(field:WeaponAwakenReq.target_weapon_guid)
    pub target_weapon_guid: u64,
    // @@protoc_insertion_point(field:WeaponAwakenReq.NEHLGIOLBJA)
    pub NEHLGIOLBJA: u32,
    // @@protoc_insertion_point(field:WeaponAwakenReq.HICOIODAHGK)
    pub HICOIODAHGK: ::std::vec::Vec<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:WeaponAwakenReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WeaponAwakenReq {
    fn default() -> &'a WeaponAwakenReq {
        <WeaponAwakenReq as ::protobuf::Message>::default_instance()
    }
}

impl WeaponAwakenReq {
    pub fn new() -> WeaponAwakenReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_weapon_guid",
            |m: &WeaponAwakenReq| { &m.target_weapon_guid },
            |m: &mut WeaponAwakenReq| { &mut m.target_weapon_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEHLGIOLBJA",
            |m: &WeaponAwakenReq| { &m.NEHLGIOLBJA },
            |m: &mut WeaponAwakenReq| { &mut m.NEHLGIOLBJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HICOIODAHGK",
            |m: &WeaponAwakenReq| { &m.HICOIODAHGK },
            |m: &mut WeaponAwakenReq| { &mut m.HICOIODAHGK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WeaponAwakenReq>(
            "WeaponAwakenReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WeaponAwakenReq {
    const NAME: &'static str = "WeaponAwakenReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.target_weapon_guid = is.read_uint64()?;
                },
                72 => {
                    self.NEHLGIOLBJA = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint64_into(&mut self.HICOIODAHGK)?;
                },
                120 => {
                    self.HICOIODAHGK.push(is.read_uint64()?);
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
        if self.target_weapon_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(13, self.target_weapon_guid);
        }
        if self.NEHLGIOLBJA != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.NEHLGIOLBJA);
        }
        my_size += ::protobuf::rt::vec_packed_uint64_size(15, &self.HICOIODAHGK);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.target_weapon_guid != 0 {
            os.write_uint64(13, self.target_weapon_guid)?;
        }
        if self.NEHLGIOLBJA != 0 {
            os.write_uint32(9, self.NEHLGIOLBJA)?;
        }
        os.write_repeated_packed_uint64(15, &self.HICOIODAHGK)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> WeaponAwakenReq {
        WeaponAwakenReq::new()
    }

    fn clear(&mut self) {
        self.target_weapon_guid = 0;
        self.NEHLGIOLBJA = 0;
        self.HICOIODAHGK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WeaponAwakenReq {
        static instance: WeaponAwakenReq = WeaponAwakenReq {
            target_weapon_guid: 0,
            NEHLGIOLBJA: 0,
            HICOIODAHGK: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WeaponAwakenReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WeaponAwakenReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WeaponAwakenReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WeaponAwakenReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15WeaponAwakenReq.proto\"\x83\x01\n\x0fWeaponAwakenReq\x12,\n\x12tar\
    get_weapon_guid\x18\r\x20\x01(\x04R\x10targetWeaponGuid\x12\x20\n\x0bNEH\
    LGIOLBJA\x18\t\x20\x01(\rR\x0bNEHLGIOLBJA\x12\x20\n\x0bHICOIODAHGK\x18\
    \x0f\x20\x03(\x04R\x0bHICOIODAHGKB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(WeaponAwakenReq::generated_message_descriptor_data());
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
