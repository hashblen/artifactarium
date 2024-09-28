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

//! Generated file from `ClientGadgetInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:ClientGadgetInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientGadgetInfo {
    // message fields
    // @@protoc_insertion_point(field:ClientGadgetInfo.camp_id)
    pub camp_id: u32,
    // @@protoc_insertion_point(field:ClientGadgetInfo.camp_type)
    pub camp_type: u32,
    // @@protoc_insertion_point(field:ClientGadgetInfo.guid)
    pub guid: u64,
    // @@protoc_insertion_point(field:ClientGadgetInfo.owner_entity_id)
    pub owner_entity_id: u32,
    // @@protoc_insertion_point(field:ClientGadgetInfo.target_entity_id)
    pub target_entity_id: u32,
    // @@protoc_insertion_point(field:ClientGadgetInfo.async_load)
    pub async_load: bool,
    // @@protoc_insertion_point(field:ClientGadgetInfo.is_peer_id_from_player)
    pub is_peer_id_from_player: bool,
    // @@protoc_insertion_point(field:ClientGadgetInfo.target_entity_id_list)
    pub target_entity_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ClientGadgetInfo.target_lock_point_index_list)
    pub target_lock_point_index_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:ClientGadgetInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientGadgetInfo {
    fn default() -> &'a ClientGadgetInfo {
        <ClientGadgetInfo as ::protobuf::Message>::default_instance()
    }
}

impl ClientGadgetInfo {
    pub fn new() -> ClientGadgetInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "camp_id",
            |m: &ClientGadgetInfo| { &m.camp_id },
            |m: &mut ClientGadgetInfo| { &mut m.camp_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "camp_type",
            |m: &ClientGadgetInfo| { &m.camp_type },
            |m: &mut ClientGadgetInfo| { &mut m.camp_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "guid",
            |m: &ClientGadgetInfo| { &m.guid },
            |m: &mut ClientGadgetInfo| { &mut m.guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "owner_entity_id",
            |m: &ClientGadgetInfo| { &m.owner_entity_id },
            |m: &mut ClientGadgetInfo| { &mut m.owner_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_entity_id",
            |m: &ClientGadgetInfo| { &m.target_entity_id },
            |m: &mut ClientGadgetInfo| { &mut m.target_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "async_load",
            |m: &ClientGadgetInfo| { &m.async_load },
            |m: &mut ClientGadgetInfo| { &mut m.async_load },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_peer_id_from_player",
            |m: &ClientGadgetInfo| { &m.is_peer_id_from_player },
            |m: &mut ClientGadgetInfo| { &mut m.is_peer_id_from_player },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "target_entity_id_list",
            |m: &ClientGadgetInfo| { &m.target_entity_id_list },
            |m: &mut ClientGadgetInfo| { &mut m.target_entity_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "target_lock_point_index_list",
            |m: &ClientGadgetInfo| { &m.target_lock_point_index_list },
            |m: &mut ClientGadgetInfo| { &mut m.target_lock_point_index_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientGadgetInfo>(
            "ClientGadgetInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientGadgetInfo {
    const NAME: &'static str = "ClientGadgetInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.camp_id = is.read_uint32()?;
                },
                16 => {
                    self.camp_type = is.read_uint32()?;
                },
                24 => {
                    self.guid = is.read_uint64()?;
                },
                32 => {
                    self.owner_entity_id = is.read_uint32()?;
                },
                40 => {
                    self.target_entity_id = is.read_uint32()?;
                },
                48 => {
                    self.async_load = is.read_bool()?;
                },
                56 => {
                    self.is_peer_id_from_player = is.read_bool()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.target_entity_id_list)?;
                },
                64 => {
                    self.target_entity_id_list.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.target_lock_point_index_list)?;
                },
                72 => {
                    self.target_lock_point_index_list.push(is.read_uint32()?);
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
        if self.camp_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.camp_id);
        }
        if self.camp_type != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.camp_type);
        }
        if self.guid != 0 {
            my_size += ::protobuf::rt::uint64_size(3, self.guid);
        }
        if self.owner_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.owner_entity_id);
        }
        if self.target_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.target_entity_id);
        }
        if self.async_load != false {
            my_size += 1 + 1;
        }
        if self.is_peer_id_from_player != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(8, &self.target_entity_id_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.target_lock_point_index_list);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.camp_id != 0 {
            os.write_uint32(1, self.camp_id)?;
        }
        if self.camp_type != 0 {
            os.write_uint32(2, self.camp_type)?;
        }
        if self.guid != 0 {
            os.write_uint64(3, self.guid)?;
        }
        if self.owner_entity_id != 0 {
            os.write_uint32(4, self.owner_entity_id)?;
        }
        if self.target_entity_id != 0 {
            os.write_uint32(5, self.target_entity_id)?;
        }
        if self.async_load != false {
            os.write_bool(6, self.async_load)?;
        }
        if self.is_peer_id_from_player != false {
            os.write_bool(7, self.is_peer_id_from_player)?;
        }
        os.write_repeated_packed_uint32(8, &self.target_entity_id_list)?;
        os.write_repeated_packed_uint32(9, &self.target_lock_point_index_list)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ClientGadgetInfo {
        ClientGadgetInfo::new()
    }

    fn clear(&mut self) {
        self.camp_id = 0;
        self.camp_type = 0;
        self.guid = 0;
        self.owner_entity_id = 0;
        self.target_entity_id = 0;
        self.async_load = false;
        self.is_peer_id_from_player = false;
        self.target_entity_id_list.clear();
        self.target_lock_point_index_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientGadgetInfo {
        static instance: ClientGadgetInfo = ClientGadgetInfo {
            camp_id: 0,
            camp_type: 0,
            guid: 0,
            owner_entity_id: 0,
            target_entity_id: 0,
            async_load: false,
            is_peer_id_from_player: false,
            target_entity_id_list: ::std::vec::Vec::new(),
            target_lock_point_index_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientGadgetInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientGadgetInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientGadgetInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientGadgetInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16ClientGadgetInfo.proto\"\xf4\x02\n\x10ClientGadgetInfo\x12\x17\n\
    \x07camp_id\x18\x01\x20\x01(\rR\x06campId\x12\x1b\n\tcamp_type\x18\x02\
    \x20\x01(\rR\x08campType\x12\x12\n\x04guid\x18\x03\x20\x01(\x04R\x04guid\
    \x12&\n\x0fowner_entity_id\x18\x04\x20\x01(\rR\rownerEntityId\x12(\n\x10\
    target_entity_id\x18\x05\x20\x01(\rR\x0etargetEntityId\x12\x1d\n\nasync_\
    load\x18\x06\x20\x01(\x08R\tasyncLoad\x122\n\x16is_peer_id_from_player\
    \x18\x07\x20\x01(\x08R\x12isPeerIdFromPlayer\x121\n\x15target_entity_id_\
    list\x18\x08\x20\x03(\rR\x12targetEntityIdList\x12>\n\x1ctarget_lock_poi\
    nt_index_list\x18\t\x20\x03(\rR\x18targetLockPointIndexListB\x1b\n\x19em\
    u.grasscutter.net.protob\x06proto3\
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
            messages.push(ClientGadgetInfo::generated_message_descriptor_data());
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
