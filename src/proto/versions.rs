// This file is generated by rust-protobuf 2.16.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `versions.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_16_2;

#[derive(PartialEq,Clone,Default)]
pub struct Versions {
    // message fields
    pub packages: ::protobuf::RepeatedField<Package>,
    repository: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Versions {
    fn default() -> &'a Versions {
        <Versions as ::protobuf::Message>::default_instance()
    }
}

impl Versions {
    pub fn new() -> Versions {
        ::std::default::Default::default()
    }

    // repeated .Package packages = 1;


    pub fn get_packages(&self) -> &[Package] {
        &self.packages
    }
    pub fn clear_packages(&mut self) {
        self.packages.clear();
    }

    // Param is passed by value, moved
    pub fn set_packages(&mut self, v: ::protobuf::RepeatedField<Package>) {
        self.packages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_packages(&mut self) -> &mut ::protobuf::RepeatedField<Package> {
        &mut self.packages
    }

    // Take field
    pub fn take_packages(&mut self) -> ::protobuf::RepeatedField<Package> {
        ::std::mem::replace(&mut self.packages, ::protobuf::RepeatedField::new())
    }

    // required string repository = 2;


    pub fn get_repository(&self) -> &str {
        match self.repository.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_repository(&mut self) {
        self.repository.clear();
    }

    pub fn has_repository(&self) -> bool {
        self.repository.is_some()
    }

    // Param is passed by value, moved
    pub fn set_repository(&mut self, v: ::std::string::String) {
        self.repository = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_repository(&mut self) -> &mut ::std::string::String {
        if self.repository.is_none() {
            self.repository.set_default();
        }
        self.repository.as_mut().unwrap()
    }

    // Take field
    pub fn take_repository(&mut self) -> ::std::string::String {
        self.repository.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for Versions {
    fn is_initialized(&self) -> bool {
        if self.repository.is_none() {
            return false;
        }
        for v in &self.packages {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.packages)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.repository)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.packages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.repository.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.packages {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.repository.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Versions {
        Versions::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Package>>(
                "packages",
                |m: &Versions| { &m.packages },
                |m: &mut Versions| { &mut m.packages },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "repository",
                |m: &Versions| { &m.repository },
                |m: &mut Versions| { &mut m.repository },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Versions>(
                "Versions",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Versions {
        static instance: ::protobuf::rt::LazyV2<Versions> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Versions::new)
    }
}

impl ::protobuf::Clear for Versions {
    fn clear(&mut self) {
        self.packages.clear();
        self.repository.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Versions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Versions {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Package {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    pub versions: ::protobuf::RepeatedField<::std::string::String>,
    pub retired: ::std::vec::Vec<i32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Package {
    fn default() -> &'a Package {
        <Package as ::protobuf::Message>::default_instance()
    }
}

impl Package {
    pub fn new() -> Package {
        ::std::default::Default::default()
    }

    // required string name = 1;


    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // repeated string versions = 2;


    pub fn get_versions(&self) -> &[::std::string::String] {
        &self.versions
    }
    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.versions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_versions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.versions
    }

    // Take field
    pub fn take_versions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.versions, ::protobuf::RepeatedField::new())
    }

    // repeated int32 retired = 3;


    pub fn get_retired(&self) -> &[i32] {
        &self.retired
    }
    pub fn clear_retired(&mut self) {
        self.retired.clear();
    }

    // Param is passed by value, moved
    pub fn set_retired(&mut self, v: ::std::vec::Vec<i32>) {
        self.retired = v;
    }

    // Mutable pointer to the field.
    pub fn mut_retired(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.retired
    }

    // Take field
    pub fn take_retired(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.retired, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Package {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.versions)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.retired)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.versions {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if !self.retired.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.retired);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.versions {
            os.write_string(2, &v)?;
        };
        if !self.retired.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.retired))?;
            for v in &self.retired {
                os.write_int32_no_tag(*v)?;
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Package {
        Package::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Package| { &m.name },
                |m: &mut Package| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "versions",
                |m: &Package| { &m.versions },
                |m: &mut Package| { &mut m.versions },
            ));
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "retired",
                |m: &Package| { &m.retired },
                |m: &mut Package| { &mut m.retired },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Package>(
                "Package",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Package {
        static instance: ::protobuf::rt::LazyV2<Package> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Package::new)
    }
}

impl ::protobuf::Clear for Package {
    fn clear(&mut self) {
        self.name.clear();
        self.versions.clear();
        self.retired.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Package {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Package {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eversions.proto\x12\0\"@\n\x08Versions\x12\x1c\n\x08packages\x18\
    \x01\x20\x03(\x0b2\x08.PackageB\0\x12\x14\n\nrepository\x18\x02\x20\x02(\
    \tB\0:\0\"D\n\x07Package\x12\x0e\n\x04name\x18\x01\x20\x02(\tB\0\x12\x12\
    \n\x08versions\x18\x02\x20\x03(\tB\0\x12\x13\n\x07retired\x18\x03\x20\
    \x03(\x05B\x02\x10\x01:\0B\0b\x06proto2\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
