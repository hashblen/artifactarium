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

//! Generated file from `ShowEquip.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:ShowEquip)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ShowEquip {
    // message fields
    // @@protoc_insertion_point(field:ShowEquip.item_id)
    pub item_id: u32,
    // message oneof groups
    pub detail: ::std::option::Option<show_equip::Detail>,
    // special fields
    // @@protoc_insertion_point(special_field:ShowEquip.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ShowEquip {
    fn default() -> &'a ShowEquip {
        <ShowEquip as ::protobuf::Message>::default_instance()
    }
}

impl ShowEquip {
    pub fn new() -> ShowEquip {
        ::std::default::Default::default()
    }

    // .Reliquary reliquary = 2;

    pub fn reliquary(&self) -> &super::Reliquary::Reliquary {
        match self.detail {
            ::std::option::Option::Some(show_equip::Detail::Reliquary(ref v)) => v,
            _ => <super::Reliquary::Reliquary as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_reliquary(&mut self) {
        self.detail = ::std::option::Option::None;
    }

    pub fn has_reliquary(&self) -> bool {
        match self.detail {
            ::std::option::Option::Some(show_equip::Detail::Reliquary(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_reliquary(&mut self, v: super::Reliquary::Reliquary) {
        self.detail = ::std::option::Option::Some(show_equip::Detail::Reliquary(v))
    }

    // Mutable pointer to the field.
    pub fn mut_reliquary(&mut self) -> &mut super::Reliquary::Reliquary {
        if let ::std::option::Option::Some(show_equip::Detail::Reliquary(_)) = self.detail {
        } else {
            self.detail = ::std::option::Option::Some(show_equip::Detail::Reliquary(super::Reliquary::Reliquary::new()));
        }
        match self.detail {
            ::std::option::Option::Some(show_equip::Detail::Reliquary(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_reliquary(&mut self) -> super::Reliquary::Reliquary {
        if self.has_reliquary() {
            match self.detail.take() {
                ::std::option::Option::Some(show_equip::Detail::Reliquary(v)) => v,
                _ => panic!(),
            }
        } else {
            super::Reliquary::Reliquary::new()
        }
    }

    // .Weapon weapon = 3;

    pub fn weapon(&self) -> &super::Weapon::Weapon {
        match self.detail {
            ::std::option::Option::Some(show_equip::Detail::Weapon(ref v)) => v,
            _ => <super::Weapon::Weapon as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_weapon(&mut self) {
        self.detail = ::std::option::Option::None;
    }

    pub fn has_weapon(&self) -> bool {
        match self.detail {
            ::std::option::Option::Some(show_equip::Detail::Weapon(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_weapon(&mut self, v: super::Weapon::Weapon) {
        self.detail = ::std::option::Option::Some(show_equip::Detail::Weapon(v))
    }

    // Mutable pointer to the field.
    pub fn mut_weapon(&mut self) -> &mut super::Weapon::Weapon {
        if let ::std::option::Option::Some(show_equip::Detail::Weapon(_)) = self.detail {
        } else {
            self.detail = ::std::option::Option::Some(show_equip::Detail::Weapon(super::Weapon::Weapon::new()));
        }
        match self.detail {
            ::std::option::Option::Some(show_equip::Detail::Weapon(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_weapon(&mut self) -> super::Weapon::Weapon {
        if self.has_weapon() {
            match self.detail.take() {
                ::std::option::Option::Some(show_equip::Detail::Weapon(v)) => v,
                _ => panic!(),
            }
        } else {
            super::Weapon::Weapon::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_id",
            |m: &ShowEquip| { &m.item_id },
            |m: &mut ShowEquip| { &mut m.item_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::Reliquary::Reliquary>(
            "reliquary",
            ShowEquip::has_reliquary,
            ShowEquip::reliquary,
            ShowEquip::mut_reliquary,
            ShowEquip::set_reliquary,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::Weapon::Weapon>(
            "weapon",
            ShowEquip::has_weapon,
            ShowEquip::weapon,
            ShowEquip::mut_weapon,
            ShowEquip::set_weapon,
        ));
        oneofs.push(show_equip::Detail::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ShowEquip>(
            "ShowEquip",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ShowEquip {
    const NAME: &'static str = "ShowEquip";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.item_id = is.read_uint32()?;
                },
                18 => {
                    self.detail = ::std::option::Option::Some(show_equip::Detail::Reliquary(is.read_message()?));
                },
                26 => {
                    self.detail = ::std::option::Option::Some(show_equip::Detail::Weapon(is.read_message()?));
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
        if self.item_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.item_id);
        }
        if let ::std::option::Option::Some(ref v) = self.detail {
            match v {
                &show_equip::Detail::Reliquary(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &show_equip::Detail::Weapon(ref v) => {
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
        if self.item_id != 0 {
            os.write_uint32(1, self.item_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.detail {
            match v {
                &show_equip::Detail::Reliquary(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &show_equip::Detail::Weapon(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> ShowEquip {
        ShowEquip::new()
    }

    fn clear(&mut self) {
        self.item_id = 0;
        self.detail = ::std::option::Option::None;
        self.detail = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ShowEquip {
        static instance: ShowEquip = ShowEquip {
            item_id: 0,
            detail: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ShowEquip {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ShowEquip").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ShowEquip {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShowEquip {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ShowEquip`
pub mod show_equip {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ShowEquip.detail)
    pub enum Detail {
        // @@protoc_insertion_point(oneof_field:ShowEquip.reliquary)
        Reliquary(super::super::Reliquary::Reliquary),
        // @@protoc_insertion_point(oneof_field:ShowEquip.weapon)
        Weapon(super::super::Weapon::Weapon),
    }

    impl ::protobuf::Oneof for Detail {
    }

    impl ::protobuf::OneofFull for Detail {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ShowEquip as ::protobuf::MessageFull>::descriptor().oneof_by_name("detail").unwrap()).clone()
        }
    }

    impl Detail {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Detail>("detail")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fShowEquip.proto\x1a\x0fReliquary.proto\x1a\x0cWeapon.proto\"}\n\tS\
    howEquip\x12\x17\n\x07item_id\x18\x01\x20\x01(\rR\x06itemId\x12*\n\treli\
    quary\x18\x02\x20\x01(\x0b2\n.ReliquaryH\0R\treliquary\x12!\n\x06weapon\
    \x18\x03\x20\x01(\x0b2\x07.WeaponH\0R\x06weaponB\x08\n\x06detailB\x1b\n\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::Reliquary::file_descriptor().clone());
            deps.push(super::Weapon::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ShowEquip::generated_message_descriptor_data());
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
