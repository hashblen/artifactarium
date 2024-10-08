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

//! Generated file from `UseItemReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:UseItemReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UseItemReq {
    // message fields
    // @@protoc_insertion_point(field:UseItemReq.count)
    pub count: u32,
    // @@protoc_insertion_point(field:UseItemReq.option_idx)
    pub option_idx: u32,
    // @@protoc_insertion_point(field:UseItemReq.target_guid)
    pub target_guid: u64,
    // @@protoc_insertion_point(field:UseItemReq.is_enter_mp_dungeon_team)
    pub is_enter_mp_dungeon_team: bool,
    // @@protoc_insertion_point(field:UseItemReq.guid)
    pub guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:UseItemReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UseItemReq {
    fn default() -> &'a UseItemReq {
        <UseItemReq as ::protobuf::Message>::default_instance()
    }
}

impl UseItemReq {
    pub fn new() -> UseItemReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &UseItemReq| { &m.count },
            |m: &mut UseItemReq| { &mut m.count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "option_idx",
            |m: &UseItemReq| { &m.option_idx },
            |m: &mut UseItemReq| { &mut m.option_idx },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_guid",
            |m: &UseItemReq| { &m.target_guid },
            |m: &mut UseItemReq| { &mut m.target_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_enter_mp_dungeon_team",
            |m: &UseItemReq| { &m.is_enter_mp_dungeon_team },
            |m: &mut UseItemReq| { &mut m.is_enter_mp_dungeon_team },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "guid",
            |m: &UseItemReq| { &m.guid },
            |m: &mut UseItemReq| { &mut m.guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UseItemReq>(
            "UseItemReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UseItemReq {
    const NAME: &'static str = "UseItemReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.count = is.read_uint32()?;
                },
                48 => {
                    self.option_idx = is.read_uint32()?;
                },
                40 => {
                    self.target_guid = is.read_uint64()?;
                },
                80 => {
                    self.is_enter_mp_dungeon_team = is.read_bool()?;
                },
                32 => {
                    self.guid = is.read_uint64()?;
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
        if self.count != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.count);
        }
        if self.option_idx != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.option_idx);
        }
        if self.target_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(5, self.target_guid);
        }
        if self.is_enter_mp_dungeon_team != false {
            my_size += 1 + 1;
        }
        if self.guid != 0 {
            my_size += ::protobuf::rt::uint64_size(4, self.guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.count != 0 {
            os.write_uint32(9, self.count)?;
        }
        if self.option_idx != 0 {
            os.write_uint32(6, self.option_idx)?;
        }
        if self.target_guid != 0 {
            os.write_uint64(5, self.target_guid)?;
        }
        if self.is_enter_mp_dungeon_team != false {
            os.write_bool(10, self.is_enter_mp_dungeon_team)?;
        }
        if self.guid != 0 {
            os.write_uint64(4, self.guid)?;
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

    fn new() -> UseItemReq {
        UseItemReq::new()
    }

    fn clear(&mut self) {
        self.count = 0;
        self.option_idx = 0;
        self.target_guid = 0;
        self.is_enter_mp_dungeon_team = false;
        self.guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UseItemReq {
        static instance: UseItemReq = UseItemReq {
            count: 0,
            option_idx: 0,
            target_guid: 0,
            is_enter_mp_dungeon_team: false,
            guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UseItemReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UseItemReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UseItemReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UseItemReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10UseItemReq.proto\"\xae\x01\n\nUseItemReq\x12\x14\n\x05count\x18\t\
    \x20\x01(\rR\x05count\x12\x1d\n\noption_idx\x18\x06\x20\x01(\rR\toptionI\
    dx\x12\x1f\n\x0btarget_guid\x18\x05\x20\x01(\x04R\ntargetGuid\x126\n\x18\
    is_enter_mp_dungeon_team\x18\n\x20\x01(\x08R\x14isEnterMpDungeonTeam\x12\
    \x12\n\x04guid\x18\x04\x20\x01(\x04R\x04guidB\x1b\n\x19emu.grasscutter.n\
    et.protob\x06proto3\
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
            messages.push(UseItemReq::generated_message_descriptor_data());
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
