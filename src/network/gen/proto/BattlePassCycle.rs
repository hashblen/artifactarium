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

//! Generated file from `BattlePassCycle.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:BattlePassCycle)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattlePassCycle {
    // message fields
    // @@protoc_insertion_point(field:BattlePassCycle.begin_time)
    pub begin_time: u32,
    // @@protoc_insertion_point(field:BattlePassCycle.cycle_idx)
    pub cycle_idx: u32,
    // @@protoc_insertion_point(field:BattlePassCycle.end_time)
    pub end_time: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BattlePassCycle.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattlePassCycle {
    fn default() -> &'a BattlePassCycle {
        <BattlePassCycle as ::protobuf::Message>::default_instance()
    }
}

impl BattlePassCycle {
    pub fn new() -> BattlePassCycle {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "begin_time",
            |m: &BattlePassCycle| { &m.begin_time },
            |m: &mut BattlePassCycle| { &mut m.begin_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cycle_idx",
            |m: &BattlePassCycle| { &m.cycle_idx },
            |m: &mut BattlePassCycle| { &mut m.cycle_idx },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_time",
            |m: &BattlePassCycle| { &m.end_time },
            |m: &mut BattlePassCycle| { &mut m.end_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattlePassCycle>(
            "BattlePassCycle",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattlePassCycle {
    const NAME: &'static str = "BattlePassCycle";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.begin_time = is.read_uint32()?;
                },
                104 => {
                    self.cycle_idx = is.read_uint32()?;
                },
                112 => {
                    self.end_time = is.read_uint32()?;
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
        if self.begin_time != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.begin_time);
        }
        if self.cycle_idx != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.cycle_idx);
        }
        if self.end_time != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.end_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.begin_time != 0 {
            os.write_uint32(1, self.begin_time)?;
        }
        if self.cycle_idx != 0 {
            os.write_uint32(13, self.cycle_idx)?;
        }
        if self.end_time != 0 {
            os.write_uint32(14, self.end_time)?;
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

    fn new() -> BattlePassCycle {
        BattlePassCycle::new()
    }

    fn clear(&mut self) {
        self.begin_time = 0;
        self.cycle_idx = 0;
        self.end_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattlePassCycle {
        static instance: BattlePassCycle = BattlePassCycle {
            begin_time: 0,
            cycle_idx: 0,
            end_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BattlePassCycle {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattlePassCycle").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattlePassCycle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattlePassCycle {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15BattlePassCycle.proto\"h\n\x0fBattlePassCycle\x12\x1d\n\nbegin_tim\
    e\x18\x01\x20\x01(\rR\tbeginTime\x12\x1b\n\tcycle_idx\x18\r\x20\x01(\rR\
    \x08cycleIdx\x12\x19\n\x08end_time\x18\x0e\x20\x01(\rR\x07endTimeB\x1b\n\
    \x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(BattlePassCycle::generated_message_descriptor_data());
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
