pub use root::*;

const _: () = ::planus::check_version_compatibility("planus-1.3.0");

/// The root namespace
///
/// Generated from these locations:
/// * File `schema/executable.fbs`
#[no_implicit_prelude]
#[allow(clippy::needless_lifetimes)]
mod root {
    /// The namespace `platforms`
    ///
    /// Generated from these locations:
    /// * File `schema/executable.fbs`
    pub mod platforms {
        /// The namespace `platforms.darwinn`
        ///
        /// Generated from these locations:
        /// * File `schema/executable.fbs`
        pub mod darwinn {
            /// The enum `Description` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Enum `Description` in the file `schema/executable.fbs:10`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i16)]
            pub enum Description {
                /// The variant `BASE_ADDRESS_OUTPUT_ACTIVATION` in the enum `Description`
                BaseAddressOutputActivation = 0,

                /// The variant `BASE_ADDRESS_INPUT_ACTIVATION` in the enum `Description`
                BaseAddressInputActivation = 1,

                /// The variant `BASE_ADDRESS_PARAMETER` in the enum `Description`
                BaseAddressParameter = 2,

                /// The variant `BASE_ADDRESS_SCRATCH` in the enum `Description`
                BaseAddressScratch = 3,
            }

            impl Description {
                /// Array containing all valid variants of Description
                pub const ENUM_VALUES: [Self; 4] = [
                    Self::BaseAddressOutputActivation,
                    Self::BaseAddressInputActivation,
                    Self::BaseAddressParameter,
                    Self::BaseAddressScratch,
                ];
            }

            impl ::core::convert::TryFrom<i16> for Description {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i16,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(Description::BaseAddressOutputActivation),
                        1 => ::core::result::Result::Ok(Description::BaseAddressInputActivation),
                        2 => ::core::result::Result::Ok(Description::BaseAddressParameter),
                        3 => ::core::result::Result::Ok(Description::BaseAddressScratch),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<Description> for i16 {
                #[inline]
                fn from(value: Description) -> Self {
                    value as i16
                }
            }

            /// # Safety
            /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
            unsafe impl ::planus::Primitive for Description {
                const ALIGNMENT: usize = 2;
                const SIZE: usize = 2;
            }

            impl ::planus::WriteAsPrimitive<Description> for Description {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i16).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<Description> for Description {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Description {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<Description, Description> for Description {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &Description,
                ) -> ::core::option::Option<Description> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<Description> for Description {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<Description> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for Description {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i16 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for Description {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 2;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value =
                        unsafe { <i16 as ::planus::VectorRead>::from_buffer(buffer, offset) };
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "Description",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<Description> for Description {
                const STRIDE: usize = 2;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 2];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (2 * i) as u32,
                        );
                    }
                }
            }

            /// The enum `Position` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Enum `Position` in the file `schema/executable.fbs:24`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i16)]
            pub enum Position {
                /// The variant `LOWER_32BIT` in the enum `Position`
                Lower32bit = 0,

                /// The variant `UPPER_32BIT` in the enum `Position`
                Upper32bit = 1,
            }

            impl Position {
                /// Array containing all valid variants of Position
                pub const ENUM_VALUES: [Self; 2] = [Self::Lower32bit, Self::Upper32bit];
            }

            impl ::core::convert::TryFrom<i16> for Position {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i16,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(Position::Lower32bit),
                        1 => ::core::result::Result::Ok(Position::Upper32bit),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<Position> for i16 {
                #[inline]
                fn from(value: Position) -> Self {
                    value as i16
                }
            }

            /// # Safety
            /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
            unsafe impl ::planus::Primitive for Position {
                const ALIGNMENT: usize = 2;
                const SIZE: usize = 2;
            }

            impl ::planus::WriteAsPrimitive<Position> for Position {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i16).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<Position> for Position {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Position {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<Position, Position> for Position {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &Position,
                ) -> ::core::option::Option<Position> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<Position> for Position {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<Position> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for Position {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i16 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for Position {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 2;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value =
                        unsafe { <i16 as ::planus::VectorRead>::from_buffer(buffer, offset) };
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "Position",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<Position> for Position {
                const STRIDE: usize = 2;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 2];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (2 * i) as u32,
                        );
                    }
                }
            }

            /// The table `Meta` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `Meta` in the file `schema/executable.fbs:34`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct Meta {
                /// The field `desc` in the table `Meta`
                pub desc: self::Description,
                /// The field `batch` in the table `Meta`
                pub batch: i32,
                /// The field `name` in the table `Meta`
                pub name: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `position` in the table `Meta`
                pub position: self::Position,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for Meta {
                fn default() -> Self {
                    Self {
                        desc: self::Description::BaseAddressOutputActivation,
                        batch: 0,
                        name: ::core::default::Default::default(),
                        position: self::Position::Lower32bit,
                    }
                }
            }

            impl Meta {
                /// Creates a [MetaBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> MetaBuilder<()> {
                    MetaBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_desc: impl ::planus::WriteAsDefault<self::Description, self::Description>,
                    field_batch: impl ::planus::WriteAsDefault<i32, i32>,
                    field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    field_position: impl ::planus::WriteAsDefault<self::Position, self::Position>,
                ) -> ::planus::Offset<Self> {
                    let prepared_desc = field_desc
                        .prepare(builder, &self::Description::BaseAddressOutputActivation);
                    let prepared_batch = field_batch.prepare(builder, &0);
                    let prepared_name = field_name.prepare(builder);
                    let prepared_position =
                        field_position.prepare(builder, &self::Position::Lower32bit);

                    let mut table_writer: ::planus::table_writer::TableWriter<12> =
                        ::core::default::Default::default();
                    if prepared_batch.is_some() {
                        table_writer.write_entry::<i32>(1);
                    }
                    if prepared_name.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(2);
                    }
                    if prepared_desc.is_some() {
                        table_writer.write_entry::<self::Description>(0);
                    }
                    if prepared_position.is_some() {
                        table_writer.write_entry::<self::Position>(3);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_batch) = prepared_batch {
                                object_writer.write::<_, _, 4>(&prepared_batch);
                            }
                            if let ::core::option::Option::Some(prepared_name) = prepared_name {
                                object_writer.write::<_, _, 4>(&prepared_name);
                            }
                            if let ::core::option::Option::Some(prepared_desc) = prepared_desc {
                                object_writer.write::<_, _, 2>(&prepared_desc);
                            }
                            if let ::core::option::Option::Some(prepared_position) =
                                prepared_position
                            {
                                object_writer.write::<_, _, 2>(&prepared_position);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<Meta>> for Meta {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Meta> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<Meta>> for Meta {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Meta>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<Meta> for Meta {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Meta> {
                    Meta::create(builder, self.desc, self.batch, &self.name, self.position)
                }
            }

            /// Builder for serializing an instance of the [Meta] type.
            ///
            /// Can be created using the [Meta::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct MetaBuilder<State>(State);

            impl MetaBuilder<()> {
                /// Setter for the [`desc` field](Meta#structfield.desc).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn desc<T0>(self, value: T0) -> MetaBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<self::Description, self::Description>,
                {
                    MetaBuilder((value,))
                }

                /// Sets the [`desc` field](Meta#structfield.desc) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn desc_as_default(self) -> MetaBuilder<(::planus::DefaultValue,)> {
                    self.desc(::planus::DefaultValue)
                }
            }

            impl<T0> MetaBuilder<(T0,)> {
                /// Setter for the [`batch` field](Meta#structfield.batch).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn batch<T1>(self, value: T1) -> MetaBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0,) = self.0;
                    MetaBuilder((v0, value))
                }

                /// Sets the [`batch` field](Meta#structfield.batch) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn batch_as_default(self) -> MetaBuilder<(T0, ::planus::DefaultValue)> {
                    self.batch(::planus::DefaultValue)
                }
            }

            impl<T0, T1> MetaBuilder<(T0, T1)> {
                /// Setter for the [`name` field](Meta#structfield.name).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name<T2>(self, value: T2) -> MetaBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0, v1) = self.0;
                    MetaBuilder((v0, v1, value))
                }

                /// Sets the [`name` field](Meta#structfield.name) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name_as_null(self) -> MetaBuilder<(T0, T1, ())> {
                    self.name(())
                }
            }

            impl<T0, T1, T2> MetaBuilder<(T0, T1, T2)> {
                /// Setter for the [`position` field](Meta#structfield.position).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn position<T3>(self, value: T3) -> MetaBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsDefault<self::Position, self::Position>,
                {
                    let (v0, v1, v2) = self.0;
                    MetaBuilder((v0, v1, v2, value))
                }

                /// Sets the [`position` field](Meta#structfield.position) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn position_as_default(
                    self,
                ) -> MetaBuilder<(T0, T1, T2, ::planus::DefaultValue)> {
                    self.position(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3> MetaBuilder<(T0, T1, T2, T3)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Meta].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Meta>
                where
                    Self: ::planus::WriteAsOffset<Meta>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<self::Description, self::Description>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T3: ::planus::WriteAsDefault<self::Position, self::Position>,
                > ::planus::WriteAs<::planus::Offset<Meta>> for MetaBuilder<(T0, T1, T2, T3)>
            {
                type Prepared = ::planus::Offset<Meta>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Meta> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<self::Description, self::Description>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T3: ::planus::WriteAsDefault<self::Position, self::Position>,
                > ::planus::WriteAsOptional<::planus::Offset<Meta>>
                for MetaBuilder<(T0, T1, T2, T3)>
            {
                type Prepared = ::planus::Offset<Meta>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Meta>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<self::Description, self::Description>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T3: ::planus::WriteAsDefault<self::Position, self::Position>,
                > ::planus::WriteAsOffset<Meta> for MetaBuilder<(T0, T1, T2, T3)>
            {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Meta> {
                    let (v0, v1, v2, v3) = &self.0;
                    Meta::create(builder, v0, v1, v2, v3)
                }
            }

            /// Reference to a deserialized [Meta].
            #[derive(Copy, Clone)]
            pub struct MetaRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> MetaRef<'a> {
                /// Getter for the [`desc` field](Meta#structfield.desc).
                #[inline]
                pub fn desc(&self) -> ::planus::Result<self::Description> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "Meta", "desc")?
                            .unwrap_or(self::Description::BaseAddressOutputActivation),
                    )
                }

                /// Getter for the [`batch` field](Meta#structfield.batch).
                #[inline]
                pub fn batch(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(self.0.access(1, "Meta", "batch")?.unwrap_or(0))
                }

                /// Getter for the [`name` field](Meta#structfield.name).
                #[inline]
                pub fn name(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(2, "Meta", "name")
                }

                /// Getter for the [`position` field](Meta#structfield.position).
                #[inline]
                pub fn position(&self) -> ::planus::Result<self::Position> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(3, "Meta", "position")?
                            .unwrap_or(self::Position::Lower32bit),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for MetaRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("MetaRef");
                    f.field("desc", &self.desc());
                    f.field("batch", &self.batch());
                    if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                        f.field("name", &field_name);
                    }
                    f.field("position", &self.position());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<MetaRef<'a>> for Meta {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: MetaRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        desc: ::core::convert::TryInto::try_into(value.desc()?)?,
                        batch: ::core::convert::TryInto::try_into(value.batch()?)?,
                        name: value.name()?.map(::core::convert::Into::into),
                        position: ::core::convert::TryInto::try_into(value.position()?)?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for MetaRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for MetaRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location("[MetaRef]", "get", buffer.offset_from_start)
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<Meta>> for Meta {
                type Value = ::planus::Offset<Meta>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<Meta>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for MetaRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[MetaRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `FieldOffset` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `FieldOffset` in the file `schema/executable.fbs:51`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct FieldOffset {
                /// The field `meta` in the table `FieldOffset`
                pub meta: ::core::option::Option<::planus::alloc::boxed::Box<self::Meta>>,
                /// The field `offset_bit` in the table `FieldOffset`
                pub offset_bit: i32,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for FieldOffset {
                fn default() -> Self {
                    Self {
                        meta: ::core::default::Default::default(),
                        offset_bit: 0,
                    }
                }
            }

            impl FieldOffset {
                /// Creates a [FieldOffsetBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> FieldOffsetBuilder<()> {
                    FieldOffsetBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_meta: impl ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    field_offset_bit: impl ::planus::WriteAsDefault<i32, i32>,
                ) -> ::planus::Offset<Self> {
                    let prepared_meta = field_meta.prepare(builder);
                    let prepared_offset_bit = field_offset_bit.prepare(builder, &0);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_meta.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::Meta>>(0);
                    }
                    if prepared_offset_bit.is_some() {
                        table_writer.write_entry::<i32>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_meta) = prepared_meta {
                                object_writer.write::<_, _, 4>(&prepared_meta);
                            }
                            if let ::core::option::Option::Some(prepared_offset_bit) =
                                prepared_offset_bit
                            {
                                object_writer.write::<_, _, 4>(&prepared_offset_bit);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<FieldOffset>> for FieldOffset {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<FieldOffset> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<FieldOffset>> for FieldOffset {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<FieldOffset>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<FieldOffset> for FieldOffset {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<FieldOffset> {
                    FieldOffset::create(builder, &self.meta, self.offset_bit)
                }
            }

            /// Builder for serializing an instance of the [FieldOffset] type.
            ///
            /// Can be created using the [FieldOffset::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct FieldOffsetBuilder<State>(State);

            impl FieldOffsetBuilder<()> {
                /// Setter for the [`meta` field](FieldOffset#structfield.meta).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn meta<T0>(self, value: T0) -> FieldOffsetBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                {
                    FieldOffsetBuilder((value,))
                }

                /// Sets the [`meta` field](FieldOffset#structfield.meta) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn meta_as_null(self) -> FieldOffsetBuilder<((),)> {
                    self.meta(())
                }
            }

            impl<T0> FieldOffsetBuilder<(T0,)> {
                /// Setter for the [`offset_bit` field](FieldOffset#structfield.offset_bit).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn offset_bit<T1>(self, value: T1) -> FieldOffsetBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0,) = self.0;
                    FieldOffsetBuilder((v0, value))
                }

                /// Sets the [`offset_bit` field](FieldOffset#structfield.offset_bit) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn offset_bit_as_default(
                    self,
                ) -> FieldOffsetBuilder<(T0, ::planus::DefaultValue)> {
                    self.offset_bit(::planus::DefaultValue)
                }
            }

            impl<T0, T1> FieldOffsetBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [FieldOffset].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<FieldOffset>
                where
                    Self: ::planus::WriteAsOffset<FieldOffset>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAs<::planus::Offset<FieldOffset>>
                for FieldOffsetBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<FieldOffset>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<FieldOffset> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAsOptional<::planus::Offset<FieldOffset>>
                for FieldOffsetBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<FieldOffset>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<FieldOffset>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAsOffset<FieldOffset> for FieldOffsetBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<FieldOffset> {
                    let (v0, v1) = &self.0;
                    FieldOffset::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [FieldOffset].
            #[derive(Copy, Clone)]
            pub struct FieldOffsetRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> FieldOffsetRef<'a> {
                /// Getter for the [`meta` field](FieldOffset#structfield.meta).
                #[inline]
                pub fn meta(&self) -> ::planus::Result<::core::option::Option<self::MetaRef<'a>>> {
                    self.0.access(0, "FieldOffset", "meta")
                }

                /// Getter for the [`offset_bit` field](FieldOffset#structfield.offset_bit).
                #[inline]
                pub fn offset_bit(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0.access(1, "FieldOffset", "offset_bit")?.unwrap_or(0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for FieldOffsetRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("FieldOffsetRef");
                    if let ::core::option::Option::Some(field_meta) = self.meta().transpose() {
                        f.field("meta", &field_meta);
                    }
                    f.field("offset_bit", &self.offset_bit());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<FieldOffsetRef<'a>> for FieldOffset {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: FieldOffsetRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        meta: if let ::core::option::Option::Some(meta) = value.meta()? {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(meta)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        offset_bit: ::core::convert::TryInto::try_into(value.offset_bit()?)?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for FieldOffsetRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for FieldOffsetRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[FieldOffsetRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<FieldOffset>> for FieldOffset {
                type Value = ::planus::Offset<FieldOffset>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<FieldOffset>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for FieldOffsetRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[FieldOffsetRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `InstructionBitstream` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `InstructionBitstream` in the file `schema/executable.fbs:60`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct InstructionBitstream {
                /// The field `bitstream` in the table `InstructionBitstream`
                pub bitstream: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `field_offsets` in the table `InstructionBitstream`
                pub field_offsets:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::FieldOffset>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for InstructionBitstream {
                fn default() -> Self {
                    Self {
                        bitstream: ::core::default::Default::default(),
                        field_offsets: ::core::default::Default::default(),
                    }
                }
            }

            impl InstructionBitstream {
                /// Creates a [InstructionBitstreamBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> InstructionBitstreamBuilder<()> {
                    InstructionBitstreamBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_bitstream: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_field_offsets: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::FieldOffset>]>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_bitstream = field_bitstream.prepare(builder);
                    let prepared_field_offsets = field_field_offsets.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_bitstream.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(0);
                    }
                    if prepared_field_offsets.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<[::planus::Offset<self::FieldOffset>]>>(
                                1,
                            );
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_bitstream) =
                                prepared_bitstream
                            {
                                object_writer.write::<_, _, 4>(&prepared_bitstream);
                            }
                            if let ::core::option::Option::Some(prepared_field_offsets) =
                                prepared_field_offsets
                            {
                                object_writer.write::<_, _, 4>(&prepared_field_offsets);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<InstructionBitstream>> for InstructionBitstream {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionBitstream> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<InstructionBitstream>> for InstructionBitstream {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InstructionBitstream>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<InstructionBitstream> for InstructionBitstream {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionBitstream> {
                    InstructionBitstream::create(builder, &self.bitstream, &self.field_offsets)
                }
            }

            /// Builder for serializing an instance of the [InstructionBitstream] type.
            ///
            /// Can be created using the [InstructionBitstream::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct InstructionBitstreamBuilder<State>(State);

            impl InstructionBitstreamBuilder<()> {
                /// Setter for the [`bitstream` field](InstructionBitstream#structfield.bitstream).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn bitstream<T0>(self, value: T0) -> InstructionBitstreamBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    InstructionBitstreamBuilder((value,))
                }

                /// Sets the [`bitstream` field](InstructionBitstream#structfield.bitstream) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn bitstream_as_null(self) -> InstructionBitstreamBuilder<((),)> {
                    self.bitstream(())
                }
            }

            impl<T0> InstructionBitstreamBuilder<(T0,)> {
                /// Setter for the [`field_offsets` field](InstructionBitstream#structfield.field_offsets).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn field_offsets<T1>(self, value: T1) -> InstructionBitstreamBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::FieldOffset>]>,
                    >,
                {
                    let (v0,) = self.0;
                    InstructionBitstreamBuilder((v0, value))
                }

                /// Sets the [`field_offsets` field](InstructionBitstream#structfield.field_offsets) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn field_offsets_as_null(self) -> InstructionBitstreamBuilder<(T0, ())> {
                    self.field_offsets(())
                }
            }

            impl<T0, T1> InstructionBitstreamBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [InstructionBitstream].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionBitstream>
                where
                    Self: ::planus::WriteAsOffset<InstructionBitstream>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::FieldOffset>]>,
                    >,
                > ::planus::WriteAs<::planus::Offset<InstructionBitstream>>
                for InstructionBitstreamBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<InstructionBitstream>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionBitstream> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::FieldOffset>]>,
                    >,
                > ::planus::WriteAsOptional<::planus::Offset<InstructionBitstream>>
                for InstructionBitstreamBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<InstructionBitstream>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InstructionBitstream>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::FieldOffset>]>,
                    >,
                > ::planus::WriteAsOffset<InstructionBitstream>
                for InstructionBitstreamBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionBitstream> {
                    let (v0, v1) = &self.0;
                    InstructionBitstream::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [InstructionBitstream].
            #[derive(Copy, Clone)]
            pub struct InstructionBitstreamRef<'a>(
                #[allow(dead_code)] ::planus::table_reader::Table<'a>,
            );

            impl<'a> InstructionBitstreamRef<'a> {
                /// Getter for the [`bitstream` field](InstructionBitstream#structfield.bitstream).
                #[inline]
                pub fn bitstream(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(0, "InstructionBitstream", "bitstream")
                }

                /// Getter for the [`field_offsets` field](InstructionBitstream#structfield.field_offsets).
                #[inline]
                pub fn field_offsets(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::FieldOffsetRef<'a>>>,
                    >,
                > {
                    self.0.access(1, "InstructionBitstream", "field_offsets")
                }
            }

            impl<'a> ::core::fmt::Debug for InstructionBitstreamRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("InstructionBitstreamRef");
                    if let ::core::option::Option::Some(field_bitstream) =
                        self.bitstream().transpose()
                    {
                        f.field("bitstream", &field_bitstream);
                    }
                    if let ::core::option::Option::Some(field_field_offsets) =
                        self.field_offsets().transpose()
                    {
                        f.field("field_offsets", &field_field_offsets);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<InstructionBitstreamRef<'a>> for InstructionBitstream {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: InstructionBitstreamRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        bitstream: value.bitstream()?.map(|v| v.to_vec()),
                        field_offsets: if let ::core::option::Option::Some(field_offsets) =
                            value.field_offsets()?
                        {
                            ::core::option::Option::Some(field_offsets.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for InstructionBitstreamRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for InstructionBitstreamRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[InstructionBitstreamRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<InstructionBitstream>> for InstructionBitstream {
                type Value = ::planus::Offset<InstructionBitstream>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<InstructionBitstream>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for InstructionBitstreamRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[InstructionBitstreamRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The enum `InterruptType` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Enum `InterruptType` in the file `schema/executable.fbs:72`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i16)]
            pub enum InterruptType {
                /// The variant `SCALAR_CORE_INT_0` in the enum `InterruptType`
                ScalarCoreInt0 = 0,

                /// The variant `SCALAR_CORE_INT_1` in the enum `InterruptType`
                ScalarCoreInt1 = 1,

                /// The variant `SCALAR_CORE_INT_2` in the enum `InterruptType`
                ScalarCoreInt2 = 2,

                /// The variant `SCALAR_CORE_INT_3` in the enum `InterruptType`
                ScalarCoreInt3 = 3,
            }

            impl InterruptType {
                /// Array containing all valid variants of InterruptType
                pub const ENUM_VALUES: [Self; 4] = [
                    Self::ScalarCoreInt0,
                    Self::ScalarCoreInt1,
                    Self::ScalarCoreInt2,
                    Self::ScalarCoreInt3,
                ];
            }

            impl ::core::convert::TryFrom<i16> for InterruptType {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i16,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(InterruptType::ScalarCoreInt0),
                        1 => ::core::result::Result::Ok(InterruptType::ScalarCoreInt1),
                        2 => ::core::result::Result::Ok(InterruptType::ScalarCoreInt2),
                        3 => ::core::result::Result::Ok(InterruptType::ScalarCoreInt3),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<InterruptType> for i16 {
                #[inline]
                fn from(value: InterruptType) -> Self {
                    value as i16
                }
            }

            /// # Safety
            /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
            unsafe impl ::planus::Primitive for InterruptType {
                const ALIGNMENT: usize = 2;
                const SIZE: usize = 2;
            }

            impl ::planus::WriteAsPrimitive<InterruptType> for InterruptType {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i16).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<InterruptType> for InterruptType {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> InterruptType {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<InterruptType, InterruptType> for InterruptType {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &InterruptType,
                ) -> ::core::option::Option<InterruptType> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<InterruptType> for InterruptType {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<InterruptType> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for InterruptType {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i16 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for InterruptType {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 2;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value =
                        unsafe { <i16 as ::planus::VectorRead>::from_buffer(buffer, offset) };
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "InterruptType",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<InterruptType> for InterruptType {
                const STRIDE: usize = 2;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 2];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (2 * i) as u32,
                        );
                    }
                }
            }

            /// The enum `Direction` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Enum `Direction` in the file `schema/executable.fbs:81`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i16)]
            pub enum Direction {
                /// The variant `INFEED` in the enum `Direction`
                Infeed = 0,

                /// The variant `OUTFEED` in the enum `Direction`
                Outfeed = 1,
            }

            impl Direction {
                /// Array containing all valid variants of Direction
                pub const ENUM_VALUES: [Self; 2] = [Self::Infeed, Self::Outfeed];
            }

            impl ::core::convert::TryFrom<i16> for Direction {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i16,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(Direction::Infeed),
                        1 => ::core::result::Result::Ok(Direction::Outfeed),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<Direction> for i16 {
                #[inline]
                fn from(value: Direction) -> Self {
                    value as i16
                }
            }

            /// # Safety
            /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
            unsafe impl ::planus::Primitive for Direction {
                const ALIGNMENT: usize = 2;
                const SIZE: usize = 2;
            }

            impl ::planus::WriteAsPrimitive<Direction> for Direction {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i16).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<Direction> for Direction {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Direction {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<Direction, Direction> for Direction {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &Direction,
                ) -> ::core::option::Option<Direction> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<Direction> for Direction {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<Direction> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for Direction {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i16 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for Direction {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 2;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value =
                        unsafe { <i16 as ::planus::VectorRead>::from_buffer(buffer, offset) };
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "Direction",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<Direction> for Direction {
                const STRIDE: usize = 2;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 2];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (2 * i) as u32,
                        );
                    }
                }
            }

            /// The table `DmaDescriptorHint` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `DmaDescriptorHint` in the file `schema/executable.fbs:90`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct DmaDescriptorHint {
                /// The field `meta` in the table `DmaDescriptorHint`
                pub meta: ::core::option::Option<::planus::alloc::boxed::Box<self::Meta>>,
                /// The field `offset_in_bytes` in the table `DmaDescriptorHint`
                pub offset_in_bytes: i32,
                /// The field `size_in_bytes` in the table `DmaDescriptorHint`
                pub size_in_bytes: i32,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for DmaDescriptorHint {
                fn default() -> Self {
                    Self {
                        meta: ::core::default::Default::default(),
                        offset_in_bytes: 0,
                        size_in_bytes: 0,
                    }
                }
            }

            impl DmaDescriptorHint {
                /// Creates a [DmaDescriptorHintBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> DmaDescriptorHintBuilder<()> {
                    DmaDescriptorHintBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_meta: impl ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    field_offset_in_bytes: impl ::planus::WriteAsDefault<i32, i32>,
                    field_size_in_bytes: impl ::planus::WriteAsDefault<i32, i32>,
                ) -> ::planus::Offset<Self> {
                    let prepared_meta = field_meta.prepare(builder);
                    let prepared_offset_in_bytes = field_offset_in_bytes.prepare(builder, &0);
                    let prepared_size_in_bytes = field_size_in_bytes.prepare(builder, &0);

                    let mut table_writer: ::planus::table_writer::TableWriter<10> =
                        ::core::default::Default::default();
                    if prepared_meta.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::Meta>>(0);
                    }
                    if prepared_offset_in_bytes.is_some() {
                        table_writer.write_entry::<i32>(1);
                    }
                    if prepared_size_in_bytes.is_some() {
                        table_writer.write_entry::<i32>(2);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_meta) = prepared_meta {
                                object_writer.write::<_, _, 4>(&prepared_meta);
                            }
                            if let ::core::option::Option::Some(prepared_offset_in_bytes) =
                                prepared_offset_in_bytes
                            {
                                object_writer.write::<_, _, 4>(&prepared_offset_in_bytes);
                            }
                            if let ::core::option::Option::Some(prepared_size_in_bytes) =
                                prepared_size_in_bytes
                            {
                                object_writer.write::<_, _, 4>(&prepared_size_in_bytes);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<DmaDescriptorHint>> for DmaDescriptorHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<DmaDescriptorHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<DmaDescriptorHint>> for DmaDescriptorHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<DmaDescriptorHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<DmaDescriptorHint> for DmaDescriptorHint {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<DmaDescriptorHint> {
                    DmaDescriptorHint::create(
                        builder,
                        &self.meta,
                        self.offset_in_bytes,
                        self.size_in_bytes,
                    )
                }
            }

            /// Builder for serializing an instance of the [DmaDescriptorHint] type.
            ///
            /// Can be created using the [DmaDescriptorHint::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct DmaDescriptorHintBuilder<State>(State);

            impl DmaDescriptorHintBuilder<()> {
                /// Setter for the [`meta` field](DmaDescriptorHint#structfield.meta).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn meta<T0>(self, value: T0) -> DmaDescriptorHintBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                {
                    DmaDescriptorHintBuilder((value,))
                }

                /// Sets the [`meta` field](DmaDescriptorHint#structfield.meta) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn meta_as_null(self) -> DmaDescriptorHintBuilder<((),)> {
                    self.meta(())
                }
            }

            impl<T0> DmaDescriptorHintBuilder<(T0,)> {
                /// Setter for the [`offset_in_bytes` field](DmaDescriptorHint#structfield.offset_in_bytes).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn offset_in_bytes<T1>(self, value: T1) -> DmaDescriptorHintBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0,) = self.0;
                    DmaDescriptorHintBuilder((v0, value))
                }

                /// Sets the [`offset_in_bytes` field](DmaDescriptorHint#structfield.offset_in_bytes) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn offset_in_bytes_as_default(
                    self,
                ) -> DmaDescriptorHintBuilder<(T0, ::planus::DefaultValue)> {
                    self.offset_in_bytes(::planus::DefaultValue)
                }
            }

            impl<T0, T1> DmaDescriptorHintBuilder<(T0, T1)> {
                /// Setter for the [`size_in_bytes` field](DmaDescriptorHint#structfield.size_in_bytes).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn size_in_bytes<T2>(self, value: T2) -> DmaDescriptorHintBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1) = self.0;
                    DmaDescriptorHintBuilder((v0, v1, value))
                }

                /// Sets the [`size_in_bytes` field](DmaDescriptorHint#structfield.size_in_bytes) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn size_in_bytes_as_default(
                    self,
                ) -> DmaDescriptorHintBuilder<(T0, T1, ::planus::DefaultValue)> {
                    self.size_in_bytes(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2> DmaDescriptorHintBuilder<(T0, T1, T2)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [DmaDescriptorHint].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<DmaDescriptorHint>
                where
                    Self: ::planus::WriteAsOffset<DmaDescriptorHint>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAs<::planus::Offset<DmaDescriptorHint>>
                for DmaDescriptorHintBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<DmaDescriptorHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<DmaDescriptorHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAsOptional<::planus::Offset<DmaDescriptorHint>>
                for DmaDescriptorHintBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<DmaDescriptorHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<DmaDescriptorHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::Meta>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAsOffset<DmaDescriptorHint>
                for DmaDescriptorHintBuilder<(T0, T1, T2)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<DmaDescriptorHint> {
                    let (v0, v1, v2) = &self.0;
                    DmaDescriptorHint::create(builder, v0, v1, v2)
                }
            }

            /// Reference to a deserialized [DmaDescriptorHint].
            #[derive(Copy, Clone)]
            pub struct DmaDescriptorHintRef<'a>(
                #[allow(dead_code)] ::planus::table_reader::Table<'a>,
            );

            impl<'a> DmaDescriptorHintRef<'a> {
                /// Getter for the [`meta` field](DmaDescriptorHint#structfield.meta).
                #[inline]
                pub fn meta(&self) -> ::planus::Result<::core::option::Option<self::MetaRef<'a>>> {
                    self.0.access(0, "DmaDescriptorHint", "meta")
                }

                /// Getter for the [`offset_in_bytes` field](DmaDescriptorHint#structfield.offset_in_bytes).
                #[inline]
                pub fn offset_in_bytes(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "DmaDescriptorHint", "offset_in_bytes")?
                            .unwrap_or(0),
                    )
                }

                /// Getter for the [`size_in_bytes` field](DmaDescriptorHint#structfield.size_in_bytes).
                #[inline]
                pub fn size_in_bytes(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(2, "DmaDescriptorHint", "size_in_bytes")?
                            .unwrap_or(0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for DmaDescriptorHintRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("DmaDescriptorHintRef");
                    if let ::core::option::Option::Some(field_meta) = self.meta().transpose() {
                        f.field("meta", &field_meta);
                    }
                    f.field("offset_in_bytes", &self.offset_in_bytes());
                    f.field("size_in_bytes", &self.size_in_bytes());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<DmaDescriptorHintRef<'a>> for DmaDescriptorHint {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: DmaDescriptorHintRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        meta: if let ::core::option::Option::Some(meta) = value.meta()? {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(meta)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        offset_in_bytes: ::core::convert::TryInto::try_into(
                            value.offset_in_bytes()?,
                        )?,
                        size_in_bytes: ::core::convert::TryInto::try_into(value.size_in_bytes()?)?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for DmaDescriptorHintRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for DmaDescriptorHintRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[DmaDescriptorHintRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<DmaDescriptorHint>> for DmaDescriptorHint {
                type Value = ::planus::Offset<DmaDescriptorHint>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<DmaDescriptorHint>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for DmaDescriptorHintRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[DmaDescriptorHintRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `InterruptHint` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `InterruptHint` in the file `schema/executable.fbs:103`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct InterruptHint {
                /// The field `type` in the table `InterruptHint`
                pub type_: self::InterruptType,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for InterruptHint {
                fn default() -> Self {
                    Self {
                        type_: self::InterruptType::ScalarCoreInt0,
                    }
                }
            }

            impl InterruptHint {
                /// Creates a [InterruptHintBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> InterruptHintBuilder<()> {
                    InterruptHintBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_type_: impl ::planus::WriteAsDefault<self::InterruptType, self::InterruptType>,
                ) -> ::planus::Offset<Self> {
                    let prepared_type_ =
                        field_type_.prepare(builder, &self::InterruptType::ScalarCoreInt0);

                    let mut table_writer: ::planus::table_writer::TableWriter<6> =
                        ::core::default::Default::default();
                    if prepared_type_.is_some() {
                        table_writer.write_entry::<self::InterruptType>(0);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_type_) = prepared_type_ {
                                object_writer.write::<_, _, 2>(&prepared_type_);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<InterruptHint>> for InterruptHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InterruptHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<InterruptHint>> for InterruptHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InterruptHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<InterruptHint> for InterruptHint {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InterruptHint> {
                    InterruptHint::create(builder, self.type_)
                }
            }

            /// Builder for serializing an instance of the [InterruptHint] type.
            ///
            /// Can be created using the [InterruptHint::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct InterruptHintBuilder<State>(State);

            impl InterruptHintBuilder<()> {
                /// Setter for the [`type` field](InterruptHint#structfield.type_).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_<T0>(self, value: T0) -> InterruptHintBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<self::InterruptType, self::InterruptType>,
                {
                    InterruptHintBuilder((value,))
                }

                /// Sets the [`type` field](InterruptHint#structfield.type_) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_as_default(self) -> InterruptHintBuilder<(::planus::DefaultValue,)> {
                    self.type_(::planus::DefaultValue)
                }
            }

            impl<T0> InterruptHintBuilder<(T0,)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [InterruptHint].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InterruptHint>
                where
                    Self: ::planus::WriteAsOffset<InterruptHint>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<T0: ::planus::WriteAsDefault<self::InterruptType, self::InterruptType>>
                ::planus::WriteAs<::planus::Offset<InterruptHint>> for InterruptHintBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<InterruptHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InterruptHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<T0: ::planus::WriteAsDefault<self::InterruptType, self::InterruptType>>
                ::planus::WriteAsOptional<::planus::Offset<InterruptHint>>
                for InterruptHintBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<InterruptHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InterruptHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<T0: ::planus::WriteAsDefault<self::InterruptType, self::InterruptType>>
                ::planus::WriteAsOffset<InterruptHint> for InterruptHintBuilder<(T0,)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InterruptHint> {
                    let (v0,) = &self.0;
                    InterruptHint::create(builder, v0)
                }
            }

            /// Reference to a deserialized [InterruptHint].
            #[derive(Copy, Clone)]
            pub struct InterruptHintRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> InterruptHintRef<'a> {
                /// Getter for the [`type` field](InterruptHint#structfield.type_).
                #[inline]
                pub fn type_(&self) -> ::planus::Result<self::InterruptType> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "InterruptHint", "type_")?
                            .unwrap_or(self::InterruptType::ScalarCoreInt0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for InterruptHintRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("InterruptHintRef");
                    f.field("type_", &self.type_());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<InterruptHintRef<'a>> for InterruptHint {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: InterruptHintRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        type_: ::core::convert::TryInto::try_into(value.type_()?)?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for InterruptHintRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for InterruptHintRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[InterruptHintRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<InterruptHint>> for InterruptHint {
                type Value = ::planus::Offset<InterruptHint>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<InterruptHint>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for InterruptHintRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[InterruptHintRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `InstructionHint` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `InstructionHint` in the file `schema/executable.fbs:108`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct InstructionHint {
                /// The field `instruction_chunk_index` in the table `InstructionHint`
                pub instruction_chunk_index: i32,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for InstructionHint {
                fn default() -> Self {
                    Self {
                        instruction_chunk_index: 0,
                    }
                }
            }

            impl InstructionHint {
                /// Creates a [InstructionHintBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> InstructionHintBuilder<()> {
                    InstructionHintBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_instruction_chunk_index: impl ::planus::WriteAsDefault<i32, i32>,
                ) -> ::planus::Offset<Self> {
                    let prepared_instruction_chunk_index =
                        field_instruction_chunk_index.prepare(builder, &0);

                    let mut table_writer: ::planus::table_writer::TableWriter<6> =
                        ::core::default::Default::default();
                    if prepared_instruction_chunk_index.is_some() {
                        table_writer.write_entry::<i32>(0);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_instruction_chunk_index) =
                                prepared_instruction_chunk_index
                            {
                                object_writer.write::<_, _, 4>(&prepared_instruction_chunk_index);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<InstructionHint>> for InstructionHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<InstructionHint>> for InstructionHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InstructionHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<InstructionHint> for InstructionHint {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionHint> {
                    InstructionHint::create(builder, self.instruction_chunk_index)
                }
            }

            /// Builder for serializing an instance of the [InstructionHint] type.
            ///
            /// Can be created using the [InstructionHint::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct InstructionHintBuilder<State>(State);

            impl InstructionHintBuilder<()> {
                /// Setter for the [`instruction_chunk_index` field](InstructionHint#structfield.instruction_chunk_index).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn instruction_chunk_index<T0>(self, value: T0) -> InstructionHintBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<i32, i32>,
                {
                    InstructionHintBuilder((value,))
                }

                /// Sets the [`instruction_chunk_index` field](InstructionHint#structfield.instruction_chunk_index) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn instruction_chunk_index_as_default(
                    self,
                ) -> InstructionHintBuilder<(::planus::DefaultValue,)> {
                    self.instruction_chunk_index(::planus::DefaultValue)
                }
            }

            impl<T0> InstructionHintBuilder<(T0,)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [InstructionHint].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionHint>
                where
                    Self: ::planus::WriteAsOffset<InstructionHint>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<T0: ::planus::WriteAsDefault<i32, i32>>
                ::planus::WriteAs<::planus::Offset<InstructionHint>>
                for InstructionHintBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<InstructionHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<T0: ::planus::WriteAsDefault<i32, i32>>
                ::planus::WriteAsOptional<::planus::Offset<InstructionHint>>
                for InstructionHintBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<InstructionHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InstructionHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<T0: ::planus::WriteAsDefault<i32, i32>> ::planus::WriteAsOffset<InstructionHint>
                for InstructionHintBuilder<(T0,)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<InstructionHint> {
                    let (v0,) = &self.0;
                    InstructionHint::create(builder, v0)
                }
            }

            /// Reference to a deserialized [InstructionHint].
            #[derive(Copy, Clone)]
            pub struct InstructionHintRef<'a>(
                #[allow(dead_code)] ::planus::table_reader::Table<'a>,
            );

            impl<'a> InstructionHintRef<'a> {
                /// Getter for the [`instruction_chunk_index` field](InstructionHint#structfield.instruction_chunk_index).
                #[inline]
                pub fn instruction_chunk_index(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "InstructionHint", "instruction_chunk_index")?
                            .unwrap_or(0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for InstructionHintRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("InstructionHintRef");
                    f.field("instruction_chunk_index", &self.instruction_chunk_index());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<InstructionHintRef<'a>> for InstructionHint {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: InstructionHintRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        instruction_chunk_index: ::core::convert::TryInto::try_into(
                            value.instruction_chunk_index()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for InstructionHintRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for InstructionHintRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[InstructionHintRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<InstructionHint>> for InstructionHint {
                type Value = ::planus::Offset<InstructionHint>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<InstructionHint>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for InstructionHintRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[InstructionHintRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `FenceHint` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `FenceHint` in the file `schema/executable.fbs:115`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct FenceHint {}

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for FenceHint {
                fn default() -> Self {
                    Self {}
                }
            }

            impl FenceHint {
                /// Creates a [FenceHintBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> FenceHintBuilder<()> {
                    FenceHintBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(builder: &mut ::planus::Builder) -> ::planus::Offset<Self> {
                    let table_writer: ::planus::table_writer::TableWriter<4> =
                        ::core::default::Default::default();
                    unsafe {
                        table_writer.finish(builder, |_table_writer| {});
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<FenceHint>> for FenceHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<FenceHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<FenceHint>> for FenceHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<FenceHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<FenceHint> for FenceHint {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<FenceHint> {
                    FenceHint::create(builder)
                }
            }

            /// Builder for serializing an instance of the [FenceHint] type.
            ///
            /// Can be created using the [FenceHint::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct FenceHintBuilder<State>(State);

            impl FenceHintBuilder<()> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [FenceHint].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<FenceHint>
                where
                    Self: ::planus::WriteAsOffset<FenceHint>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl ::planus::WriteAs<::planus::Offset<FenceHint>> for FenceHintBuilder<()> {
                type Prepared = ::planus::Offset<FenceHint>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<FenceHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<FenceHint>> for FenceHintBuilder<()> {
                type Prepared = ::planus::Offset<FenceHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<FenceHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<FenceHint> for FenceHintBuilder<()> {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<FenceHint> {
                    FenceHint::create(builder)
                }
            }

            /// Reference to a deserialized [FenceHint].
            #[derive(Copy, Clone)]
            pub struct FenceHintRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> FenceHintRef<'a> {}

            impl<'a> ::core::fmt::Debug for FenceHintRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("FenceHintRef");

                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<FenceHintRef<'a>> for FenceHint {
                type Error = ::planus::Error;

                fn try_from(_value: FenceHintRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {})
                }
            }

            impl<'a> ::planus::TableRead<'a> for FenceHintRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for FenceHintRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[FenceHintRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<FenceHint>> for FenceHint {
                type Value = ::planus::Offset<FenceHint>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<FenceHint>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for FenceHintRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[FenceHintRef]", "read_as_root", 0)
                    })
                }
            }

            /// The union `AnyHint` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Union `AnyHint` in the file `schema/executable.fbs:119`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub enum AnyHint {
                /// The variant of type `DmaDescriptorHint` in the union `AnyHint`
                DmaDescriptorHint(::planus::alloc::boxed::Box<self::DmaDescriptorHint>),

                /// The variant of type `InstructionHint` in the union `AnyHint`
                InstructionHint(::planus::alloc::boxed::Box<self::InstructionHint>),

                /// The variant of type `InterruptHint` in the union `AnyHint`
                InterruptHint(::planus::alloc::boxed::Box<self::InterruptHint>),

                /// The variant of type `FenceHint` in the union `AnyHint`
                FenceHint(::planus::alloc::boxed::Box<self::FenceHint>),
            }

            impl AnyHint {
                /// Creates a [AnyHintBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> AnyHintBuilder<::planus::Uninitialized> {
                    AnyHintBuilder(::planus::Uninitialized)
                }

                #[inline]
                pub fn create_dma_descriptor_hint(
                    builder: &mut ::planus::Builder,
                    value: impl ::planus::WriteAsOffset<self::DmaDescriptorHint>,
                ) -> ::planus::UnionOffset<Self> {
                    ::planus::UnionOffset::new(1, value.prepare(builder).downcast())
                }

                #[inline]
                pub fn create_instruction_hint(
                    builder: &mut ::planus::Builder,
                    value: impl ::planus::WriteAsOffset<self::InstructionHint>,
                ) -> ::planus::UnionOffset<Self> {
                    ::planus::UnionOffset::new(2, value.prepare(builder).downcast())
                }

                #[inline]
                pub fn create_interrupt_hint(
                    builder: &mut ::planus::Builder,
                    value: impl ::planus::WriteAsOffset<self::InterruptHint>,
                ) -> ::planus::UnionOffset<Self> {
                    ::planus::UnionOffset::new(3, value.prepare(builder).downcast())
                }

                #[inline]
                pub fn create_fence_hint(
                    builder: &mut ::planus::Builder,
                    value: impl ::planus::WriteAsOffset<self::FenceHint>,
                ) -> ::planus::UnionOffset<Self> {
                    ::planus::UnionOffset::new(4, value.prepare(builder).downcast())
                }
            }

            impl ::planus::WriteAsUnion<AnyHint> for AnyHint {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Self> {
                    match self {
                        Self::DmaDescriptorHint(value) => {
                            Self::create_dma_descriptor_hint(builder, value)
                        }
                        Self::InstructionHint(value) => {
                            Self::create_instruction_hint(builder, value)
                        }
                        Self::InterruptHint(value) => Self::create_interrupt_hint(builder, value),
                        Self::FenceHint(value) => Self::create_fence_hint(builder, value),
                    }
                }
            }

            impl ::planus::WriteAsOptionalUnion<AnyHint> for AnyHint {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<Self>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }

            /// Builder for serializing an instance of the [AnyHint] type.
            ///
            /// Can be created using the [AnyHint::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct AnyHintBuilder<T>(T);

            impl AnyHintBuilder<::planus::Uninitialized> {
                /// Creates an instance of the [`DmaDescriptorHint` variant](AnyHint#variant.DmaDescriptorHint).
                #[inline]
                pub fn dma_descriptor_hint<T>(
                    self,
                    value: T,
                ) -> AnyHintBuilder<::planus::Initialized<1, T>>
                where
                    T: ::planus::WriteAsOffset<self::DmaDescriptorHint>,
                {
                    AnyHintBuilder(::planus::Initialized(value))
                }

                /// Creates an instance of the [`InstructionHint` variant](AnyHint#variant.InstructionHint).
                #[inline]
                pub fn instruction_hint<T>(
                    self,
                    value: T,
                ) -> AnyHintBuilder<::planus::Initialized<2, T>>
                where
                    T: ::planus::WriteAsOffset<self::InstructionHint>,
                {
                    AnyHintBuilder(::planus::Initialized(value))
                }

                /// Creates an instance of the [`InterruptHint` variant](AnyHint#variant.InterruptHint).
                #[inline]
                pub fn interrupt_hint<T>(
                    self,
                    value: T,
                ) -> AnyHintBuilder<::planus::Initialized<3, T>>
                where
                    T: ::planus::WriteAsOffset<self::InterruptHint>,
                {
                    AnyHintBuilder(::planus::Initialized(value))
                }

                /// Creates an instance of the [`FenceHint` variant](AnyHint#variant.FenceHint).
                #[inline]
                pub fn fence_hint<T>(self, value: T) -> AnyHintBuilder<::planus::Initialized<4, T>>
                where
                    T: ::planus::WriteAsOffset<self::FenceHint>,
                {
                    AnyHintBuilder(::planus::Initialized(value))
                }
            }

            impl<const N: u8, T> AnyHintBuilder<::planus::Initialized<N, T>> {
                /// Finish writing the builder to get an [UnionOffset](::planus::UnionOffset) to a serialized [AnyHint].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyHint>
                where
                    Self: ::planus::WriteAsUnion<AnyHint>,
                {
                    ::planus::WriteAsUnion::prepare(&self, builder)
                }
            }

            impl<T> ::planus::WriteAsUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<1, T>>
            where
                T: ::planus::WriteAsOffset<self::DmaDescriptorHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyHint> {
                    ::planus::UnionOffset::new(1, (self.0).0.prepare(builder).downcast())
                }
            }

            impl<T> ::planus::WriteAsOptionalUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<1, T>>
            where
                T: ::planus::WriteAsOffset<self::DmaDescriptorHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<AnyHint>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }
            impl<T> ::planus::WriteAsUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<2, T>>
            where
                T: ::planus::WriteAsOffset<self::InstructionHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyHint> {
                    ::planus::UnionOffset::new(2, (self.0).0.prepare(builder).downcast())
                }
            }

            impl<T> ::planus::WriteAsOptionalUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<2, T>>
            where
                T: ::planus::WriteAsOffset<self::InstructionHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<AnyHint>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }
            impl<T> ::planus::WriteAsUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<3, T>>
            where
                T: ::planus::WriteAsOffset<self::InterruptHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyHint> {
                    ::planus::UnionOffset::new(3, (self.0).0.prepare(builder).downcast())
                }
            }

            impl<T> ::planus::WriteAsOptionalUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<3, T>>
            where
                T: ::planus::WriteAsOffset<self::InterruptHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<AnyHint>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }
            impl<T> ::planus::WriteAsUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<4, T>>
            where
                T: ::planus::WriteAsOffset<self::FenceHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyHint> {
                    ::planus::UnionOffset::new(4, (self.0).0.prepare(builder).downcast())
                }
            }

            impl<T> ::planus::WriteAsOptionalUnion<AnyHint> for AnyHintBuilder<::planus::Initialized<4, T>>
            where
                T: ::planus::WriteAsOffset<self::FenceHint>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<AnyHint>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }

            /// Reference to a deserialized [AnyHint].
            #[derive(Copy, Clone, Debug)]
            pub enum AnyHintRef<'a> {
                DmaDescriptorHint(self::DmaDescriptorHintRef<'a>),
                InstructionHint(self::InstructionHintRef<'a>),
                InterruptHint(self::InterruptHintRef<'a>),
                FenceHint(self::FenceHintRef<'a>),
            }

            impl<'a> ::core::convert::TryFrom<AnyHintRef<'a>> for AnyHint {
                type Error = ::planus::Error;

                fn try_from(value: AnyHintRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(match value {
                        AnyHintRef::DmaDescriptorHint(value) => {
                            Self::DmaDescriptorHint(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryFrom::try_from(value)?,
                            ))
                        }

                        AnyHintRef::InstructionHint(value) => {
                            Self::InstructionHint(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryFrom::try_from(value)?,
                            ))
                        }

                        AnyHintRef::InterruptHint(value) => {
                            Self::InterruptHint(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryFrom::try_from(value)?,
                            ))
                        }

                        AnyHintRef::FenceHint(value) => {
                            Self::FenceHint(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryFrom::try_from(value)?,
                            ))
                        }
                    })
                }
            }

            impl<'a> ::planus::TableReadUnion<'a> for AnyHintRef<'a> {
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    tag: u8,
                    field_offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    match tag {
                        1 => ::core::result::Result::Ok(Self::DmaDescriptorHint(
                            ::planus::TableRead::from_buffer(buffer, field_offset)?,
                        )),
                        2 => ::core::result::Result::Ok(Self::InstructionHint(
                            ::planus::TableRead::from_buffer(buffer, field_offset)?,
                        )),
                        3 => ::core::result::Result::Ok(Self::InterruptHint(
                            ::planus::TableRead::from_buffer(buffer, field_offset)?,
                        )),
                        4 => ::core::result::Result::Ok(Self::FenceHint(
                            ::planus::TableRead::from_buffer(buffer, field_offset)?,
                        )),
                        _ => ::core::result::Result::Err(
                            ::planus::errors::ErrorKind::UnknownUnionTag { tag },
                        ),
                    }
                }
            }

            impl<'a> ::planus::VectorReadUnion<'a> for AnyHintRef<'a> {
                const VECTOR_NAME: &'static str = "[AnyHintRef]";
            }

            /// The table `DmaHint` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `DmaHint` in the file `schema/executable.fbs:127`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct DmaHint {
                /// The field `any_hint` in the table `DmaHint`
                pub any_hint: ::core::option::Option<self::AnyHint>,
                /// The field `direction` in the table `DmaHint`
                pub direction: self::Direction,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for DmaHint {
                fn default() -> Self {
                    Self {
                        any_hint: ::core::default::Default::default(),
                        direction: self::Direction::Infeed,
                    }
                }
            }

            impl DmaHint {
                /// Creates a [DmaHintBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> DmaHintBuilder<()> {
                    DmaHintBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_any_hint: impl ::planus::WriteAsOptionalUnion<self::AnyHint>,
                    field_direction: impl ::planus::WriteAsDefault<self::Direction, self::Direction>,
                ) -> ::planus::Offset<Self> {
                    let prepared_any_hint = field_any_hint.prepare(builder);
                    let prepared_direction =
                        field_direction.prepare(builder, &self::Direction::Infeed);

                    let mut table_writer: ::planus::table_writer::TableWriter<10> =
                        ::core::default::Default::default();
                    if prepared_any_hint.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::AnyHint>>(1);
                    }
                    if prepared_direction.is_some() {
                        table_writer.write_entry::<self::Direction>(2);
                    }
                    if prepared_any_hint.is_some() {
                        table_writer.write_entry::<u8>(0);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_any_hint) =
                                prepared_any_hint
                            {
                                object_writer.write::<_, _, 4>(&prepared_any_hint.offset());
                            }
                            if let ::core::option::Option::Some(prepared_direction) =
                                prepared_direction
                            {
                                object_writer.write::<_, _, 2>(&prepared_direction);
                            }
                            if let ::core::option::Option::Some(prepared_any_hint) =
                                prepared_any_hint
                            {
                                object_writer.write::<_, _, 1>(&prepared_any_hint.tag());
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<DmaHint>> for DmaHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<DmaHint>> for DmaHint {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<DmaHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<DmaHint> for DmaHint {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHint> {
                    DmaHint::create(builder, &self.any_hint, self.direction)
                }
            }

            /// Builder for serializing an instance of the [DmaHint] type.
            ///
            /// Can be created using the [DmaHint::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct DmaHintBuilder<State>(State);

            impl DmaHintBuilder<()> {
                /// Setter for the [`any_hint` field](DmaHint#structfield.any_hint).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn any_hint<T0>(self, value: T0) -> DmaHintBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptionalUnion<self::AnyHint>,
                {
                    DmaHintBuilder((value,))
                }

                /// Sets the [`any_hint` field](DmaHint#structfield.any_hint) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn any_hint_as_null(self) -> DmaHintBuilder<((),)> {
                    self.any_hint(())
                }
            }

            impl<T0> DmaHintBuilder<(T0,)> {
                /// Setter for the [`direction` field](DmaHint#structfield.direction).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn direction<T1>(self, value: T1) -> DmaHintBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<self::Direction, self::Direction>,
                {
                    let (v0,) = self.0;
                    DmaHintBuilder((v0, value))
                }

                /// Sets the [`direction` field](DmaHint#structfield.direction) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn direction_as_default(self) -> DmaHintBuilder<(T0, ::planus::DefaultValue)> {
                    self.direction(::planus::DefaultValue)
                }
            }

            impl<T0, T1> DmaHintBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [DmaHint].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHint>
                where
                    Self: ::planus::WriteAsOffset<DmaHint>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptionalUnion<self::AnyHint>,
                    T1: ::planus::WriteAsDefault<self::Direction, self::Direction>,
                > ::planus::WriteAs<::planus::Offset<DmaHint>> for DmaHintBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<DmaHint>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHint> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptionalUnion<self::AnyHint>,
                    T1: ::planus::WriteAsDefault<self::Direction, self::Direction>,
                > ::planus::WriteAsOptional<::planus::Offset<DmaHint>>
                for DmaHintBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<DmaHint>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<DmaHint>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptionalUnion<self::AnyHint>,
                    T1: ::planus::WriteAsDefault<self::Direction, self::Direction>,
                > ::planus::WriteAsOffset<DmaHint> for DmaHintBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHint> {
                    let (v0, v1) = &self.0;
                    DmaHint::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [DmaHint].
            #[derive(Copy, Clone)]
            pub struct DmaHintRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> DmaHintRef<'a> {
                /// Getter for the [`any_hint` field](DmaHint#structfield.any_hint).
                #[inline]
                pub fn any_hint(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::AnyHintRef<'a>>>
                {
                    self.0.access_union(0, "DmaHint", "any_hint")
                }

                /// Getter for the [`direction` field](DmaHint#structfield.direction).
                #[inline]
                pub fn direction(&self) -> ::planus::Result<self::Direction> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(2, "DmaHint", "direction")?
                            .unwrap_or(self::Direction::Infeed),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for DmaHintRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("DmaHintRef");
                    if let ::core::option::Option::Some(field_any_hint) =
                        self.any_hint().transpose()
                    {
                        f.field("any_hint", &field_any_hint);
                    }
                    f.field("direction", &self.direction());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<DmaHintRef<'a>> for DmaHint {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: DmaHintRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        any_hint: if let ::core::option::Option::Some(any_hint) =
                            value.any_hint()?
                        {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(
                                any_hint,
                            )?)
                        } else {
                            ::core::option::Option::None
                        },
                        direction: ::core::convert::TryInto::try_into(value.direction()?)?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for DmaHintRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for DmaHintRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[DmaHintRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<DmaHint>> for DmaHint {
                type Value = ::planus::Offset<DmaHint>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<DmaHint>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for DmaHintRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[DmaHintRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `DmaHints` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `DmaHints` in the file `schema/executable.fbs:135`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct DmaHints {
                /// The field `hints` in the table `DmaHints`
                pub hints: ::core::option::Option<::planus::alloc::vec::Vec<self::DmaHint>>,
                /// The field `fully_deterministic` in the table `DmaHints`
                pub fully_deterministic: bool,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for DmaHints {
                fn default() -> Self {
                    Self {
                        hints: ::core::default::Default::default(),
                        fully_deterministic: false,
                    }
                }
            }

            impl DmaHints {
                /// Creates a [DmaHintsBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> DmaHintsBuilder<()> {
                    DmaHintsBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_hints: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::DmaHint>]>,
                    >,
                    field_fully_deterministic: impl ::planus::WriteAsDefault<bool, bool>,
                ) -> ::planus::Offset<Self> {
                    let prepared_hints = field_hints.prepare(builder);
                    let prepared_fully_deterministic =
                        field_fully_deterministic.prepare(builder, &false);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_hints.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<[::planus::Offset<self::DmaHint>]>>(0);
                    }
                    if prepared_fully_deterministic.is_some() {
                        table_writer.write_entry::<bool>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_hints) = prepared_hints {
                                object_writer.write::<_, _, 4>(&prepared_hints);
                            }
                            if let ::core::option::Option::Some(prepared_fully_deterministic) =
                                prepared_fully_deterministic
                            {
                                object_writer.write::<_, _, 1>(&prepared_fully_deterministic);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<DmaHints>> for DmaHints {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHints> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<DmaHints>> for DmaHints {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<DmaHints>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<DmaHints> for DmaHints {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHints> {
                    DmaHints::create(builder, &self.hints, self.fully_deterministic)
                }
            }

            /// Builder for serializing an instance of the [DmaHints] type.
            ///
            /// Can be created using the [DmaHints::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct DmaHintsBuilder<State>(State);

            impl DmaHintsBuilder<()> {
                /// Setter for the [`hints` field](DmaHints#structfield.hints).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn hints<T0>(self, value: T0) -> DmaHintsBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::DmaHint>]>,
                    >,
                {
                    DmaHintsBuilder((value,))
                }

                /// Sets the [`hints` field](DmaHints#structfield.hints) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn hints_as_null(self) -> DmaHintsBuilder<((),)> {
                    self.hints(())
                }
            }

            impl<T0> DmaHintsBuilder<(T0,)> {
                /// Setter for the [`fully_deterministic` field](DmaHints#structfield.fully_deterministic).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn fully_deterministic<T1>(self, value: T1) -> DmaHintsBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0,) = self.0;
                    DmaHintsBuilder((v0, value))
                }

                /// Sets the [`fully_deterministic` field](DmaHints#structfield.fully_deterministic) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn fully_deterministic_as_default(
                    self,
                ) -> DmaHintsBuilder<(T0, ::planus::DefaultValue)> {
                    self.fully_deterministic(::planus::DefaultValue)
                }
            }

            impl<T0, T1> DmaHintsBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [DmaHints].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHints>
                where
                    Self: ::planus::WriteAsOffset<DmaHints>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::DmaHint>]>>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAs<::planus::Offset<DmaHints>> for DmaHintsBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<DmaHints>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHints> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::DmaHint>]>>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAsOptional<::planus::Offset<DmaHints>>
                for DmaHintsBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<DmaHints>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<DmaHints>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::DmaHint>]>>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAsOffset<DmaHints> for DmaHintsBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<DmaHints> {
                    let (v0, v1) = &self.0;
                    DmaHints::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [DmaHints].
            #[derive(Copy, Clone)]
            pub struct DmaHintsRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> DmaHintsRef<'a> {
                /// Getter for the [`hints` field](DmaHints#structfield.hints).
                #[inline]
                pub fn hints(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::DmaHintRef<'a>>>,
                    >,
                > {
                    self.0.access(0, "DmaHints", "hints")
                }

                /// Getter for the [`fully_deterministic` field](DmaHints#structfield.fully_deterministic).
                #[inline]
                pub fn fully_deterministic(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "DmaHints", "fully_deterministic")?
                            .unwrap_or(false),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for DmaHintsRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("DmaHintsRef");
                    if let ::core::option::Option::Some(field_hints) = self.hints().transpose() {
                        f.field("hints", &field_hints);
                    }
                    f.field("fully_deterministic", &self.fully_deterministic());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<DmaHintsRef<'a>> for DmaHints {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: DmaHintsRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        hints: if let ::core::option::Option::Some(hints) = value.hints()? {
                            ::core::option::Option::Some(hints.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        fully_deterministic: ::core::convert::TryInto::try_into(
                            value.fully_deterministic()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for DmaHintsRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for DmaHintsRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[DmaHintsRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<DmaHints>> for DmaHints {
                type Value = ::planus::Offset<DmaHints>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<DmaHints>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for DmaHintsRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[DmaHintsRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `OutputLayout` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `OutputLayout` in the file `schema/executable.fbs:183`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct OutputLayout {
                /// The field `y_coordinate_to_linear_tile_id_map` in the table `OutputLayout`
                pub y_coordinate_to_linear_tile_id_map:
                    ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
                /// The field `x_coordinate_to_linear_tile_id_map` in the table `OutputLayout`
                pub x_coordinate_to_linear_tile_id_map:
                    ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
                /// The field `linearized_tile_byte_offset` in the table `OutputLayout`
                pub linearized_tile_byte_offset:
                    ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
                /// The field `x_coordinate_to_local_byte_offset` in the table `OutputLayout`
                pub x_coordinate_to_local_byte_offset:
                    ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
                /// The field `y_coordinate_to_local_y_offset` in the table `OutputLayout`
                pub y_coordinate_to_local_y_offset:
                    ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
                /// The field `x_coordinate_to_local_y_row_size` in the table `OutputLayout`
                pub x_coordinate_to_local_y_row_size:
                    ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for OutputLayout {
                fn default() -> Self {
                    Self {
                        y_coordinate_to_linear_tile_id_map: ::core::default::Default::default(),
                        x_coordinate_to_linear_tile_id_map: ::core::default::Default::default(),
                        linearized_tile_byte_offset: ::core::default::Default::default(),
                        x_coordinate_to_local_byte_offset: ::core::default::Default::default(),
                        y_coordinate_to_local_y_offset: ::core::default::Default::default(),
                        x_coordinate_to_local_y_row_size: ::core::default::Default::default(),
                    }
                }
            }

            impl OutputLayout {
                /// Creates a [OutputLayoutBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> OutputLayoutBuilder<()> {
                    OutputLayoutBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_y_coordinate_to_linear_tile_id_map: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[i32]>,
                    >,
                    field_x_coordinate_to_linear_tile_id_map: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[i32]>,
                    >,
                    field_linearized_tile_byte_offset: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[i32]>,
                    >,
                    field_x_coordinate_to_local_byte_offset: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[i32]>,
                    >,
                    field_y_coordinate_to_local_y_offset: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[i32]>,
                    >,
                    field_x_coordinate_to_local_y_row_size: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[i32]>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_y_coordinate_to_linear_tile_id_map =
                        field_y_coordinate_to_linear_tile_id_map.prepare(builder);
                    let prepared_x_coordinate_to_linear_tile_id_map =
                        field_x_coordinate_to_linear_tile_id_map.prepare(builder);
                    let prepared_linearized_tile_byte_offset =
                        field_linearized_tile_byte_offset.prepare(builder);
                    let prepared_x_coordinate_to_local_byte_offset =
                        field_x_coordinate_to_local_byte_offset.prepare(builder);
                    let prepared_y_coordinate_to_local_y_offset =
                        field_y_coordinate_to_local_y_offset.prepare(builder);
                    let prepared_x_coordinate_to_local_y_row_size =
                        field_x_coordinate_to_local_y_row_size.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<16> =
                        ::core::default::Default::default();
                    if prepared_y_coordinate_to_linear_tile_id_map.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(0);
                    }
                    if prepared_x_coordinate_to_linear_tile_id_map.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(1);
                    }
                    if prepared_linearized_tile_byte_offset.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(2);
                    }
                    if prepared_x_coordinate_to_local_byte_offset.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(3);
                    }
                    if prepared_y_coordinate_to_local_y_offset.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(4);
                    }
                    if prepared_x_coordinate_to_local_y_row_size.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(5);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(
                                prepared_y_coordinate_to_linear_tile_id_map,
                            ) = prepared_y_coordinate_to_linear_tile_id_map
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_y_coordinate_to_linear_tile_id_map);
                            }
                            if let ::core::option::Option::Some(
                                prepared_x_coordinate_to_linear_tile_id_map,
                            ) = prepared_x_coordinate_to_linear_tile_id_map
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_x_coordinate_to_linear_tile_id_map);
                            }
                            if let ::core::option::Option::Some(
                                prepared_linearized_tile_byte_offset,
                            ) = prepared_linearized_tile_byte_offset
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_linearized_tile_byte_offset);
                            }
                            if let ::core::option::Option::Some(
                                prepared_x_coordinate_to_local_byte_offset,
                            ) = prepared_x_coordinate_to_local_byte_offset
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_x_coordinate_to_local_byte_offset);
                            }
                            if let ::core::option::Option::Some(
                                prepared_y_coordinate_to_local_y_offset,
                            ) = prepared_y_coordinate_to_local_y_offset
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_y_coordinate_to_local_y_offset);
                            }
                            if let ::core::option::Option::Some(
                                prepared_x_coordinate_to_local_y_row_size,
                            ) = prepared_x_coordinate_to_local_y_row_size
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_x_coordinate_to_local_y_row_size);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<OutputLayout>> for OutputLayout {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayout> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<OutputLayout>> for OutputLayout {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<OutputLayout>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<OutputLayout> for OutputLayout {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayout> {
                    OutputLayout::create(
                        builder,
                        &self.y_coordinate_to_linear_tile_id_map,
                        &self.x_coordinate_to_linear_tile_id_map,
                        &self.linearized_tile_byte_offset,
                        &self.x_coordinate_to_local_byte_offset,
                        &self.y_coordinate_to_local_y_offset,
                        &self.x_coordinate_to_local_y_row_size,
                    )
                }
            }

            /// Builder for serializing an instance of the [OutputLayout] type.
            ///
            /// Can be created using the [OutputLayout::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct OutputLayoutBuilder<State>(State);

            impl OutputLayoutBuilder<()> {
                /// Setter for the [`y_coordinate_to_linear_tile_id_map` field](OutputLayout#structfield.y_coordinate_to_linear_tile_id_map).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn y_coordinate_to_linear_tile_id_map<T0>(
                    self,
                    value: T0,
                ) -> OutputLayoutBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    OutputLayoutBuilder((value,))
                }

                /// Sets the [`y_coordinate_to_linear_tile_id_map` field](OutputLayout#structfield.y_coordinate_to_linear_tile_id_map) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn y_coordinate_to_linear_tile_id_map_as_null(
                    self,
                ) -> OutputLayoutBuilder<((),)> {
                    self.y_coordinate_to_linear_tile_id_map(())
                }
            }

            impl<T0> OutputLayoutBuilder<(T0,)> {
                /// Setter for the [`x_coordinate_to_linear_tile_id_map` field](OutputLayout#structfield.x_coordinate_to_linear_tile_id_map).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_coordinate_to_linear_tile_id_map<T1>(
                    self,
                    value: T1,
                ) -> OutputLayoutBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    let (v0,) = self.0;
                    OutputLayoutBuilder((v0, value))
                }

                /// Sets the [`x_coordinate_to_linear_tile_id_map` field](OutputLayout#structfield.x_coordinate_to_linear_tile_id_map) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_coordinate_to_linear_tile_id_map_as_null(
                    self,
                ) -> OutputLayoutBuilder<(T0, ())> {
                    self.x_coordinate_to_linear_tile_id_map(())
                }
            }

            impl<T0, T1> OutputLayoutBuilder<(T0, T1)> {
                /// Setter for the [`linearized_tile_byte_offset` field](OutputLayout#structfield.linearized_tile_byte_offset).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn linearized_tile_byte_offset<T2>(
                    self,
                    value: T2,
                ) -> OutputLayoutBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    let (v0, v1) = self.0;
                    OutputLayoutBuilder((v0, v1, value))
                }

                /// Sets the [`linearized_tile_byte_offset` field](OutputLayout#structfield.linearized_tile_byte_offset) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn linearized_tile_byte_offset_as_null(
                    self,
                ) -> OutputLayoutBuilder<(T0, T1, ())> {
                    self.linearized_tile_byte_offset(())
                }
            }

            impl<T0, T1, T2> OutputLayoutBuilder<(T0, T1, T2)> {
                /// Setter for the [`x_coordinate_to_local_byte_offset` field](OutputLayout#structfield.x_coordinate_to_local_byte_offset).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_coordinate_to_local_byte_offset<T3>(
                    self,
                    value: T3,
                ) -> OutputLayoutBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    let (v0, v1, v2) = self.0;
                    OutputLayoutBuilder((v0, v1, v2, value))
                }

                /// Sets the [`x_coordinate_to_local_byte_offset` field](OutputLayout#structfield.x_coordinate_to_local_byte_offset) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_coordinate_to_local_byte_offset_as_null(
                    self,
                ) -> OutputLayoutBuilder<(T0, T1, T2, ())> {
                    self.x_coordinate_to_local_byte_offset(())
                }
            }

            impl<T0, T1, T2, T3> OutputLayoutBuilder<(T0, T1, T2, T3)> {
                /// Setter for the [`y_coordinate_to_local_y_offset` field](OutputLayout#structfield.y_coordinate_to_local_y_offset).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn y_coordinate_to_local_y_offset<T4>(
                    self,
                    value: T4,
                ) -> OutputLayoutBuilder<(T0, T1, T2, T3, T4)>
                where
                    T4: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    let (v0, v1, v2, v3) = self.0;
                    OutputLayoutBuilder((v0, v1, v2, v3, value))
                }

                /// Sets the [`y_coordinate_to_local_y_offset` field](OutputLayout#structfield.y_coordinate_to_local_y_offset) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn y_coordinate_to_local_y_offset_as_null(
                    self,
                ) -> OutputLayoutBuilder<(T0, T1, T2, T3, ())> {
                    self.y_coordinate_to_local_y_offset(())
                }
            }

            impl<T0, T1, T2, T3, T4> OutputLayoutBuilder<(T0, T1, T2, T3, T4)> {
                /// Setter for the [`x_coordinate_to_local_y_row_size` field](OutputLayout#structfield.x_coordinate_to_local_y_row_size).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_coordinate_to_local_y_row_size<T5>(
                    self,
                    value: T5,
                ) -> OutputLayoutBuilder<(T0, T1, T2, T3, T4, T5)>
                where
                    T5: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    let (v0, v1, v2, v3, v4) = self.0;
                    OutputLayoutBuilder((v0, v1, v2, v3, v4, value))
                }

                /// Sets the [`x_coordinate_to_local_y_row_size` field](OutputLayout#structfield.x_coordinate_to_local_y_row_size) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_coordinate_to_local_y_row_size_as_null(
                    self,
                ) -> OutputLayoutBuilder<(T0, T1, T2, T3, T4, ())> {
                    self.x_coordinate_to_local_y_row_size(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5> OutputLayoutBuilder<(T0, T1, T2, T3, T4, T5)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [OutputLayout].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayout>
                where
                    Self: ::planus::WriteAsOffset<OutputLayout>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T3: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T4: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T5: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAs<::planus::Offset<OutputLayout>>
                for OutputLayoutBuilder<(T0, T1, T2, T3, T4, T5)>
            {
                type Prepared = ::planus::Offset<OutputLayout>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayout> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T3: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T4: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T5: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAsOptional<::planus::Offset<OutputLayout>>
                for OutputLayoutBuilder<(T0, T1, T2, T3, T4, T5)>
            {
                type Prepared = ::planus::Offset<OutputLayout>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<OutputLayout>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T3: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T4: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                    T5: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAsOffset<OutputLayout>
                for OutputLayoutBuilder<(T0, T1, T2, T3, T4, T5)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayout> {
                    let (v0, v1, v2, v3, v4, v5) = &self.0;
                    OutputLayout::create(builder, v0, v1, v2, v3, v4, v5)
                }
            }

            /// Reference to a deserialized [OutputLayout].
            #[derive(Copy, Clone)]
            pub struct OutputLayoutRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> OutputLayoutRef<'a> {
                /// Getter for the [`y_coordinate_to_linear_tile_id_map` field](OutputLayout#structfield.y_coordinate_to_linear_tile_id_map).
                #[inline]
                pub fn y_coordinate_to_linear_tile_id_map(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0
                        .access(0, "OutputLayout", "y_coordinate_to_linear_tile_id_map")
                }

                /// Getter for the [`x_coordinate_to_linear_tile_id_map` field](OutputLayout#structfield.x_coordinate_to_linear_tile_id_map).
                #[inline]
                pub fn x_coordinate_to_linear_tile_id_map(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0
                        .access(1, "OutputLayout", "x_coordinate_to_linear_tile_id_map")
                }

                /// Getter for the [`linearized_tile_byte_offset` field](OutputLayout#structfield.linearized_tile_byte_offset).
                #[inline]
                pub fn linearized_tile_byte_offset(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0
                        .access(2, "OutputLayout", "linearized_tile_byte_offset")
                }

                /// Getter for the [`x_coordinate_to_local_byte_offset` field](OutputLayout#structfield.x_coordinate_to_local_byte_offset).
                #[inline]
                pub fn x_coordinate_to_local_byte_offset(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0
                        .access(3, "OutputLayout", "x_coordinate_to_local_byte_offset")
                }

                /// Getter for the [`y_coordinate_to_local_y_offset` field](OutputLayout#structfield.y_coordinate_to_local_y_offset).
                #[inline]
                pub fn y_coordinate_to_local_y_offset(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0
                        .access(4, "OutputLayout", "y_coordinate_to_local_y_offset")
                }

                /// Getter for the [`x_coordinate_to_local_y_row_size` field](OutputLayout#structfield.x_coordinate_to_local_y_row_size).
                #[inline]
                pub fn x_coordinate_to_local_y_row_size(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0
                        .access(5, "OutputLayout", "x_coordinate_to_local_y_row_size")
                }
            }

            impl<'a> ::core::fmt::Debug for OutputLayoutRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("OutputLayoutRef");
                    if let ::core::option::Option::Some(field_y_coordinate_to_linear_tile_id_map) =
                        self.y_coordinate_to_linear_tile_id_map().transpose()
                    {
                        f.field(
                            "y_coordinate_to_linear_tile_id_map",
                            &field_y_coordinate_to_linear_tile_id_map,
                        );
                    }
                    if let ::core::option::Option::Some(field_x_coordinate_to_linear_tile_id_map) =
                        self.x_coordinate_to_linear_tile_id_map().transpose()
                    {
                        f.field(
                            "x_coordinate_to_linear_tile_id_map",
                            &field_x_coordinate_to_linear_tile_id_map,
                        );
                    }
                    if let ::core::option::Option::Some(field_linearized_tile_byte_offset) =
                        self.linearized_tile_byte_offset().transpose()
                    {
                        f.field(
                            "linearized_tile_byte_offset",
                            &field_linearized_tile_byte_offset,
                        );
                    }
                    if let ::core::option::Option::Some(field_x_coordinate_to_local_byte_offset) =
                        self.x_coordinate_to_local_byte_offset().transpose()
                    {
                        f.field(
                            "x_coordinate_to_local_byte_offset",
                            &field_x_coordinate_to_local_byte_offset,
                        );
                    }
                    if let ::core::option::Option::Some(field_y_coordinate_to_local_y_offset) =
                        self.y_coordinate_to_local_y_offset().transpose()
                    {
                        f.field(
                            "y_coordinate_to_local_y_offset",
                            &field_y_coordinate_to_local_y_offset,
                        );
                    }
                    if let ::core::option::Option::Some(field_x_coordinate_to_local_y_row_size) =
                        self.x_coordinate_to_local_y_row_size().transpose()
                    {
                        f.field(
                            "x_coordinate_to_local_y_row_size",
                            &field_x_coordinate_to_local_y_row_size,
                        );
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<OutputLayoutRef<'a>> for OutputLayout {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: OutputLayoutRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        y_coordinate_to_linear_tile_id_map: if let ::core::option::Option::Some(
                            y_coordinate_to_linear_tile_id_map,
                        ) =
                            value.y_coordinate_to_linear_tile_id_map()?
                        {
                            ::core::option::Option::Some(
                                y_coordinate_to_linear_tile_id_map.to_vec()?,
                            )
                        } else {
                            ::core::option::Option::None
                        },
                        x_coordinate_to_linear_tile_id_map: if let ::core::option::Option::Some(
                            x_coordinate_to_linear_tile_id_map,
                        ) =
                            value.x_coordinate_to_linear_tile_id_map()?
                        {
                            ::core::option::Option::Some(
                                x_coordinate_to_linear_tile_id_map.to_vec()?,
                            )
                        } else {
                            ::core::option::Option::None
                        },
                        linearized_tile_byte_offset: if let ::core::option::Option::Some(
                            linearized_tile_byte_offset,
                        ) = value.linearized_tile_byte_offset()?
                        {
                            ::core::option::Option::Some(linearized_tile_byte_offset.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                        x_coordinate_to_local_byte_offset: if let ::core::option::Option::Some(
                            x_coordinate_to_local_byte_offset,
                        ) =
                            value.x_coordinate_to_local_byte_offset()?
                        {
                            ::core::option::Option::Some(
                                x_coordinate_to_local_byte_offset.to_vec()?,
                            )
                        } else {
                            ::core::option::Option::None
                        },
                        y_coordinate_to_local_y_offset: if let ::core::option::Option::Some(
                            y_coordinate_to_local_y_offset,
                        ) =
                            value.y_coordinate_to_local_y_offset()?
                        {
                            ::core::option::Option::Some(y_coordinate_to_local_y_offset.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                        x_coordinate_to_local_y_row_size: if let ::core::option::Option::Some(
                            x_coordinate_to_local_y_row_size,
                        ) =
                            value.x_coordinate_to_local_y_row_size()?
                        {
                            ::core::option::Option::Some(x_coordinate_to_local_y_row_size.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for OutputLayoutRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for OutputLayoutRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[OutputLayoutRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<OutputLayout>> for OutputLayout {
                type Value = ::planus::Offset<OutputLayout>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<OutputLayout>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for OutputLayoutRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[OutputLayoutRef]", "read_as_root", 0)
                    })
                }
            }

            /// The struct `Range` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Struct `Range` in the file `schema/executable.fbs:208`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                Default,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct Range {
                /// The field `start` in the struct `Range`
                pub start: i32,

                /// The field `end` in the struct `Range`
                pub end: i32,
            }

            /// # Safety
            /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
            unsafe impl ::planus::Primitive for Range {
                const ALIGNMENT: usize = 4;
                const SIZE: usize = 8;
            }

            #[allow(clippy::identity_op)]
            impl ::planus::WriteAsPrimitive<Range> for Range {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    let (cur, cursor) = cursor.split::<4, 4>();
                    self.start.write(cur, buffer_position - 0);
                    let (cur, cursor) = cursor.split::<4, 0>();
                    self.end.write(cur, buffer_position - 4);
                    cursor.finish([]);
                }
            }

            impl ::planus::WriteAsOffset<Range> for Range {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Range> {
                    unsafe {
                        builder.write_with(8, 3, |buffer_position, bytes| {
                            let bytes = bytes.as_mut_ptr();

                            ::planus::WriteAsPrimitive::write(
                                self,
                                ::planus::Cursor::new(
                                    &mut *(bytes as *mut [::core::mem::MaybeUninit<u8>; 8]),
                                ),
                                buffer_position,
                            );
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<Range> for Range {
                type Prepared = Self;
                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }
            }

            impl ::planus::WriteAsOptional<Range> for Range {
                type Prepared = Self;
                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<Self> {
                    ::core::option::Option::Some(*self)
                }
            }

            /// Reference to a deserialized [Range].
            #[derive(Copy, Clone)]
            pub struct RangeRef<'a>(::planus::ArrayWithStartOffset<'a, 8>);

            impl<'a> RangeRef<'a> {
                /// Getter for the [`start` field](Range#structfield.start).
                pub fn start(&self) -> i32 {
                    let buffer = self.0.advance_as_array::<4>(0).unwrap();

                    i32::from_le_bytes(*buffer.as_array())
                }

                /// Getter for the [`end` field](Range#structfield.end).
                pub fn end(&self) -> i32 {
                    let buffer = self.0.advance_as_array::<4>(4).unwrap();

                    i32::from_le_bytes(*buffer.as_array())
                }
            }

            impl<'a> ::core::fmt::Debug for RangeRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("RangeRef");
                    f.field("start", &self.start());
                    f.field("end", &self.end());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::From<::planus::ArrayWithStartOffset<'a, 8>> for RangeRef<'a> {
                fn from(array: ::planus::ArrayWithStartOffset<'a, 8>) -> Self {
                    Self(array)
                }
            }

            impl<'a> ::core::convert::From<RangeRef<'a>> for Range {
                #[allow(unreachable_code)]
                fn from(value: RangeRef<'a>) -> Self {
                    Self {
                        start: value.start(),
                        end: value.end(),
                    }
                }
            }

            impl<'a, 'b> ::core::cmp::PartialEq<RangeRef<'a>> for RangeRef<'b> {
                fn eq(&self, other: &RangeRef<'_>) -> bool {
                    self.start() == other.start() && self.end() == other.end()
                }
            }

            impl<'a> ::core::cmp::Eq for RangeRef<'a> {}
            impl<'a, 'b> ::core::cmp::PartialOrd<RangeRef<'a>> for RangeRef<'b> {
                fn partial_cmp(
                    &self,
                    other: &RangeRef<'_>,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::option::Option::Some(::core::cmp::Ord::cmp(self, other))
                }
            }

            impl<'a> ::core::cmp::Ord for RangeRef<'a> {
                fn cmp(&self, other: &RangeRef<'_>) -> ::core::cmp::Ordering {
                    self.start()
                        .cmp(&other.start())
                        .then_with(|| self.end().cmp(&other.end()))
                }
            }

            impl<'a> ::core::hash::Hash for RangeRef<'a> {
                fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                    self.start().hash(state);
                    self.end().hash(state);
                }
            }

            impl<'a> ::planus::TableRead<'a> for RangeRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let buffer = buffer.advance_as_array::<8>(offset)?;
                    ::core::result::Result::Ok(Self(buffer))
                }
            }

            impl<'a> ::planus::VectorRead<'a> for RangeRef<'a> {
                const STRIDE: usize = 8;

                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> Self {
                    Self(unsafe { buffer.unchecked_advance_as_array(offset) })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<Range> for Range {
                const STRIDE: usize = 8;

                type Value = Range;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self::Value {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Range],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 8];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (8 * i) as u32,
                        );
                    }
                }
            }

            /// The table `TensorShape` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `TensorShape` in the file `schema/executable.fbs:214`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct TensorShape {
                /// The field `dimension` in the table `TensorShape`
                pub dimension: ::core::option::Option<::planus::alloc::vec::Vec<self::Range>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for TensorShape {
                fn default() -> Self {
                    Self {
                        dimension: ::core::default::Default::default(),
                    }
                }
            }

            impl TensorShape {
                /// Creates a [TensorShapeBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> TensorShapeBuilder<()> {
                    TensorShapeBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_dimension: impl ::planus::WriteAsOptional<::planus::Offset<[self::Range]>>,
                ) -> ::planus::Offset<Self> {
                    let prepared_dimension = field_dimension.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<6> =
                        ::core::default::Default::default();
                    if prepared_dimension.is_some() {
                        table_writer.write_entry::<::planus::Offset<[self::Range]>>(0);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_dimension) =
                                prepared_dimension
                            {
                                object_writer.write::<_, _, 4>(&prepared_dimension);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<TensorShape>> for TensorShape {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorShape> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<TensorShape>> for TensorShape {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<TensorShape>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<TensorShape> for TensorShape {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorShape> {
                    TensorShape::create(builder, &self.dimension)
                }
            }

            /// Builder for serializing an instance of the [TensorShape] type.
            ///
            /// Can be created using the [TensorShape::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct TensorShapeBuilder<State>(State);

            impl TensorShapeBuilder<()> {
                /// Setter for the [`dimension` field](TensorShape#structfield.dimension).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn dimension<T0>(self, value: T0) -> TensorShapeBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[self::Range]>>,
                {
                    TensorShapeBuilder((value,))
                }

                /// Sets the [`dimension` field](TensorShape#structfield.dimension) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn dimension_as_null(self) -> TensorShapeBuilder<((),)> {
                    self.dimension(())
                }
            }

            impl<T0> TensorShapeBuilder<(T0,)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [TensorShape].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorShape>
                where
                    Self: ::planus::WriteAsOffset<TensorShape>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[self::Range]>>>
                ::planus::WriteAs<::planus::Offset<TensorShape>> for TensorShapeBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<TensorShape>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorShape> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[self::Range]>>>
                ::planus::WriteAsOptional<::planus::Offset<TensorShape>>
                for TensorShapeBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<TensorShape>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<TensorShape>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[self::Range]>>>
                ::planus::WriteAsOffset<TensorShape> for TensorShapeBuilder<(T0,)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorShape> {
                    let (v0,) = &self.0;
                    TensorShape::create(builder, v0)
                }
            }

            /// Reference to a deserialized [TensorShape].
            #[derive(Copy, Clone)]
            pub struct TensorShapeRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> TensorShapeRef<'a> {
                /// Getter for the [`dimension` field](TensorShape#structfield.dimension).
                #[inline]
                pub fn dimension(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<::planus::Vector<'a, self::RangeRef<'a>>>,
                > {
                    self.0.access(0, "TensorShape", "dimension")
                }
            }

            impl<'a> ::core::fmt::Debug for TensorShapeRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("TensorShapeRef");
                    if let ::core::option::Option::Some(field_dimension) =
                        self.dimension().transpose()
                    {
                        f.field("dimension", &field_dimension);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<TensorShapeRef<'a>> for TensorShape {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: TensorShapeRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        dimension: if let ::core::option::Option::Some(dimension) =
                            value.dimension()?
                        {
                            ::core::option::Option::Some(dimension.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for TensorShapeRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for TensorShapeRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[TensorShapeRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<TensorShape>> for TensorShape {
                type Value = ::planus::Offset<TensorShape>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<TensorShape>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for TensorShapeRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[TensorShapeRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `TensorLayout` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `TensorLayout` in the file `schema/executable.fbs:221`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct TensorLayout {
                /// The field `shape` in the table `TensorLayout`
                pub shape: ::core::option::Option<::planus::alloc::boxed::Box<self::TensorShape>>,
                /// The field `stride` in the table `TensorLayout`
                pub stride: ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for TensorLayout {
                fn default() -> Self {
                    Self {
                        shape: ::core::default::Default::default(),
                        stride: ::core::default::Default::default(),
                    }
                }
            }

            impl TensorLayout {
                /// Creates a [TensorLayoutBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> TensorLayoutBuilder<()> {
                    TensorLayoutBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_shape: impl ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                    field_stride: impl ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                ) -> ::planus::Offset<Self> {
                    let prepared_shape = field_shape.prepare(builder);
                    let prepared_stride = field_stride.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_shape.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::TensorShape>>(0);
                    }
                    if prepared_stride.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_shape) = prepared_shape {
                                object_writer.write::<_, _, 4>(&prepared_shape);
                            }
                            if let ::core::option::Option::Some(prepared_stride) = prepared_stride {
                                object_writer.write::<_, _, 4>(&prepared_stride);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<TensorLayout>> for TensorLayout {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorLayout> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<TensorLayout>> for TensorLayout {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<TensorLayout>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<TensorLayout> for TensorLayout {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorLayout> {
                    TensorLayout::create(builder, &self.shape, &self.stride)
                }
            }

            /// Builder for serializing an instance of the [TensorLayout] type.
            ///
            /// Can be created using the [TensorLayout::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct TensorLayoutBuilder<State>(State);

            impl TensorLayoutBuilder<()> {
                /// Setter for the [`shape` field](TensorLayout#structfield.shape).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn shape<T0>(self, value: T0) -> TensorLayoutBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                {
                    TensorLayoutBuilder((value,))
                }

                /// Sets the [`shape` field](TensorLayout#structfield.shape) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn shape_as_null(self) -> TensorLayoutBuilder<((),)> {
                    self.shape(())
                }
            }

            impl<T0> TensorLayoutBuilder<(T0,)> {
                /// Setter for the [`stride` field](TensorLayout#structfield.stride).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn stride<T1>(self, value: T1) -> TensorLayoutBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    let (v0,) = self.0;
                    TensorLayoutBuilder((v0, value))
                }

                /// Sets the [`stride` field](TensorLayout#structfield.stride) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn stride_as_null(self) -> TensorLayoutBuilder<(T0, ())> {
                    self.stride(())
                }
            }

            impl<T0, T1> TensorLayoutBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [TensorLayout].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorLayout>
                where
                    Self: ::planus::WriteAsOffset<TensorLayout>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAs<::planus::Offset<TensorLayout>>
                for TensorLayoutBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<TensorLayout>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorLayout> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAsOptional<::planus::Offset<TensorLayout>>
                for TensorLayoutBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<TensorLayout>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<TensorLayout>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAsOffset<TensorLayout> for TensorLayoutBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<TensorLayout> {
                    let (v0, v1) = &self.0;
                    TensorLayout::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [TensorLayout].
            #[derive(Copy, Clone)]
            pub struct TensorLayoutRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> TensorLayoutRef<'a> {
                /// Getter for the [`shape` field](TensorLayout#structfield.shape).
                #[inline]
                pub fn shape(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::TensorShapeRef<'a>>>
                {
                    self.0.access(0, "TensorLayout", "shape")
                }

                /// Getter for the [`stride` field](TensorLayout#structfield.stride).
                #[inline]
                pub fn stride(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0.access(1, "TensorLayout", "stride")
                }
            }

            impl<'a> ::core::fmt::Debug for TensorLayoutRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("TensorLayoutRef");
                    if let ::core::option::Option::Some(field_shape) = self.shape().transpose() {
                        f.field("shape", &field_shape);
                    }
                    if let ::core::option::Option::Some(field_stride) = self.stride().transpose() {
                        f.field("stride", &field_stride);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<TensorLayoutRef<'a>> for TensorLayout {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: TensorLayoutRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        shape: if let ::core::option::Option::Some(shape) = value.shape()? {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(shape)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        stride: if let ::core::option::Option::Some(stride) = value.stride()? {
                            ::core::option::Option::Some(stride.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for TensorLayoutRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for TensorLayoutRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[TensorLayoutRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<TensorLayout>> for TensorLayout {
                type Value = ::planus::Offset<TensorLayout>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<TensorLayout>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for TensorLayoutRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[TensorLayoutRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `OutputShapeInfo` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `OutputShapeInfo` in the file `schema/executable.fbs:232`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct OutputShapeInfo {
                /// The field `slice_layout` in the table `OutputShapeInfo`
                pub slice_layout:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::TensorLayout>>,
                /// The field `slice_offset` in the table `OutputShapeInfo`
                pub slice_offset: ::core::option::Option<::planus::alloc::vec::Vec<i32>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for OutputShapeInfo {
                fn default() -> Self {
                    Self {
                        slice_layout: ::core::default::Default::default(),
                        slice_offset: ::core::default::Default::default(),
                    }
                }
            }

            impl OutputShapeInfo {
                /// Creates a [OutputShapeInfoBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> OutputShapeInfoBuilder<()> {
                    OutputShapeInfoBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_slice_layout: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::TensorLayout>]>,
                    >,
                    field_slice_offset: impl ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                ) -> ::planus::Offset<Self> {
                    let prepared_slice_layout = field_slice_layout.prepare(builder);
                    let prepared_slice_offset = field_slice_offset.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_slice_layout.is_some() {
                        table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::TensorLayout>]>>(0);
                    }
                    if prepared_slice_offset.is_some() {
                        table_writer.write_entry::<::planus::Offset<[i32]>>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_slice_layout) =
                                prepared_slice_layout
                            {
                                object_writer.write::<_, _, 4>(&prepared_slice_layout);
                            }
                            if let ::core::option::Option::Some(prepared_slice_offset) =
                                prepared_slice_offset
                            {
                                object_writer.write::<_, _, 4>(&prepared_slice_offset);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<OutputShapeInfo>> for OutputShapeInfo {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputShapeInfo> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<OutputShapeInfo>> for OutputShapeInfo {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<OutputShapeInfo>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<OutputShapeInfo> for OutputShapeInfo {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputShapeInfo> {
                    OutputShapeInfo::create(builder, &self.slice_layout, &self.slice_offset)
                }
            }

            /// Builder for serializing an instance of the [OutputShapeInfo] type.
            ///
            /// Can be created using the [OutputShapeInfo::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct OutputShapeInfoBuilder<State>(State);

            impl OutputShapeInfoBuilder<()> {
                /// Setter for the [`slice_layout` field](OutputShapeInfo#structfield.slice_layout).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn slice_layout<T0>(self, value: T0) -> OutputShapeInfoBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::TensorLayout>]>,
                    >,
                {
                    OutputShapeInfoBuilder((value,))
                }

                /// Sets the [`slice_layout` field](OutputShapeInfo#structfield.slice_layout) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn slice_layout_as_null(self) -> OutputShapeInfoBuilder<((),)> {
                    self.slice_layout(())
                }
            }

            impl<T0> OutputShapeInfoBuilder<(T0,)> {
                /// Setter for the [`slice_offset` field](OutputShapeInfo#structfield.slice_offset).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn slice_offset<T1>(self, value: T1) -> OutputShapeInfoBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                {
                    let (v0,) = self.0;
                    OutputShapeInfoBuilder((v0, value))
                }

                /// Sets the [`slice_offset` field](OutputShapeInfo#structfield.slice_offset) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn slice_offset_as_null(self) -> OutputShapeInfoBuilder<(T0, ())> {
                    self.slice_offset(())
                }
            }

            impl<T0, T1> OutputShapeInfoBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [OutputShapeInfo].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputShapeInfo>
                where
                    Self: ::planus::WriteAsOffset<OutputShapeInfo>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::TensorLayout>]>,
                    >,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAs<::planus::Offset<OutputShapeInfo>>
                for OutputShapeInfoBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<OutputShapeInfo>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputShapeInfo> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::TensorLayout>]>,
                    >,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAsOptional<::planus::Offset<OutputShapeInfo>>
                for OutputShapeInfoBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<OutputShapeInfo>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<OutputShapeInfo>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::TensorLayout>]>,
                    >,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[i32]>>,
                > ::planus::WriteAsOffset<OutputShapeInfo> for OutputShapeInfoBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputShapeInfo> {
                    let (v0, v1) = &self.0;
                    OutputShapeInfo::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [OutputShapeInfo].
            #[derive(Copy, Clone)]
            pub struct OutputShapeInfoRef<'a>(
                #[allow(dead_code)] ::planus::table_reader::Table<'a>,
            );

            impl<'a> OutputShapeInfoRef<'a> {
                /// Getter for the [`slice_layout` field](OutputShapeInfo#structfield.slice_layout).
                #[inline]
                pub fn slice_layout(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::TensorLayoutRef<'a>>>,
                    >,
                > {
                    self.0.access(0, "OutputShapeInfo", "slice_layout")
                }

                /// Getter for the [`slice_offset` field](OutputShapeInfo#structfield.slice_offset).
                #[inline]
                pub fn slice_offset(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, i32>>>
                {
                    self.0.access(1, "OutputShapeInfo", "slice_offset")
                }
            }

            impl<'a> ::core::fmt::Debug for OutputShapeInfoRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("OutputShapeInfoRef");
                    if let ::core::option::Option::Some(field_slice_layout) =
                        self.slice_layout().transpose()
                    {
                        f.field("slice_layout", &field_slice_layout);
                    }
                    if let ::core::option::Option::Some(field_slice_offset) =
                        self.slice_offset().transpose()
                    {
                        f.field("slice_offset", &field_slice_offset);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<OutputShapeInfoRef<'a>> for OutputShapeInfo {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: OutputShapeInfoRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        slice_layout: if let ::core::option::Option::Some(slice_layout) =
                            value.slice_layout()?
                        {
                            ::core::option::Option::Some(slice_layout.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        slice_offset: if let ::core::option::Option::Some(slice_offset) =
                            value.slice_offset()?
                        {
                            ::core::option::Option::Some(slice_offset.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for OutputShapeInfoRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for OutputShapeInfoRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[OutputShapeInfoRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<OutputShapeInfo>> for OutputShapeInfo {
                type Value = ::planus::Offset<OutputShapeInfo>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<OutputShapeInfo>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for OutputShapeInfoRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[OutputShapeInfoRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `NumericsConstants` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `NumericsConstants` in the file `schema/executable.fbs:243`
            #[derive(
                Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize,
            )]
            pub struct NumericsConstants {
                /// The field `zero_point` in the table `NumericsConstants`
                pub zero_point: i32,
                /// The field `dequantization_factor` in the table `NumericsConstants`
                pub dequantization_factor: f32,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for NumericsConstants {
                fn default() -> Self {
                    Self {
                        zero_point: 0,
                        dequantization_factor: 0.0,
                    }
                }
            }

            impl NumericsConstants {
                /// Creates a [NumericsConstantsBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> NumericsConstantsBuilder<()> {
                    NumericsConstantsBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_zero_point: impl ::planus::WriteAsDefault<i32, i32>,
                    field_dequantization_factor: impl ::planus::WriteAsDefault<f32, f32>,
                ) -> ::planus::Offset<Self> {
                    let prepared_zero_point = field_zero_point.prepare(builder, &0);
                    let prepared_dequantization_factor =
                        field_dequantization_factor.prepare(builder, &0.0);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_zero_point.is_some() {
                        table_writer.write_entry::<i32>(0);
                    }
                    if prepared_dequantization_factor.is_some() {
                        table_writer.write_entry::<f32>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_zero_point) =
                                prepared_zero_point
                            {
                                object_writer.write::<_, _, 4>(&prepared_zero_point);
                            }
                            if let ::core::option::Option::Some(prepared_dequantization_factor) =
                                prepared_dequantization_factor
                            {
                                object_writer.write::<_, _, 4>(&prepared_dequantization_factor);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<NumericsConstants>> for NumericsConstants {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<NumericsConstants> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<NumericsConstants>> for NumericsConstants {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<NumericsConstants>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<NumericsConstants> for NumericsConstants {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<NumericsConstants> {
                    NumericsConstants::create(builder, self.zero_point, self.dequantization_factor)
                }
            }

            /// Builder for serializing an instance of the [NumericsConstants] type.
            ///
            /// Can be created using the [NumericsConstants::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct NumericsConstantsBuilder<State>(State);

            impl NumericsConstantsBuilder<()> {
                /// Setter for the [`zero_point` field](NumericsConstants#structfield.zero_point).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn zero_point<T0>(self, value: T0) -> NumericsConstantsBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<i32, i32>,
                {
                    NumericsConstantsBuilder((value,))
                }

                /// Sets the [`zero_point` field](NumericsConstants#structfield.zero_point) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn zero_point_as_default(
                    self,
                ) -> NumericsConstantsBuilder<(::planus::DefaultValue,)> {
                    self.zero_point(::planus::DefaultValue)
                }
            }

            impl<T0> NumericsConstantsBuilder<(T0,)> {
                /// Setter for the [`dequantization_factor` field](NumericsConstants#structfield.dequantization_factor).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn dequantization_factor<T1>(
                    self,
                    value: T1,
                ) -> NumericsConstantsBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<f32, f32>,
                {
                    let (v0,) = self.0;
                    NumericsConstantsBuilder((v0, value))
                }

                /// Sets the [`dequantization_factor` field](NumericsConstants#structfield.dequantization_factor) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn dequantization_factor_as_default(
                    self,
                ) -> NumericsConstantsBuilder<(T0, ::planus::DefaultValue)> {
                    self.dequantization_factor(::planus::DefaultValue)
                }
            }

            impl<T0, T1> NumericsConstantsBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [NumericsConstants].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<NumericsConstants>
                where
                    Self: ::planus::WriteAsOffset<NumericsConstants>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsDefault<f32, f32>,
                > ::planus::WriteAs<::planus::Offset<NumericsConstants>>
                for NumericsConstantsBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<NumericsConstants>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<NumericsConstants> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsDefault<f32, f32>,
                > ::planus::WriteAsOptional<::planus::Offset<NumericsConstants>>
                for NumericsConstantsBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<NumericsConstants>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<NumericsConstants>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsDefault<f32, f32>,
                > ::planus::WriteAsOffset<NumericsConstants>
                for NumericsConstantsBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<NumericsConstants> {
                    let (v0, v1) = &self.0;
                    NumericsConstants::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [NumericsConstants].
            #[derive(Copy, Clone)]
            pub struct NumericsConstantsRef<'a>(
                #[allow(dead_code)] ::planus::table_reader::Table<'a>,
            );

            impl<'a> NumericsConstantsRef<'a> {
                /// Getter for the [`zero_point` field](NumericsConstants#structfield.zero_point).
                #[inline]
                pub fn zero_point(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "NumericsConstants", "zero_point")?
                            .unwrap_or(0),
                    )
                }

                /// Getter for the [`dequantization_factor` field](NumericsConstants#structfield.dequantization_factor).
                #[inline]
                pub fn dequantization_factor(&self) -> ::planus::Result<f32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "NumericsConstants", "dequantization_factor")?
                            .unwrap_or(0.0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for NumericsConstantsRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("NumericsConstantsRef");
                    f.field("zero_point", &self.zero_point());
                    f.field("dequantization_factor", &self.dequantization_factor());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<NumericsConstantsRef<'a>> for NumericsConstants {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: NumericsConstantsRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        zero_point: ::core::convert::TryInto::try_into(value.zero_point()?)?,
                        dequantization_factor: ::core::convert::TryInto::try_into(
                            value.dequantization_factor()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for NumericsConstantsRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for NumericsConstantsRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[NumericsConstantsRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<NumericsConstants>> for NumericsConstants {
                type Value = ::planus::Offset<NumericsConstants>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<NumericsConstants>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for NumericsConstantsRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[NumericsConstantsRef]", "read_as_root", 0)
                    })
                }
            }

            /// The enum `DataType` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Enum `DataType` in the file `schema/executable.fbs:254`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i16)]
            pub enum DataType {
                /// The variant `FIXED_POINT8` in the enum `DataType`
                FixedPoint8 = 0,

                /// The variant `FIXED_POINT16` in the enum `DataType`
                FixedPoint16 = 1,

                /// The variant `SIGNED_FIXED_POINT32` in the enum `DataType`
                SignedFixedPoint32 = 2,

                /// The variant `BFLOAT` in the enum `DataType`
                Bfloat = 3,

                /// The variant `HALF` in the enum `DataType`
                Half = 4,

                /// The variant `SINGLE` in the enum `DataType`
                Single = 5,

                /// The variant `SIGNED_FIXED_POINT8` in the enum `DataType`
                SignedFixedPoint8 = 8,

                /// The variant `SIGNED_FIXED_POINT16` in the enum `DataType`
                SignedFixedPoint16 = 9,
            }

            impl DataType {
                /// Array containing all valid variants of DataType
                pub const ENUM_VALUES: [Self; 8] = [
                    Self::FixedPoint8,
                    Self::FixedPoint16,
                    Self::SignedFixedPoint32,
                    Self::Bfloat,
                    Self::Half,
                    Self::Single,
                    Self::SignedFixedPoint8,
                    Self::SignedFixedPoint16,
                ];
            }

            impl ::core::convert::TryFrom<i16> for DataType {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i16,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(DataType::FixedPoint8),
                        1 => ::core::result::Result::Ok(DataType::FixedPoint16),
                        2 => ::core::result::Result::Ok(DataType::SignedFixedPoint32),
                        3 => ::core::result::Result::Ok(DataType::Bfloat),
                        4 => ::core::result::Result::Ok(DataType::Half),
                        5 => ::core::result::Result::Ok(DataType::Single),
                        8 => ::core::result::Result::Ok(DataType::SignedFixedPoint8),
                        9 => ::core::result::Result::Ok(DataType::SignedFixedPoint16),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<DataType> for i16 {
                #[inline]
                fn from(value: DataType) -> Self {
                    value as i16
                }
            }

            /// # Safety
            /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
            unsafe impl ::planus::Primitive for DataType {
                const ALIGNMENT: usize = 2;
                const SIZE: usize = 2;
            }

            impl ::planus::WriteAsPrimitive<DataType> for DataType {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i16).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<DataType> for DataType {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> DataType {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<DataType, DataType> for DataType {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &DataType,
                ) -> ::core::option::Option<DataType> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<DataType> for DataType {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<DataType> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for DataType {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i16 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for DataType {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 2;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value =
                        unsafe { <i16 as ::planus::VectorRead>::from_buffer(buffer, offset) };
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "DataType",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<DataType> for DataType {
                const STRIDE: usize = 2;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 2];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (2 * i) as u32,
                        );
                    }
                }
            }

            /// The table `OutputLayer` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `OutputLayer` in the file `schema/executable.fbs:288`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct OutputLayer {
                /// The field `layout` in the table `OutputLayer`
                pub layout: ::core::option::Option<::planus::alloc::boxed::Box<self::OutputLayout>>,
                /// The field `data_type` in the table `OutputLayer`
                pub data_type: self::DataType,
                /// The field `shape_info` in the table `OutputLayer`
                pub shape_info:
                    ::core::option::Option<::planus::alloc::boxed::Box<self::OutputShapeInfo>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for OutputLayer {
                fn default() -> Self {
                    Self {
                        layout: ::core::default::Default::default(),
                        data_type: self::DataType::FixedPoint8,
                        shape_info: ::core::default::Default::default(),
                    }
                }
            }

            impl OutputLayer {
                /// Creates a [OutputLayerBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> OutputLayerBuilder<()> {
                    OutputLayerBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_layout: impl ::planus::WriteAsOptional<::planus::Offset<self::OutputLayout>>,
                    field_data_type: impl ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    field_shape_info: impl ::planus::WriteAsOptional<
                        ::planus::Offset<self::OutputShapeInfo>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_layout = field_layout.prepare(builder);
                    let prepared_data_type =
                        field_data_type.prepare(builder, &self::DataType::FixedPoint8);
                    let prepared_shape_info = field_shape_info.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<10> =
                        ::core::default::Default::default();
                    if prepared_layout.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::OutputLayout>>(0);
                    }
                    if prepared_shape_info.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::OutputShapeInfo>>(2);
                    }
                    if prepared_data_type.is_some() {
                        table_writer.write_entry::<self::DataType>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_layout) = prepared_layout {
                                object_writer.write::<_, _, 4>(&prepared_layout);
                            }
                            if let ::core::option::Option::Some(prepared_shape_info) =
                                prepared_shape_info
                            {
                                object_writer.write::<_, _, 4>(&prepared_shape_info);
                            }
                            if let ::core::option::Option::Some(prepared_data_type) =
                                prepared_data_type
                            {
                                object_writer.write::<_, _, 2>(&prepared_data_type);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<OutputLayer>> for OutputLayer {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayer> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<OutputLayer>> for OutputLayer {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<OutputLayer>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<OutputLayer> for OutputLayer {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayer> {
                    OutputLayer::create(builder, &self.layout, self.data_type, &self.shape_info)
                }
            }

            /// Builder for serializing an instance of the [OutputLayer] type.
            ///
            /// Can be created using the [OutputLayer::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct OutputLayerBuilder<State>(State);

            impl OutputLayerBuilder<()> {
                /// Setter for the [`layout` field](OutputLayer#structfield.layout).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn layout<T0>(self, value: T0) -> OutputLayerBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::OutputLayout>>,
                {
                    OutputLayerBuilder((value,))
                }

                /// Sets the [`layout` field](OutputLayer#structfield.layout) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn layout_as_null(self) -> OutputLayerBuilder<((),)> {
                    self.layout(())
                }
            }

            impl<T0> OutputLayerBuilder<(T0,)> {
                /// Setter for the [`data_type` field](OutputLayer#structfield.data_type).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn data_type<T1>(self, value: T1) -> OutputLayerBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                {
                    let (v0,) = self.0;
                    OutputLayerBuilder((v0, value))
                }

                /// Sets the [`data_type` field](OutputLayer#structfield.data_type) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn data_type_as_default(
                    self,
                ) -> OutputLayerBuilder<(T0, ::planus::DefaultValue)> {
                    self.data_type(::planus::DefaultValue)
                }
            }

            impl<T0, T1> OutputLayerBuilder<(T0, T1)> {
                /// Setter for the [`shape_info` field](OutputLayer#structfield.shape_info).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn shape_info<T2>(self, value: T2) -> OutputLayerBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<self::OutputShapeInfo>>,
                {
                    let (v0, v1) = self.0;
                    OutputLayerBuilder((v0, v1, value))
                }

                /// Sets the [`shape_info` field](OutputLayer#structfield.shape_info) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn shape_info_as_null(self) -> OutputLayerBuilder<(T0, T1, ())> {
                    self.shape_info(())
                }
            }

            impl<T0, T1, T2> OutputLayerBuilder<(T0, T1, T2)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [OutputLayer].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayer>
                where
                    Self: ::planus::WriteAsOffset<OutputLayer>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::OutputLayout>>,
                    T1: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<self::OutputShapeInfo>>,
                > ::planus::WriteAs<::planus::Offset<OutputLayer>>
                for OutputLayerBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<OutputLayer>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayer> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::OutputLayout>>,
                    T1: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<self::OutputShapeInfo>>,
                > ::planus::WriteAsOptional<::planus::Offset<OutputLayer>>
                for OutputLayerBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<OutputLayer>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<OutputLayer>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::OutputLayout>>,
                    T1: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<self::OutputShapeInfo>>,
                > ::planus::WriteAsOffset<OutputLayer> for OutputLayerBuilder<(T0, T1, T2)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<OutputLayer> {
                    let (v0, v1, v2) = &self.0;
                    OutputLayer::create(builder, v0, v1, v2)
                }
            }

            /// Reference to a deserialized [OutputLayer].
            #[derive(Copy, Clone)]
            pub struct OutputLayerRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> OutputLayerRef<'a> {
                /// Getter for the [`layout` field](OutputLayer#structfield.layout).
                #[inline]
                pub fn layout(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::OutputLayoutRef<'a>>>
                {
                    self.0.access(0, "OutputLayer", "layout")
                }

                /// Getter for the [`data_type` field](OutputLayer#structfield.data_type).
                #[inline]
                pub fn data_type(&self) -> ::planus::Result<self::DataType> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "OutputLayer", "data_type")?
                            .unwrap_or(self::DataType::FixedPoint8),
                    )
                }

                /// Getter for the [`shape_info` field](OutputLayer#structfield.shape_info).
                #[inline]
                pub fn shape_info(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::OutputShapeInfoRef<'a>>>
                {
                    self.0.access(2, "OutputLayer", "shape_info")
                }
            }

            impl<'a> ::core::fmt::Debug for OutputLayerRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("OutputLayerRef");
                    if let ::core::option::Option::Some(field_layout) = self.layout().transpose() {
                        f.field("layout", &field_layout);
                    }
                    f.field("data_type", &self.data_type());
                    if let ::core::option::Option::Some(field_shape_info) =
                        self.shape_info().transpose()
                    {
                        f.field("shape_info", &field_shape_info);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<OutputLayerRef<'a>> for OutputLayer {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: OutputLayerRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        layout: if let ::core::option::Option::Some(layout) = value.layout()? {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(layout)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        data_type: ::core::convert::TryInto::try_into(value.data_type()?)?,
                        shape_info: if let ::core::option::Option::Some(shape_info) =
                            value.shape_info()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(shape_info)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for OutputLayerRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for OutputLayerRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[OutputLayerRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<OutputLayer>> for OutputLayer {
                type Value = ::planus::Offset<OutputLayer>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<OutputLayer>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for OutputLayerRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[OutputLayerRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `InputLayer` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `InputLayer` in the file `schema/executable.fbs:300`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct InputLayer {}

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for InputLayer {
                fn default() -> Self {
                    Self {}
                }
            }

            impl InputLayer {
                /// Creates a [InputLayerBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> InputLayerBuilder<()> {
                    InputLayerBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(builder: &mut ::planus::Builder) -> ::planus::Offset<Self> {
                    let table_writer: ::planus::table_writer::TableWriter<4> =
                        ::core::default::Default::default();
                    unsafe {
                        table_writer.finish(builder, |_table_writer| {});
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<InputLayer>> for InputLayer {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<InputLayer> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<InputLayer>> for InputLayer {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InputLayer>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<InputLayer> for InputLayer {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<InputLayer> {
                    InputLayer::create(builder)
                }
            }

            /// Builder for serializing an instance of the [InputLayer] type.
            ///
            /// Can be created using the [InputLayer::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct InputLayerBuilder<State>(State);

            impl InputLayerBuilder<()> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [InputLayer].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<InputLayer>
                where
                    Self: ::planus::WriteAsOffset<InputLayer>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl ::planus::WriteAs<::planus::Offset<InputLayer>> for InputLayerBuilder<()> {
                type Prepared = ::planus::Offset<InputLayer>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<InputLayer> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<InputLayer>> for InputLayerBuilder<()> {
                type Prepared = ::planus::Offset<InputLayer>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<InputLayer>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<InputLayer> for InputLayerBuilder<()> {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<InputLayer> {
                    InputLayer::create(builder)
                }
            }

            /// Reference to a deserialized [InputLayer].
            #[derive(Copy, Clone)]
            pub struct InputLayerRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> InputLayerRef<'a> {}

            impl<'a> ::core::fmt::Debug for InputLayerRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("InputLayerRef");

                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<InputLayerRef<'a>> for InputLayer {
                type Error = ::planus::Error;

                fn try_from(_value: InputLayerRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {})
                }
            }

            impl<'a> ::planus::TableRead<'a> for InputLayerRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for InputLayerRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[InputLayerRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<InputLayer>> for InputLayer {
                type Value = ::planus::Offset<InputLayer>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<InputLayer>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for InputLayerRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[InputLayerRef]", "read_as_root", 0)
                    })
                }
            }

            /// The union `AnyLayer` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Union `AnyLayer` in the file `schema/executable.fbs:304`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub enum AnyLayer {
                /// The variant of type `OutputLayer` in the union `AnyLayer`
                OutputLayer(::planus::alloc::boxed::Box<self::OutputLayer>),

                /// The variant of type `InputLayer` in the union `AnyLayer`
                InputLayer(::planus::alloc::boxed::Box<self::InputLayer>),
            }

            impl AnyLayer {
                /// Creates a [AnyLayerBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> AnyLayerBuilder<::planus::Uninitialized> {
                    AnyLayerBuilder(::planus::Uninitialized)
                }

                #[inline]
                pub fn create_output_layer(
                    builder: &mut ::planus::Builder,
                    value: impl ::planus::WriteAsOffset<self::OutputLayer>,
                ) -> ::planus::UnionOffset<Self> {
                    ::planus::UnionOffset::new(1, value.prepare(builder).downcast())
                }

                #[inline]
                pub fn create_input_layer(
                    builder: &mut ::planus::Builder,
                    value: impl ::planus::WriteAsOffset<self::InputLayer>,
                ) -> ::planus::UnionOffset<Self> {
                    ::planus::UnionOffset::new(2, value.prepare(builder).downcast())
                }
            }

            impl ::planus::WriteAsUnion<AnyLayer> for AnyLayer {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Self> {
                    match self {
                        Self::OutputLayer(value) => Self::create_output_layer(builder, value),
                        Self::InputLayer(value) => Self::create_input_layer(builder, value),
                    }
                }
            }

            impl ::planus::WriteAsOptionalUnion<AnyLayer> for AnyLayer {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<Self>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }

            /// Builder for serializing an instance of the [AnyLayer] type.
            ///
            /// Can be created using the [AnyLayer::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct AnyLayerBuilder<T>(T);

            impl AnyLayerBuilder<::planus::Uninitialized> {
                /// Creates an instance of the [`OutputLayer` variant](AnyLayer#variant.OutputLayer).
                #[inline]
                pub fn output_layer<T>(
                    self,
                    value: T,
                ) -> AnyLayerBuilder<::planus::Initialized<1, T>>
                where
                    T: ::planus::WriteAsOffset<self::OutputLayer>,
                {
                    AnyLayerBuilder(::planus::Initialized(value))
                }

                /// Creates an instance of the [`InputLayer` variant](AnyLayer#variant.InputLayer).
                #[inline]
                pub fn input_layer<T>(
                    self,
                    value: T,
                ) -> AnyLayerBuilder<::planus::Initialized<2, T>>
                where
                    T: ::planus::WriteAsOffset<self::InputLayer>,
                {
                    AnyLayerBuilder(::planus::Initialized(value))
                }
            }

            impl<const N: u8, T> AnyLayerBuilder<::planus::Initialized<N, T>> {
                /// Finish writing the builder to get an [UnionOffset](::planus::UnionOffset) to a serialized [AnyLayer].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyLayer>
                where
                    Self: ::planus::WriteAsUnion<AnyLayer>,
                {
                    ::planus::WriteAsUnion::prepare(&self, builder)
                }
            }

            impl<T> ::planus::WriteAsUnion<AnyLayer> for AnyLayerBuilder<::planus::Initialized<1, T>>
            where
                T: ::planus::WriteAsOffset<self::OutputLayer>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyLayer> {
                    ::planus::UnionOffset::new(1, (self.0).0.prepare(builder).downcast())
                }
            }

            impl<T> ::planus::WriteAsOptionalUnion<AnyLayer> for AnyLayerBuilder<::planus::Initialized<1, T>>
            where
                T: ::planus::WriteAsOffset<self::OutputLayer>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<AnyLayer>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }
            impl<T> ::planus::WriteAsUnion<AnyLayer> for AnyLayerBuilder<::planus::Initialized<2, T>>
            where
                T: ::planus::WriteAsOffset<self::InputLayer>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::UnionOffset<AnyLayer> {
                    ::planus::UnionOffset::new(2, (self.0).0.prepare(builder).downcast())
                }
            }

            impl<T> ::planus::WriteAsOptionalUnion<AnyLayer> for AnyLayerBuilder<::planus::Initialized<2, T>>
            where
                T: ::planus::WriteAsOffset<self::InputLayer>,
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::UnionOffset<AnyLayer>> {
                    ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
                }
            }

            /// Reference to a deserialized [AnyLayer].
            #[derive(Copy, Clone, Debug)]
            pub enum AnyLayerRef<'a> {
                OutputLayer(self::OutputLayerRef<'a>),
                InputLayer(self::InputLayerRef<'a>),
            }

            impl<'a> ::core::convert::TryFrom<AnyLayerRef<'a>> for AnyLayer {
                type Error = ::planus::Error;

                fn try_from(value: AnyLayerRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(match value {
                        AnyLayerRef::OutputLayer(value) => {
                            Self::OutputLayer(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryFrom::try_from(value)?,
                            ))
                        }

                        AnyLayerRef::InputLayer(value) => {
                            Self::InputLayer(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryFrom::try_from(value)?,
                            ))
                        }
                    })
                }
            }

            impl<'a> ::planus::TableReadUnion<'a> for AnyLayerRef<'a> {
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    tag: u8,
                    field_offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    match tag {
                        1 => ::core::result::Result::Ok(Self::OutputLayer(
                            ::planus::TableRead::from_buffer(buffer, field_offset)?,
                        )),
                        2 => ::core::result::Result::Ok(Self::InputLayer(
                            ::planus::TableRead::from_buffer(buffer, field_offset)?,
                        )),
                        _ => ::core::result::Result::Err(
                            ::planus::errors::ErrorKind::UnknownUnionTag { tag },
                        ),
                    }
                }
            }

            impl<'a> ::planus::VectorReadUnion<'a> for AnyLayerRef<'a> {
                const VECTOR_NAME: &'static str = "[AnyLayerRef]";
            }

            /// The table `Layer` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `Layer` in the file `schema/executable.fbs:310`
            #[derive(
                Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize,
            )]
            pub struct Layer {
                /// The field `name` in the table `Layer`
                pub name: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `size_bytes` in the table `Layer`
                pub size_bytes: i32,
                /// The field `y_dim` in the table `Layer`
                pub y_dim: i32,
                /// The field `x_dim` in the table `Layer`
                pub x_dim: i32,
                /// The field `z_dim` in the table `Layer`
                pub z_dim: i32,
                /// The field `numerics` in the table `Layer`
                pub numerics:
                    ::core::option::Option<::planus::alloc::boxed::Box<self::NumericsConstants>>,
                /// The field `data_type` in the table `Layer`
                pub data_type: self::DataType,
                /// The field `any_layer` in the table `Layer`
                pub any_layer: ::core::option::Option<self::AnyLayer>,
                /// The field `execution_count_per_inference` in the table `Layer`
                pub execution_count_per_inference: i32,
                /// The field `cache_on_dram` in the table `Layer`
                pub cache_on_dram: bool,
                /// The field `shape` in the table `Layer`
                pub shape: ::core::option::Option<::planus::alloc::boxed::Box<self::TensorShape>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for Layer {
                fn default() -> Self {
                    Self {
                        name: ::core::default::Default::default(),
                        size_bytes: 0,
                        y_dim: 0,
                        x_dim: 0,
                        z_dim: 0,
                        numerics: ::core::default::Default::default(),
                        data_type: self::DataType::FixedPoint8,
                        any_layer: ::core::default::Default::default(),
                        execution_count_per_inference: 1,
                        cache_on_dram: false,
                        shape: ::core::default::Default::default(),
                    }
                }
            }

            impl Layer {
                /// Creates a [LayerBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> LayerBuilder<()> {
                    LayerBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    field_size_bytes: impl ::planus::WriteAsDefault<i32, i32>,
                    field_y_dim: impl ::planus::WriteAsDefault<i32, i32>,
                    field_x_dim: impl ::planus::WriteAsDefault<i32, i32>,
                    field_z_dim: impl ::planus::WriteAsDefault<i32, i32>,
                    field_numerics: impl ::planus::WriteAsOptional<
                        ::planus::Offset<self::NumericsConstants>,
                    >,
                    field_data_type: impl ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    field_any_layer: impl ::planus::WriteAsOptionalUnion<self::AnyLayer>,
                    field_execution_count_per_inference: impl ::planus::WriteAsDefault<i32, i32>,
                    field_cache_on_dram: impl ::planus::WriteAsDefault<bool, bool>,
                    field_shape: impl ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                ) -> ::planus::Offset<Self> {
                    let prepared_name = field_name.prepare(builder);
                    let prepared_size_bytes = field_size_bytes.prepare(builder, &0);
                    let prepared_y_dim = field_y_dim.prepare(builder, &0);
                    let prepared_x_dim = field_x_dim.prepare(builder, &0);
                    let prepared_z_dim = field_z_dim.prepare(builder, &0);
                    let prepared_numerics = field_numerics.prepare(builder);
                    let prepared_data_type =
                        field_data_type.prepare(builder, &self::DataType::FixedPoint8);
                    let prepared_any_layer = field_any_layer.prepare(builder);
                    let prepared_execution_count_per_inference =
                        field_execution_count_per_inference.prepare(builder, &1);
                    let prepared_cache_on_dram = field_cache_on_dram.prepare(builder, &false);
                    let prepared_shape = field_shape.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<28> =
                        ::core::default::Default::default();
                    if prepared_name.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(0);
                    }
                    if prepared_size_bytes.is_some() {
                        table_writer.write_entry::<i32>(1);
                    }
                    if prepared_y_dim.is_some() {
                        table_writer.write_entry::<i32>(2);
                    }
                    if prepared_x_dim.is_some() {
                        table_writer.write_entry::<i32>(3);
                    }
                    if prepared_z_dim.is_some() {
                        table_writer.write_entry::<i32>(4);
                    }
                    if prepared_numerics.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::NumericsConstants>>(5);
                    }
                    if prepared_any_layer.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::AnyLayer>>(8);
                    }
                    if prepared_execution_count_per_inference.is_some() {
                        table_writer.write_entry::<i32>(9);
                    }
                    if prepared_shape.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::TensorShape>>(11);
                    }
                    if prepared_data_type.is_some() {
                        table_writer.write_entry::<self::DataType>(6);
                    }
                    if prepared_any_layer.is_some() {
                        table_writer.write_entry::<u8>(7);
                    }
                    if prepared_cache_on_dram.is_some() {
                        table_writer.write_entry::<bool>(10);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_name) = prepared_name {
                                object_writer.write::<_, _, 4>(&prepared_name);
                            }
                            if let ::core::option::Option::Some(prepared_size_bytes) =
                                prepared_size_bytes
                            {
                                object_writer.write::<_, _, 4>(&prepared_size_bytes);
                            }
                            if let ::core::option::Option::Some(prepared_y_dim) = prepared_y_dim {
                                object_writer.write::<_, _, 4>(&prepared_y_dim);
                            }
                            if let ::core::option::Option::Some(prepared_x_dim) = prepared_x_dim {
                                object_writer.write::<_, _, 4>(&prepared_x_dim);
                            }
                            if let ::core::option::Option::Some(prepared_z_dim) = prepared_z_dim {
                                object_writer.write::<_, _, 4>(&prepared_z_dim);
                            }
                            if let ::core::option::Option::Some(prepared_numerics) =
                                prepared_numerics
                            {
                                object_writer.write::<_, _, 4>(&prepared_numerics);
                            }
                            if let ::core::option::Option::Some(prepared_any_layer) =
                                prepared_any_layer
                            {
                                object_writer.write::<_, _, 4>(&prepared_any_layer.offset());
                            }
                            if let ::core::option::Option::Some(
                                prepared_execution_count_per_inference,
                            ) = prepared_execution_count_per_inference
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_execution_count_per_inference);
                            }
                            if let ::core::option::Option::Some(prepared_shape) = prepared_shape {
                                object_writer.write::<_, _, 4>(&prepared_shape);
                            }
                            if let ::core::option::Option::Some(prepared_data_type) =
                                prepared_data_type
                            {
                                object_writer.write::<_, _, 2>(&prepared_data_type);
                            }
                            if let ::core::option::Option::Some(prepared_any_layer) =
                                prepared_any_layer
                            {
                                object_writer.write::<_, _, 1>(&prepared_any_layer.tag());
                            }
                            if let ::core::option::Option::Some(prepared_cache_on_dram) =
                                prepared_cache_on_dram
                            {
                                object_writer.write::<_, _, 1>(&prepared_cache_on_dram);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<Layer>> for Layer {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Layer> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<Layer>> for Layer {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Layer>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<Layer> for Layer {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Layer> {
                    Layer::create(
                        builder,
                        &self.name,
                        self.size_bytes,
                        self.y_dim,
                        self.x_dim,
                        self.z_dim,
                        &self.numerics,
                        self.data_type,
                        &self.any_layer,
                        self.execution_count_per_inference,
                        self.cache_on_dram,
                        &self.shape,
                    )
                }
            }

            /// Builder for serializing an instance of the [Layer] type.
            ///
            /// Can be created using the [Layer::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct LayerBuilder<State>(State);

            impl LayerBuilder<()> {
                /// Setter for the [`name` field](Layer#structfield.name).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name<T0>(self, value: T0) -> LayerBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    LayerBuilder((value,))
                }

                /// Sets the [`name` field](Layer#structfield.name) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name_as_null(self) -> LayerBuilder<((),)> {
                    self.name(())
                }
            }

            impl<T0> LayerBuilder<(T0,)> {
                /// Setter for the [`size_bytes` field](Layer#structfield.size_bytes).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn size_bytes<T1>(self, value: T1) -> LayerBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0,) = self.0;
                    LayerBuilder((v0, value))
                }

                /// Sets the [`size_bytes` field](Layer#structfield.size_bytes) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn size_bytes_as_default(self) -> LayerBuilder<(T0, ::planus::DefaultValue)> {
                    self.size_bytes(::planus::DefaultValue)
                }
            }

            impl<T0, T1> LayerBuilder<(T0, T1)> {
                /// Setter for the [`y_dim` field](Layer#structfield.y_dim).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn y_dim<T2>(self, value: T2) -> LayerBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1) = self.0;
                    LayerBuilder((v0, v1, value))
                }

                /// Sets the [`y_dim` field](Layer#structfield.y_dim) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn y_dim_as_default(self) -> LayerBuilder<(T0, T1, ::planus::DefaultValue)> {
                    self.y_dim(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2> LayerBuilder<(T0, T1, T2)> {
                /// Setter for the [`x_dim` field](Layer#structfield.x_dim).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_dim<T3>(self, value: T3) -> LayerBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2) = self.0;
                    LayerBuilder((v0, v1, v2, value))
                }

                /// Sets the [`x_dim` field](Layer#structfield.x_dim) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn x_dim_as_default(
                    self,
                ) -> LayerBuilder<(T0, T1, T2, ::planus::DefaultValue)> {
                    self.x_dim(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3> LayerBuilder<(T0, T1, T2, T3)> {
                /// Setter for the [`z_dim` field](Layer#structfield.z_dim).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn z_dim<T4>(self, value: T4) -> LayerBuilder<(T0, T1, T2, T3, T4)>
                where
                    T4: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2, v3) = self.0;
                    LayerBuilder((v0, v1, v2, v3, value))
                }

                /// Sets the [`z_dim` field](Layer#structfield.z_dim) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn z_dim_as_default(
                    self,
                ) -> LayerBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)> {
                    self.z_dim(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4> LayerBuilder<(T0, T1, T2, T3, T4)> {
                /// Setter for the [`numerics` field](Layer#structfield.numerics).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn numerics<T5>(self, value: T5) -> LayerBuilder<(T0, T1, T2, T3, T4, T5)>
                where
                    T5: ::planus::WriteAsOptional<::planus::Offset<self::NumericsConstants>>,
                {
                    let (v0, v1, v2, v3, v4) = self.0;
                    LayerBuilder((v0, v1, v2, v3, v4, value))
                }

                /// Sets the [`numerics` field](Layer#structfield.numerics) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn numerics_as_null(self) -> LayerBuilder<(T0, T1, T2, T3, T4, ())> {
                    self.numerics(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5> LayerBuilder<(T0, T1, T2, T3, T4, T5)> {
                /// Setter for the [`data_type` field](Layer#structfield.data_type).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn data_type<T6>(self, value: T6) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6)>
                where
                    T6: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                {
                    let (v0, v1, v2, v3, v4, v5) = self.0;
                    LayerBuilder((v0, v1, v2, v3, v4, v5, value))
                }

                /// Sets the [`data_type` field](Layer#structfield.data_type) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn data_type_as_default(
                    self,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, ::planus::DefaultValue)>
                {
                    self.data_type(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6)> {
                /// Setter for the [`any_layer` field](Layer#structfield.any_layer).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn any_layer<T7>(
                    self,
                    value: T7,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
                where
                    T7: ::planus::WriteAsOptionalUnion<self::AnyLayer>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6) = self.0;
                    LayerBuilder((v0, v1, v2, v3, v4, v5, v6, value))
                }

                /// Sets the [`any_layer` field](Layer#structfield.any_layer) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn any_layer_as_null(self) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, ())> {
                    self.any_layer(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)> {
                /// Setter for the [`execution_count_per_inference` field](Layer#structfield.execution_count_per_inference).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn execution_count_per_inference<T8>(
                    self,
                    value: T8,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
                where
                    T8: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7) = self.0;
                    LayerBuilder((v0, v1, v2, v3, v4, v5, v6, v7, value))
                }

                /// Sets the [`execution_count_per_inference` field](Layer#structfield.execution_count_per_inference) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn execution_count_per_inference_as_default(
                    self,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, ::planus::DefaultValue)>
                {
                    self.execution_count_per_inference(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
                /// Setter for the [`cache_on_dram` field](Layer#structfield.cache_on_dram).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cache_on_dram<T9>(
                    self,
                    value: T9,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
                where
                    T9: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self.0;
                    LayerBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, value))
                }

                /// Sets the [`cache_on_dram` field](Layer#structfield.cache_on_dram) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cache_on_dram_as_default(
                    self,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, ::planus::DefaultValue)>
                {
                    self.cache_on_dram(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
                LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
            {
                /// Setter for the [`shape` field](Layer#structfield.shape).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn shape<T10>(
                    self,
                    value: T10,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
                where
                    T10: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self.0;
                    LayerBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, value))
                }

                /// Sets the [`shape` field](Layer#structfield.shape) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn shape_as_null(
                    self,
                ) -> LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, ())> {
                    self.shape(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
                LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
            {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Layer].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Layer>
                where
                    Self: ::planus::WriteAsOffset<Layer>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsDefault<i32, i32>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsDefault<i32, i32>,
                    T5: ::planus::WriteAsOptional<::planus::Offset<self::NumericsConstants>>,
                    T6: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    T7: ::planus::WriteAsOptionalUnion<self::AnyLayer>,
                    T8: ::planus::WriteAsDefault<i32, i32>,
                    T9: ::planus::WriteAsDefault<bool, bool>,
                    T10: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                > ::planus::WriteAs<::planus::Offset<Layer>>
                for LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
            {
                type Prepared = ::planus::Offset<Layer>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Layer> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsDefault<i32, i32>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsDefault<i32, i32>,
                    T5: ::planus::WriteAsOptional<::planus::Offset<self::NumericsConstants>>,
                    T6: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    T7: ::planus::WriteAsOptionalUnion<self::AnyLayer>,
                    T8: ::planus::WriteAsDefault<i32, i32>,
                    T9: ::planus::WriteAsDefault<bool, bool>,
                    T10: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                > ::planus::WriteAsOptional<::planus::Offset<Layer>>
                for LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
            {
                type Prepared = ::planus::Offset<Layer>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Layer>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                    T2: ::planus::WriteAsDefault<i32, i32>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsDefault<i32, i32>,
                    T5: ::planus::WriteAsOptional<::planus::Offset<self::NumericsConstants>>,
                    T6: ::planus::WriteAsDefault<self::DataType, self::DataType>,
                    T7: ::planus::WriteAsOptionalUnion<self::AnyLayer>,
                    T8: ::planus::WriteAsDefault<i32, i32>,
                    T9: ::planus::WriteAsDefault<bool, bool>,
                    T10: ::planus::WriteAsOptional<::planus::Offset<self::TensorShape>>,
                > ::planus::WriteAsOffset<Layer>
                for LayerBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
            {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Layer> {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = &self.0;
                    Layer::create(builder, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
                }
            }

            /// Reference to a deserialized [Layer].
            #[derive(Copy, Clone)]
            pub struct LayerRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> LayerRef<'a> {
                /// Getter for the [`name` field](Layer#structfield.name).
                #[inline]
                pub fn name(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(0, "Layer", "name")
                }

                /// Getter for the [`size_bytes` field](Layer#structfield.size_bytes).
                #[inline]
                pub fn size_bytes(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0.access(1, "Layer", "size_bytes")?.unwrap_or(0),
                    )
                }

                /// Getter for the [`y_dim` field](Layer#structfield.y_dim).
                #[inline]
                pub fn y_dim(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(self.0.access(2, "Layer", "y_dim")?.unwrap_or(0))
                }

                /// Getter for the [`x_dim` field](Layer#structfield.x_dim).
                #[inline]
                pub fn x_dim(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(self.0.access(3, "Layer", "x_dim")?.unwrap_or(0))
                }

                /// Getter for the [`z_dim` field](Layer#structfield.z_dim).
                #[inline]
                pub fn z_dim(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(self.0.access(4, "Layer", "z_dim")?.unwrap_or(0))
                }

                /// Getter for the [`numerics` field](Layer#structfield.numerics).
                #[inline]
                pub fn numerics(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::NumericsConstantsRef<'a>>>
                {
                    self.0.access(5, "Layer", "numerics")
                }

                /// Getter for the [`data_type` field](Layer#structfield.data_type).
                #[inline]
                pub fn data_type(&self) -> ::planus::Result<self::DataType> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(6, "Layer", "data_type")?
                            .unwrap_or(self::DataType::FixedPoint8),
                    )
                }

                /// Getter for the [`any_layer` field](Layer#structfield.any_layer).
                #[inline]
                pub fn any_layer(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::AnyLayerRef<'a>>>
                {
                    self.0.access_union(7, "Layer", "any_layer")
                }

                /// Getter for the [`execution_count_per_inference` field](Layer#structfield.execution_count_per_inference).
                #[inline]
                pub fn execution_count_per_inference(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(9, "Layer", "execution_count_per_inference")?
                            .unwrap_or(1),
                    )
                }

                /// Getter for the [`cache_on_dram` field](Layer#structfield.cache_on_dram).
                #[inline]
                pub fn cache_on_dram(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(10, "Layer", "cache_on_dram")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`shape` field](Layer#structfield.shape).
                #[inline]
                pub fn shape(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::TensorShapeRef<'a>>>
                {
                    self.0.access(11, "Layer", "shape")
                }
            }

            impl<'a> ::core::fmt::Debug for LayerRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("LayerRef");
                    if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                        f.field("name", &field_name);
                    }
                    f.field("size_bytes", &self.size_bytes());
                    f.field("y_dim", &self.y_dim());
                    f.field("x_dim", &self.x_dim());
                    f.field("z_dim", &self.z_dim());
                    if let ::core::option::Option::Some(field_numerics) =
                        self.numerics().transpose()
                    {
                        f.field("numerics", &field_numerics);
                    }
                    f.field("data_type", &self.data_type());
                    if let ::core::option::Option::Some(field_any_layer) =
                        self.any_layer().transpose()
                    {
                        f.field("any_layer", &field_any_layer);
                    }
                    f.field(
                        "execution_count_per_inference",
                        &self.execution_count_per_inference(),
                    );
                    f.field("cache_on_dram", &self.cache_on_dram());
                    if let ::core::option::Option::Some(field_shape) = self.shape().transpose() {
                        f.field("shape", &field_shape);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<LayerRef<'a>> for Layer {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: LayerRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        name: value.name()?.map(::core::convert::Into::into),
                        size_bytes: ::core::convert::TryInto::try_into(value.size_bytes()?)?,
                        y_dim: ::core::convert::TryInto::try_into(value.y_dim()?)?,
                        x_dim: ::core::convert::TryInto::try_into(value.x_dim()?)?,
                        z_dim: ::core::convert::TryInto::try_into(value.z_dim()?)?,
                        numerics: if let ::core::option::Option::Some(numerics) =
                            value.numerics()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(numerics)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        data_type: ::core::convert::TryInto::try_into(value.data_type()?)?,
                        any_layer: if let ::core::option::Option::Some(any_layer) =
                            value.any_layer()?
                        {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(
                                any_layer,
                            )?)
                        } else {
                            ::core::option::Option::None
                        },
                        execution_count_per_inference: ::core::convert::TryInto::try_into(
                            value.execution_count_per_inference()?,
                        )?,
                        cache_on_dram: ::core::convert::TryInto::try_into(value.cache_on_dram()?)?,
                        shape: if let ::core::option::Option::Some(shape) = value.shape()? {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(shape)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for LayerRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for LayerRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[LayerRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<Layer>> for Layer {
                type Value = ::planus::Offset<Layer>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<Layer>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for LayerRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[LayerRef]", "read_as_root", 0)
                    })
                }
            }

            /// The enum `ExecutableType` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Enum `ExecutableType` in the file `schema/executable.fbs:349`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i16)]
            pub enum ExecutableType {
                /// The variant `STAND_ALONE` in the enum `ExecutableType`
                StandAlone = 0,

                /// The variant `PARAMETER_CACHING` in the enum `ExecutableType`
                ParameterCaching = 1,

                /// The variant `EXECUTION_ONLY` in the enum `ExecutableType`
                ExecutionOnly = 2,
            }

            impl ExecutableType {
                /// Array containing all valid variants of ExecutableType
                pub const ENUM_VALUES: [Self; 3] = [
                    Self::StandAlone,
                    Self::ParameterCaching,
                    Self::ExecutionOnly,
                ];
            }

            impl ::core::convert::TryFrom<i16> for ExecutableType {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i16,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(ExecutableType::StandAlone),
                        1 => ::core::result::Result::Ok(ExecutableType::ParameterCaching),
                        2 => ::core::result::Result::Ok(ExecutableType::ExecutionOnly),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<ExecutableType> for i16 {
                #[inline]
                fn from(value: ExecutableType) -> Self {
                    value as i16
                }
            }

            /// # Safety
            /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
            unsafe impl ::planus::Primitive for ExecutableType {
                const ALIGNMENT: usize = 2;
                const SIZE: usize = 2;
            }

            impl ::planus::WriteAsPrimitive<ExecutableType> for ExecutableType {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i16).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<ExecutableType> for ExecutableType {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> ExecutableType {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<ExecutableType, ExecutableType> for ExecutableType {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &ExecutableType,
                ) -> ::core::option::Option<ExecutableType> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<ExecutableType> for ExecutableType {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<ExecutableType> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for ExecutableType {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i16 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for ExecutableType {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 2;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value =
                        unsafe { <i16 as ::planus::VectorRead>::from_buffer(buffer, offset) };
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "ExecutableType",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<ExecutableType> for ExecutableType {
                const STRIDE: usize = 2;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 2];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (2 * i) as u32,
                        );
                    }
                }
            }

            /// The table `Executable` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `Executable` in the file `schema/executable.fbs:363`
            #[derive(
                Clone, Debug, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize,
            )]
            pub struct Executable {
                /// The field `version` in the table `Executable`
                pub version: i32,
                /// The field `name` in the table `Executable`
                pub name: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `serialized_model` in the table `Executable`
                pub serialized_model: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `batch_size` in the table `Executable`
                pub batch_size: i32,
                /// The field `scratch_size_bytes` in the table `Executable`
                pub scratch_size_bytes: i32,
                /// The field `instruction_bitstreams` in the table `Executable`
                pub instruction_bitstreams:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::InstructionBitstream>>,
                /// The field `parameters` in the table `Executable`
                pub parameters: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `dma_hints` in the table `Executable`
                pub dma_hints: ::core::option::Option<::planus::alloc::boxed::Box<self::DmaHints>>,
                /// The field `input_layers` in the table `Executable`
                pub input_layers: ::core::option::Option<::planus::alloc::vec::Vec<self::Layer>>,
                /// The field `output_layers` in the table `Executable`
                pub output_layers: ::core::option::Option<::planus::alloc::vec::Vec<self::Layer>>,
                /// The field `chip` in the table `Executable`
                pub chip: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `estimated_cycles` in the table `Executable`
                pub estimated_cycles: i32,
                /// The field `used_narrow_memory_bytes_per_tile` in the table `Executable`
                pub used_narrow_memory_bytes_per_tile: i32,
                /// The field `type` in the table `Executable`
                pub type_: self::ExecutableType,
                /// The field `parameter_caching_token` in the table `Executable`
                pub parameter_caching_token: u64,
                /// The field `use_tpu_dram_for_parameters` in the table `Executable`
                pub use_tpu_dram_for_parameters: bool,
                /// The field `estimated_cycles_64bit` in the table `Executable`
                pub estimated_cycles_64bit: i64,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for Executable {
                fn default() -> Self {
                    Self {
                        version: 0,
                        name: ::core::default::Default::default(),
                        serialized_model: ::core::default::Default::default(),
                        batch_size: 0,
                        scratch_size_bytes: 0,
                        instruction_bitstreams: ::core::default::Default::default(),
                        parameters: ::core::default::Default::default(),
                        dma_hints: ::core::default::Default::default(),
                        input_layers: ::core::default::Default::default(),
                        output_layers: ::core::default::Default::default(),
                        chip: ::core::default::Default::default(),
                        estimated_cycles: 0,
                        used_narrow_memory_bytes_per_tile: 0,
                        type_: self::ExecutableType::StandAlone,
                        parameter_caching_token: 0,
                        use_tpu_dram_for_parameters: false,
                        estimated_cycles_64bit: 0,
                    }
                }
            }

            impl Executable {
                /// Creates a [ExecutableBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> ExecutableBuilder<()> {
                    ExecutableBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_version: impl ::planus::WriteAsDefault<i32, i32>,
                    field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    field_serialized_model: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_batch_size: impl ::planus::WriteAsDefault<i32, i32>,
                    field_scratch_size_bytes: impl ::planus::WriteAsDefault<i32, i32>,
                    field_instruction_bitstreams: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::InstructionBitstream>]>,
                    >,
                    field_parameters: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_dma_hints: impl ::planus::WriteAsOptional<::planus::Offset<self::DmaHints>>,
                    field_input_layers: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::Layer>]>,
                    >,
                    field_output_layers: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::Layer>]>,
                    >,
                    field_chip: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    field_estimated_cycles: impl ::planus::WriteAsDefault<i32, i32>,
                    field_used_narrow_memory_bytes_per_tile: impl ::planus::WriteAsDefault<i32, i32>,
                    field_type_: impl ::planus::WriteAsDefault<
                        self::ExecutableType,
                        self::ExecutableType,
                    >,
                    field_parameter_caching_token: impl ::planus::WriteAsDefault<u64, u64>,
                    field_use_tpu_dram_for_parameters: impl ::planus::WriteAsDefault<bool, bool>,
                    field_estimated_cycles_64bit: impl ::planus::WriteAsDefault<i64, i64>,
                ) -> ::planus::Offset<Self> {
                    let prepared_version = field_version.prepare(builder, &0);
                    let prepared_name = field_name.prepare(builder);
                    let prepared_serialized_model = field_serialized_model.prepare(builder);
                    let prepared_batch_size = field_batch_size.prepare(builder, &0);
                    let prepared_scratch_size_bytes = field_scratch_size_bytes.prepare(builder, &0);
                    let prepared_instruction_bitstreams =
                        field_instruction_bitstreams.prepare(builder);
                    let prepared_parameters = field_parameters.prepare(builder);
                    let prepared_dma_hints = field_dma_hints.prepare(builder);
                    let prepared_input_layers = field_input_layers.prepare(builder);
                    let prepared_output_layers = field_output_layers.prepare(builder);
                    let prepared_chip = field_chip.prepare(builder);
                    let prepared_estimated_cycles = field_estimated_cycles.prepare(builder, &0);
                    let prepared_used_narrow_memory_bytes_per_tile =
                        field_used_narrow_memory_bytes_per_tile.prepare(builder, &0);
                    let prepared_type_ =
                        field_type_.prepare(builder, &self::ExecutableType::StandAlone);
                    let prepared_parameter_caching_token =
                        field_parameter_caching_token.prepare(builder, &0);
                    let prepared_use_tpu_dram_for_parameters =
                        field_use_tpu_dram_for_parameters.prepare(builder, &false);
                    let prepared_estimated_cycles_64bit =
                        field_estimated_cycles_64bit.prepare(builder, &0);

                    let mut table_writer: ::planus::table_writer::TableWriter<38> =
                        ::core::default::Default::default();
                    if prepared_parameter_caching_token.is_some() {
                        table_writer.write_entry::<u64>(14);
                    }
                    if prepared_estimated_cycles_64bit.is_some() {
                        table_writer.write_entry::<i64>(16);
                    }
                    if prepared_version.is_some() {
                        table_writer.write_entry::<i32>(0);
                    }
                    if prepared_name.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(1);
                    }
                    if prepared_serialized_model.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(2);
                    }
                    if prepared_batch_size.is_some() {
                        table_writer.write_entry::<i32>(3);
                    }
                    if prepared_scratch_size_bytes.is_some() {
                        table_writer.write_entry::<i32>(4);
                    }
                    if prepared_instruction_bitstreams.is_some() {
                        table_writer.write_entry::<::planus::Offset<
                            [::planus::Offset<self::InstructionBitstream>],
                        >>(5);
                    }
                    if prepared_parameters.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(6);
                    }
                    if prepared_dma_hints.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::DmaHints>>(7);
                    }
                    if prepared_input_layers.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<[::planus::Offset<self::Layer>]>>(8);
                    }
                    if prepared_output_layers.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<[::planus::Offset<self::Layer>]>>(9);
                    }
                    if prepared_chip.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(10);
                    }
                    if prepared_estimated_cycles.is_some() {
                        table_writer.write_entry::<i32>(11);
                    }
                    if prepared_used_narrow_memory_bytes_per_tile.is_some() {
                        table_writer.write_entry::<i32>(12);
                    }
                    if prepared_type_.is_some() {
                        table_writer.write_entry::<self::ExecutableType>(13);
                    }
                    if prepared_use_tpu_dram_for_parameters.is_some() {
                        table_writer.write_entry::<bool>(15);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_parameter_caching_token) =
                                prepared_parameter_caching_token
                            {
                                object_writer.write::<_, _, 8>(&prepared_parameter_caching_token);
                            }
                            if let ::core::option::Option::Some(prepared_estimated_cycles_64bit) =
                                prepared_estimated_cycles_64bit
                            {
                                object_writer.write::<_, _, 8>(&prepared_estimated_cycles_64bit);
                            }
                            if let ::core::option::Option::Some(prepared_version) = prepared_version
                            {
                                object_writer.write::<_, _, 4>(&prepared_version);
                            }
                            if let ::core::option::Option::Some(prepared_name) = prepared_name {
                                object_writer.write::<_, _, 4>(&prepared_name);
                            }
                            if let ::core::option::Option::Some(prepared_serialized_model) =
                                prepared_serialized_model
                            {
                                object_writer.write::<_, _, 4>(&prepared_serialized_model);
                            }
                            if let ::core::option::Option::Some(prepared_batch_size) =
                                prepared_batch_size
                            {
                                object_writer.write::<_, _, 4>(&prepared_batch_size);
                            }
                            if let ::core::option::Option::Some(prepared_scratch_size_bytes) =
                                prepared_scratch_size_bytes
                            {
                                object_writer.write::<_, _, 4>(&prepared_scratch_size_bytes);
                            }
                            if let ::core::option::Option::Some(prepared_instruction_bitstreams) =
                                prepared_instruction_bitstreams
                            {
                                object_writer.write::<_, _, 4>(&prepared_instruction_bitstreams);
                            }
                            if let ::core::option::Option::Some(prepared_parameters) =
                                prepared_parameters
                            {
                                object_writer.write::<_, _, 4>(&prepared_parameters);
                            }
                            if let ::core::option::Option::Some(prepared_dma_hints) =
                                prepared_dma_hints
                            {
                                object_writer.write::<_, _, 4>(&prepared_dma_hints);
                            }
                            if let ::core::option::Option::Some(prepared_input_layers) =
                                prepared_input_layers
                            {
                                object_writer.write::<_, _, 4>(&prepared_input_layers);
                            }
                            if let ::core::option::Option::Some(prepared_output_layers) =
                                prepared_output_layers
                            {
                                object_writer.write::<_, _, 4>(&prepared_output_layers);
                            }
                            if let ::core::option::Option::Some(prepared_chip) = prepared_chip {
                                object_writer.write::<_, _, 4>(&prepared_chip);
                            }
                            if let ::core::option::Option::Some(prepared_estimated_cycles) =
                                prepared_estimated_cycles
                            {
                                object_writer.write::<_, _, 4>(&prepared_estimated_cycles);
                            }
                            if let ::core::option::Option::Some(
                                prepared_used_narrow_memory_bytes_per_tile,
                            ) = prepared_used_narrow_memory_bytes_per_tile
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_used_narrow_memory_bytes_per_tile);
                            }
                            if let ::core::option::Option::Some(prepared_type_) = prepared_type_ {
                                object_writer.write::<_, _, 2>(&prepared_type_);
                            }
                            if let ::core::option::Option::Some(
                                prepared_use_tpu_dram_for_parameters,
                            ) = prepared_use_tpu_dram_for_parameters
                            {
                                object_writer
                                    .write::<_, _, 1>(&prepared_use_tpu_dram_for_parameters);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<Executable>> for Executable {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Executable> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<Executable>> for Executable {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Executable>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<Executable> for Executable {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Executable> {
                    Executable::create(
                        builder,
                        self.version,
                        &self.name,
                        &self.serialized_model,
                        self.batch_size,
                        self.scratch_size_bytes,
                        &self.instruction_bitstreams,
                        &self.parameters,
                        &self.dma_hints,
                        &self.input_layers,
                        &self.output_layers,
                        &self.chip,
                        self.estimated_cycles,
                        self.used_narrow_memory_bytes_per_tile,
                        self.type_,
                        self.parameter_caching_token,
                        self.use_tpu_dram_for_parameters,
                        self.estimated_cycles_64bit,
                    )
                }
            }

            /// Builder for serializing an instance of the [Executable] type.
            ///
            /// Can be created using the [Executable::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct ExecutableBuilder<State>(State);

            impl ExecutableBuilder<()> {
                /// Setter for the [`version` field](Executable#structfield.version).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn version<T0>(self, value: T0) -> ExecutableBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<i32, i32>,
                {
                    ExecutableBuilder((value,))
                }

                /// Sets the [`version` field](Executable#structfield.version) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn version_as_default(self) -> ExecutableBuilder<(::planus::DefaultValue,)> {
                    self.version(::planus::DefaultValue)
                }
            }

            impl<T0> ExecutableBuilder<(T0,)> {
                /// Setter for the [`name` field](Executable#structfield.name).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name<T1>(self, value: T1) -> ExecutableBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0,) = self.0;
                    ExecutableBuilder((v0, value))
                }

                /// Sets the [`name` field](Executable#structfield.name) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name_as_null(self) -> ExecutableBuilder<(T0, ())> {
                    self.name(())
                }
            }

            impl<T0, T1> ExecutableBuilder<(T0, T1)> {
                /// Setter for the [`serialized_model` field](Executable#structfield.serialized_model).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_model<T2>(self, value: T2) -> ExecutableBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    let (v0, v1) = self.0;
                    ExecutableBuilder((v0, v1, value))
                }

                /// Sets the [`serialized_model` field](Executable#structfield.serialized_model) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_model_as_null(self) -> ExecutableBuilder<(T0, T1, ())> {
                    self.serialized_model(())
                }
            }

            impl<T0, T1, T2> ExecutableBuilder<(T0, T1, T2)> {
                /// Setter for the [`batch_size` field](Executable#structfield.batch_size).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn batch_size<T3>(self, value: T3) -> ExecutableBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2) = self.0;
                    ExecutableBuilder((v0, v1, v2, value))
                }

                /// Sets the [`batch_size` field](Executable#structfield.batch_size) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn batch_size_as_default(
                    self,
                ) -> ExecutableBuilder<(T0, T1, T2, ::planus::DefaultValue)> {
                    self.batch_size(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3> ExecutableBuilder<(T0, T1, T2, T3)> {
                /// Setter for the [`scratch_size_bytes` field](Executable#structfield.scratch_size_bytes).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn scratch_size_bytes<T4>(
                    self,
                    value: T4,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4)>
                where
                    T4: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2, v3) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, value))
                }

                /// Sets the [`scratch_size_bytes` field](Executable#structfield.scratch_size_bytes) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn scratch_size_bytes_as_default(
                    self,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)> {
                    self.scratch_size_bytes(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4> ExecutableBuilder<(T0, T1, T2, T3, T4)> {
                /// Setter for the [`instruction_bitstreams` field](Executable#structfield.instruction_bitstreams).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn instruction_bitstreams<T5>(
                    self,
                    value: T5,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5)>
                where
                    T5: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::InstructionBitstream>]>,
                    >,
                {
                    let (v0, v1, v2, v3, v4) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, value))
                }

                /// Sets the [`instruction_bitstreams` field](Executable#structfield.instruction_bitstreams) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn instruction_bitstreams_as_null(
                    self,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, ())> {
                    self.instruction_bitstreams(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5> ExecutableBuilder<(T0, T1, T2, T3, T4, T5)> {
                /// Setter for the [`parameters` field](Executable#structfield.parameters).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn parameters<T6>(
                    self,
                    value: T6,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6)>
                where
                    T6: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    let (v0, v1, v2, v3, v4, v5) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, v5, value))
                }

                /// Sets the [`parameters` field](Executable#structfield.parameters) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn parameters_as_null(self) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, ())> {
                    self.parameters(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6)> {
                /// Setter for the [`dma_hints` field](Executable#structfield.dma_hints).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn dma_hints<T7>(
                    self,
                    value: T7,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
                where
                    T7: ::planus::WriteAsOptional<::planus::Offset<self::DmaHints>>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, v5, v6, value))
                }

                /// Sets the [`dma_hints` field](Executable#structfield.dma_hints) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn dma_hints_as_null(
                    self,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, ())> {
                    self.dma_hints(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)> {
                /// Setter for the [`input_layers` field](Executable#structfield.input_layers).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn input_layers<T8>(
                    self,
                    value: T8,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
                where
                    T8: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::Layer>]>,
                    >,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, v5, v6, v7, value))
                }

                /// Sets the [`input_layers` field](Executable#structfield.input_layers) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn input_layers_as_null(
                    self,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, ())> {
                    self.input_layers(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
                /// Setter for the [`output_layers` field](Executable#structfield.output_layers).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn output_layers<T9>(
                    self,
                    value: T9,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
                where
                    T9: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::Layer>]>,
                    >,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, value))
                }

                /// Sets the [`output_layers` field](Executable#structfield.output_layers) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn output_layers_as_null(
                    self,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, ())> {
                    self.output_layers(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
                ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
            {
                /// Setter for the [`chip` field](Executable#structfield.chip).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn chip<T10>(
                    self,
                    value: T10,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
                where
                    T10: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, value))
                }

                /// Sets the [`chip` field](Executable#structfield.chip) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn chip_as_null(
                    self,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, ())>
                {
                    self.chip(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
                ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
            {
                /// Setter for the [`estimated_cycles` field](Executable#structfield.estimated_cycles).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn estimated_cycles<T11>(
                    self,
                    value: T11,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
                where
                    T11: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, value))
                }

                /// Sets the [`estimated_cycles` field](Executable#structfield.estimated_cycles) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn estimated_cycles_as_default(
                    self,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    ::planus::DefaultValue,
                )> {
                    self.estimated_cycles(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
                ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
            {
                /// Setter for the [`used_narrow_memory_bytes_per_tile` field](Executable#structfield.used_narrow_memory_bytes_per_tile).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn used_narrow_memory_bytes_per_tile<T12>(
                    self,
                    value: T12,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
                where
                    T12: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = self.0;
                    ExecutableBuilder((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, value))
                }

                /// Sets the [`used_narrow_memory_bytes_per_tile` field](Executable#structfield.used_narrow_memory_bytes_per_tile) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn used_narrow_memory_bytes_per_tile_as_default(
                    self,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    ::planus::DefaultValue,
                )> {
                    self.used_narrow_memory_bytes_per_tile(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
                ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
            {
                /// Setter for the [`type` field](Executable#structfield.type_).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_<T13>(
                    self,
                    value: T13,
                ) -> ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>
                where
                    T13: ::planus::WriteAsDefault<self::ExecutableType, self::ExecutableType>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = self.0;
                    ExecutableBuilder((
                        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, value,
                    ))
                }

                /// Sets the [`type` field](Executable#structfield.type_) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_as_default(
                    self,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    ::planus::DefaultValue,
                )> {
                    self.type_(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
                ExecutableBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>
            {
                /// Setter for the [`parameter_caching_token` field](Executable#structfield.parameter_caching_token).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn parameter_caching_token<T14>(
                    self,
                    value: T14,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                )>
                where
                    T14: ::planus::WriteAsDefault<u64, u64>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) = self.0;
                    ExecutableBuilder((
                        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, value,
                    ))
                }

                /// Sets the [`parameter_caching_token` field](Executable#structfield.parameter_caching_token) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn parameter_caching_token_as_default(
                    self,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    ::planus::DefaultValue,
                )> {
                    self.parameter_caching_token(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
                ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                )>
            {
                /// Setter for the [`use_tpu_dram_for_parameters` field](Executable#structfield.use_tpu_dram_for_parameters).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn use_tpu_dram_for_parameters<T15>(
                    self,
                    value: T15,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                )>
                where
                    T15: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = self.0;
                    ExecutableBuilder((
                        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, value,
                    ))
                }

                /// Sets the [`use_tpu_dram_for_parameters` field](Executable#structfield.use_tpu_dram_for_parameters) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn use_tpu_dram_for_parameters_as_default(
                    self,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    ::planus::DefaultValue,
                )> {
                    self.use_tpu_dram_for_parameters(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
                ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                )>
            {
                /// Setter for the [`estimated_cycles_64bit` field](Executable#structfield.estimated_cycles_64bit).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn estimated_cycles_64bit<T16>(
                    self,
                    value: T16,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                )>
                where
                    T16: ::planus::WriteAsDefault<i64, i64>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) =
                        self.0;
                    ExecutableBuilder((
                        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, value,
                    ))
                }

                /// Sets the [`estimated_cycles_64bit` field](Executable#structfield.estimated_cycles_64bit) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn estimated_cycles_64bit_as_default(
                    self,
                ) -> ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    ::planus::DefaultValue,
                )> {
                    self.estimated_cycles_64bit(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
                ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                )>
            {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Executable].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Executable>
                where
                    Self: ::planus::WriteAsOffset<Executable>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsDefault<i32, i32>,
                    T5: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::InstructionBitstream>]>,
                    >,
                    T6: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T7: ::planus::WriteAsOptional<::planus::Offset<self::DmaHints>>,
                    T8: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::Layer>]>>,
                    T9: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::Layer>]>>,
                    T10: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T11: ::planus::WriteAsDefault<i32, i32>,
                    T12: ::planus::WriteAsDefault<i32, i32>,
                    T13: ::planus::WriteAsDefault<self::ExecutableType, self::ExecutableType>,
                    T14: ::planus::WriteAsDefault<u64, u64>,
                    T15: ::planus::WriteAsDefault<bool, bool>,
                    T16: ::planus::WriteAsDefault<i64, i64>,
                > ::planus::WriteAs<::planus::Offset<Executable>>
                for ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                )>
            {
                type Prepared = ::planus::Offset<Executable>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Executable> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsDefault<i32, i32>,
                    T5: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::InstructionBitstream>]>,
                    >,
                    T6: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T7: ::planus::WriteAsOptional<::planus::Offset<self::DmaHints>>,
                    T8: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::Layer>]>>,
                    T9: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::Layer>]>>,
                    T10: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T11: ::planus::WriteAsDefault<i32, i32>,
                    T12: ::planus::WriteAsDefault<i32, i32>,
                    T13: ::planus::WriteAsDefault<self::ExecutableType, self::ExecutableType>,
                    T14: ::planus::WriteAsDefault<u64, u64>,
                    T15: ::planus::WriteAsDefault<bool, bool>,
                    T16: ::planus::WriteAsDefault<i64, i64>,
                > ::planus::WriteAsOptional<::planus::Offset<Executable>>
                for ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                )>
            {
                type Prepared = ::planus::Offset<Executable>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Executable>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsDefault<i32, i32>,
                    T5: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::InstructionBitstream>]>,
                    >,
                    T6: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T7: ::planus::WriteAsOptional<::planus::Offset<self::DmaHints>>,
                    T8: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::Layer>]>>,
                    T9: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<self::Layer>]>>,
                    T10: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T11: ::planus::WriteAsDefault<i32, i32>,
                    T12: ::planus::WriteAsDefault<i32, i32>,
                    T13: ::planus::WriteAsDefault<self::ExecutableType, self::ExecutableType>,
                    T14: ::planus::WriteAsDefault<u64, u64>,
                    T15: ::planus::WriteAsDefault<bool, bool>,
                    T16: ::planus::WriteAsDefault<i64, i64>,
                > ::planus::WriteAsOffset<Executable>
                for ExecutableBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                )>
            {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Executable> {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) =
                        &self.0;
                    Executable::create(
                        builder, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14,
                        v15, v16,
                    )
                }
            }

            /// Reference to a deserialized [Executable].
            #[derive(Copy, Clone)]
            pub struct ExecutableRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> ExecutableRef<'a> {
                /// Getter for the [`version` field](Executable#structfield.version).
                #[inline]
                pub fn version(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0.access(0, "Executable", "version")?.unwrap_or(0),
                    )
                }

                /// Getter for the [`name` field](Executable#structfield.name).
                #[inline]
                pub fn name(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(1, "Executable", "name")
                }

                /// Getter for the [`serialized_model` field](Executable#structfield.serialized_model).
                #[inline]
                pub fn serialized_model(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(2, "Executable", "serialized_model")
                }

                /// Getter for the [`batch_size` field](Executable#structfield.batch_size).
                #[inline]
                pub fn batch_size(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0.access(3, "Executable", "batch_size")?.unwrap_or(0),
                    )
                }

                /// Getter for the [`scratch_size_bytes` field](Executable#structfield.scratch_size_bytes).
                #[inline]
                pub fn scratch_size_bytes(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(4, "Executable", "scratch_size_bytes")?
                            .unwrap_or(0),
                    )
                }

                /// Getter for the [`instruction_bitstreams` field](Executable#structfield.instruction_bitstreams).
                #[inline]
                pub fn instruction_bitstreams(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::InstructionBitstreamRef<'a>>>,
                    >,
                > {
                    self.0.access(5, "Executable", "instruction_bitstreams")
                }

                /// Getter for the [`parameters` field](Executable#structfield.parameters).
                #[inline]
                pub fn parameters(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(6, "Executable", "parameters")
                }

                /// Getter for the [`dma_hints` field](Executable#structfield.dma_hints).
                #[inline]
                pub fn dma_hints(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::DmaHintsRef<'a>>>
                {
                    self.0.access(7, "Executable", "dma_hints")
                }

                /// Getter for the [`input_layers` field](Executable#structfield.input_layers).
                #[inline]
                pub fn input_layers(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::LayerRef<'a>>>,
                    >,
                > {
                    self.0.access(8, "Executable", "input_layers")
                }

                /// Getter for the [`output_layers` field](Executable#structfield.output_layers).
                #[inline]
                pub fn output_layers(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::LayerRef<'a>>>,
                    >,
                > {
                    self.0.access(9, "Executable", "output_layers")
                }

                /// Getter for the [`chip` field](Executable#structfield.chip).
                #[inline]
                pub fn chip(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(10, "Executable", "chip")
                }

                /// Getter for the [`estimated_cycles` field](Executable#structfield.estimated_cycles).
                #[inline]
                pub fn estimated_cycles(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(11, "Executable", "estimated_cycles")?
                            .unwrap_or(0),
                    )
                }

                /// Getter for the [`used_narrow_memory_bytes_per_tile` field](Executable#structfield.used_narrow_memory_bytes_per_tile).
                #[inline]
                pub fn used_narrow_memory_bytes_per_tile(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(12, "Executable", "used_narrow_memory_bytes_per_tile")?
                            .unwrap_or(0),
                    )
                }

                /// Getter for the [`type` field](Executable#structfield.type_).
                #[inline]
                pub fn type_(&self) -> ::planus::Result<self::ExecutableType> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(13, "Executable", "type_")?
                            .unwrap_or(self::ExecutableType::StandAlone),
                    )
                }

                /// Getter for the [`parameter_caching_token` field](Executable#structfield.parameter_caching_token).
                #[inline]
                pub fn parameter_caching_token(&self) -> ::planus::Result<u64> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(14, "Executable", "parameter_caching_token")?
                            .unwrap_or(0),
                    )
                }

                /// Getter for the [`use_tpu_dram_for_parameters` field](Executable#structfield.use_tpu_dram_for_parameters).
                #[inline]
                pub fn use_tpu_dram_for_parameters(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(15, "Executable", "use_tpu_dram_for_parameters")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`estimated_cycles_64bit` field](Executable#structfield.estimated_cycles_64bit).
                #[inline]
                pub fn estimated_cycles_64bit(&self) -> ::planus::Result<i64> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(16, "Executable", "estimated_cycles_64bit")?
                            .unwrap_or(0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for ExecutableRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("ExecutableRef");
                    f.field("version", &self.version());
                    if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                        f.field("name", &field_name);
                    }
                    if let ::core::option::Option::Some(field_serialized_model) =
                        self.serialized_model().transpose()
                    {
                        f.field("serialized_model", &field_serialized_model);
                    }
                    f.field("batch_size", &self.batch_size());
                    f.field("scratch_size_bytes", &self.scratch_size_bytes());
                    if let ::core::option::Option::Some(field_instruction_bitstreams) =
                        self.instruction_bitstreams().transpose()
                    {
                        f.field("instruction_bitstreams", &field_instruction_bitstreams);
                    }
                    if let ::core::option::Option::Some(field_parameters) =
                        self.parameters().transpose()
                    {
                        f.field("parameters", &field_parameters);
                    }
                    if let ::core::option::Option::Some(field_dma_hints) =
                        self.dma_hints().transpose()
                    {
                        f.field("dma_hints", &field_dma_hints);
                    }
                    if let ::core::option::Option::Some(field_input_layers) =
                        self.input_layers().transpose()
                    {
                        f.field("input_layers", &field_input_layers);
                    }
                    if let ::core::option::Option::Some(field_output_layers) =
                        self.output_layers().transpose()
                    {
                        f.field("output_layers", &field_output_layers);
                    }
                    if let ::core::option::Option::Some(field_chip) = self.chip().transpose() {
                        f.field("chip", &field_chip);
                    }
                    f.field("estimated_cycles", &self.estimated_cycles());
                    f.field(
                        "used_narrow_memory_bytes_per_tile",
                        &self.used_narrow_memory_bytes_per_tile(),
                    );
                    f.field("type_", &self.type_());
                    f.field("parameter_caching_token", &self.parameter_caching_token());
                    f.field(
                        "use_tpu_dram_for_parameters",
                        &self.use_tpu_dram_for_parameters(),
                    );
                    f.field("estimated_cycles_64bit", &self.estimated_cycles_64bit());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<ExecutableRef<'a>> for Executable {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: ExecutableRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        version: ::core::convert::TryInto::try_into(value.version()?)?,
                        name: value.name()?.map(::core::convert::Into::into),
                        serialized_model: value.serialized_model()?.map(|v| v.to_vec()),
                        batch_size: ::core::convert::TryInto::try_into(value.batch_size()?)?,
                        scratch_size_bytes: ::core::convert::TryInto::try_into(
                            value.scratch_size_bytes()?,
                        )?,
                        instruction_bitstreams: if let ::core::option::Option::Some(
                            instruction_bitstreams,
                        ) = value.instruction_bitstreams()?
                        {
                            ::core::option::Option::Some(instruction_bitstreams.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        parameters: value.parameters()?.map(|v| v.to_vec()),
                        dma_hints: if let ::core::option::Option::Some(dma_hints) =
                            value.dma_hints()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(dma_hints)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        input_layers: if let ::core::option::Option::Some(input_layers) =
                            value.input_layers()?
                        {
                            ::core::option::Option::Some(input_layers.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        output_layers: if let ::core::option::Option::Some(output_layers) =
                            value.output_layers()?
                        {
                            ::core::option::Option::Some(output_layers.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        chip: value.chip()?.map(::core::convert::Into::into),
                        estimated_cycles: ::core::convert::TryInto::try_into(
                            value.estimated_cycles()?,
                        )?,
                        used_narrow_memory_bytes_per_tile: ::core::convert::TryInto::try_into(
                            value.used_narrow_memory_bytes_per_tile()?,
                        )?,
                        type_: ::core::convert::TryInto::try_into(value.type_()?)?,
                        parameter_caching_token: ::core::convert::TryInto::try_into(
                            value.parameter_caching_token()?,
                        )?,
                        use_tpu_dram_for_parameters: ::core::convert::TryInto::try_into(
                            value.use_tpu_dram_for_parameters()?,
                        )?,
                        estimated_cycles_64bit: ::core::convert::TryInto::try_into(
                            value.estimated_cycles_64bit()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for ExecutableRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for ExecutableRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[ExecutableRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<Executable>> for Executable {
                type Value = ::planus::Offset<Executable>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<Executable>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for ExecutableRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[ExecutableRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `MultiExecutable` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `MultiExecutable` in the file `schema/executable.fbs:429`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct MultiExecutable {
                /// The field `serialized_executables` in the table `MultiExecutable`
                pub serialized_executables: ::core::option::Option<
                    ::planus::alloc::vec::Vec<::planus::alloc::string::String>,
                >,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for MultiExecutable {
                fn default() -> Self {
                    Self {
                        serialized_executables: ::core::default::Default::default(),
                    }
                }
            }

            impl MultiExecutable {
                /// Creates a [MultiExecutableBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> MultiExecutableBuilder<()> {
                    MultiExecutableBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_serialized_executables: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<str>]>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_serialized_executables =
                        field_serialized_executables.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<6> =
                        ::core::default::Default::default();
                    if prepared_serialized_executables.is_some() {
                        table_writer.write_entry::<::planus::Offset<[::planus::Offset<str>]>>(0);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_serialized_executables) =
                                prepared_serialized_executables
                            {
                                object_writer.write::<_, _, 4>(&prepared_serialized_executables);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<MultiExecutable>> for MultiExecutable {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<MultiExecutable> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<MultiExecutable>> for MultiExecutable {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<MultiExecutable>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<MultiExecutable> for MultiExecutable {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<MultiExecutable> {
                    MultiExecutable::create(builder, &self.serialized_executables)
                }
            }

            /// Builder for serializing an instance of the [MultiExecutable] type.
            ///
            /// Can be created using the [MultiExecutable::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct MultiExecutableBuilder<State>(State);

            impl MultiExecutableBuilder<()> {
                /// Setter for the [`serialized_executables` field](MultiExecutable#structfield.serialized_executables).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_executables<T0>(self, value: T0) -> MultiExecutableBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<str>]>>,
                {
                    MultiExecutableBuilder((value,))
                }

                /// Sets the [`serialized_executables` field](MultiExecutable#structfield.serialized_executables) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_executables_as_null(self) -> MultiExecutableBuilder<((),)> {
                    self.serialized_executables(())
                }
            }

            impl<T0> MultiExecutableBuilder<(T0,)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [MultiExecutable].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<MultiExecutable>
                where
                    Self: ::planus::WriteAsOffset<MultiExecutable>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<str>]>>>
                ::planus::WriteAs<::planus::Offset<MultiExecutable>>
                for MultiExecutableBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<MultiExecutable>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<MultiExecutable> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<str>]>>>
                ::planus::WriteAsOptional<::planus::Offset<MultiExecutable>>
                for MultiExecutableBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<MultiExecutable>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<MultiExecutable>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[::planus::Offset<str>]>>>
                ::planus::WriteAsOffset<MultiExecutable> for MultiExecutableBuilder<(T0,)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<MultiExecutable> {
                    let (v0,) = &self.0;
                    MultiExecutable::create(builder, v0)
                }
            }

            /// Reference to a deserialized [MultiExecutable].
            #[derive(Copy, Clone)]
            pub struct MultiExecutableRef<'a>(
                #[allow(dead_code)] ::planus::table_reader::Table<'a>,
            );

            impl<'a> MultiExecutableRef<'a> {
                /// Getter for the [`serialized_executables` field](MultiExecutable#structfield.serialized_executables).
                #[inline]
                pub fn serialized_executables(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<&'a ::core::primitive::str>>,
                    >,
                > {
                    self.0
                        .access(0, "MultiExecutable", "serialized_executables")
                }
            }

            impl<'a> ::core::fmt::Debug for MultiExecutableRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("MultiExecutableRef");
                    if let ::core::option::Option::Some(field_serialized_executables) =
                        self.serialized_executables().transpose()
                    {
                        f.field("serialized_executables", &field_serialized_executables);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<MultiExecutableRef<'a>> for MultiExecutable {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: MultiExecutableRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        serialized_executables: if let ::core::option::Option::Some(
                            serialized_executables,
                        ) = value.serialized_executables()?
                        {
                            ::core::option::Option::Some(serialized_executables.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for MultiExecutableRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for MultiExecutableRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[MultiExecutableRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<MultiExecutable>> for MultiExecutable {
                type Value = ::planus::Offset<MultiExecutable>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<MultiExecutable>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for MultiExecutableRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[MultiExecutableRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `SerializedPackage` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `SerializedPackage` in the file `schema/executable.fbs:435`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct SerializedPackage {
                /// The field `serialized_package` in the table `SerializedPackage`
                pub serialized_package: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for SerializedPackage {
                fn default() -> Self {
                    Self {
                        serialized_package: ::core::default::Default::default(),
                    }
                }
            }

            impl SerializedPackage {
                /// Creates a [SerializedPackageBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> SerializedPackageBuilder<()> {
                    SerializedPackageBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_serialized_package: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                ) -> ::planus::Offset<Self> {
                    let prepared_serialized_package = field_serialized_package.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<6> =
                        ::core::default::Default::default();
                    if prepared_serialized_package.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(0);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_serialized_package) =
                                prepared_serialized_package
                            {
                                object_writer.write::<_, _, 4>(&prepared_serialized_package);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<SerializedPackage>> for SerializedPackage {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<SerializedPackage> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<SerializedPackage>> for SerializedPackage {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<SerializedPackage>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<SerializedPackage> for SerializedPackage {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<SerializedPackage> {
                    SerializedPackage::create(builder, &self.serialized_package)
                }
            }

            /// Builder for serializing an instance of the [SerializedPackage] type.
            ///
            /// Can be created using the [SerializedPackage::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct SerializedPackageBuilder<State>(State);

            impl SerializedPackageBuilder<()> {
                /// Setter for the [`serialized_package` field](SerializedPackage#structfield.serialized_package).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_package<T0>(self, value: T0) -> SerializedPackageBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    SerializedPackageBuilder((value,))
                }

                /// Sets the [`serialized_package` field](SerializedPackage#structfield.serialized_package) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_package_as_null(self) -> SerializedPackageBuilder<((),)> {
                    self.serialized_package(())
                }
            }

            impl<T0> SerializedPackageBuilder<(T0,)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [SerializedPackage].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<SerializedPackage>
                where
                    Self: ::planus::WriteAsOffset<SerializedPackage>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>>
                ::planus::WriteAs<::planus::Offset<SerializedPackage>>
                for SerializedPackageBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<SerializedPackage>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<SerializedPackage> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>>
                ::planus::WriteAsOptional<::planus::Offset<SerializedPackage>>
                for SerializedPackageBuilder<(T0,)>
            {
                type Prepared = ::planus::Offset<SerializedPackage>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<SerializedPackage>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>>
                ::planus::WriteAsOffset<SerializedPackage> for SerializedPackageBuilder<(T0,)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<SerializedPackage> {
                    let (v0,) = &self.0;
                    SerializedPackage::create(builder, v0)
                }
            }

            /// Reference to a deserialized [SerializedPackage].
            #[derive(Copy, Clone)]
            pub struct SerializedPackageRef<'a>(
                #[allow(dead_code)] ::planus::table_reader::Table<'a>,
            );

            impl<'a> SerializedPackageRef<'a> {
                /// Getter for the [`serialized_package` field](SerializedPackage#structfield.serialized_package).
                #[inline]
                pub fn serialized_package(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(0, "SerializedPackage", "serialized_package")
                }
            }

            impl<'a> ::core::fmt::Debug for SerializedPackageRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("SerializedPackageRef");
                    if let ::core::option::Option::Some(field_serialized_package) =
                        self.serialized_package().transpose()
                    {
                        f.field("serialized_package", &field_serialized_package);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<SerializedPackageRef<'a>> for SerializedPackage {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: SerializedPackageRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        serialized_package: value.serialized_package()?.map(|v| v.to_vec()),
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for SerializedPackageRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for SerializedPackageRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[SerializedPackageRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<SerializedPackage>> for SerializedPackage {
                type Value = ::planus::Offset<SerializedPackage>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<SerializedPackage>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for SerializedPackageRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[SerializedPackageRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `Package` in the namespace `platforms.darwinn`
            ///
            /// Generated from these locations:
            /// * Table `Package` in the file `schema/executable.fbs:441`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct Package {
                /// The field `min_runtime_version` in the table `Package`
                pub min_runtime_version: i32,
                /// The field `serialized_multi_executable` in the table `Package`
                pub serialized_multi_executable:
                    ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `signature` in the table `Package`
                pub signature: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `keypair_version` in the table `Package`
                pub keypair_version: i32,
                /// The field `compiler_version` in the table `Package`
                pub compiler_version: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `virtual_chip_id` in the table `Package`
                pub virtual_chip_id: i32,
                /// The field `multi_chip_package` in the table `Package`
                pub multi_chip_package:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::SerializedPackage>>,
                /// The field `model_identifier` in the table `Package`
                pub model_identifier: ::core::option::Option<::planus::alloc::string::String>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for Package {
                fn default() -> Self {
                    Self {
                        min_runtime_version: 0,
                        serialized_multi_executable: ::core::default::Default::default(),
                        signature: ::core::default::Default::default(),
                        keypair_version: 0,
                        compiler_version: ::core::default::Default::default(),
                        virtual_chip_id: 0,
                        multi_chip_package: ::core::default::Default::default(),
                        model_identifier: ::core::default::Default::default(),
                    }
                }
            }

            impl Package {
                /// Creates a [PackageBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> PackageBuilder<()> {
                    PackageBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_min_runtime_version: impl ::planus::WriteAsDefault<i32, i32>,
                    field_serialized_multi_executable: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[u8]>,
                    >,
                    field_signature: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_keypair_version: impl ::planus::WriteAsDefault<i32, i32>,
                    field_compiler_version: impl ::planus::WriteAsOptional<
                        ::planus::Offset<::core::primitive::str>,
                    >,
                    field_virtual_chip_id: impl ::planus::WriteAsDefault<i32, i32>,
                    field_multi_chip_package: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::SerializedPackage>]>,
                    >,
                    field_model_identifier: impl ::planus::WriteAsOptional<
                        ::planus::Offset<::core::primitive::str>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_min_runtime_version =
                        field_min_runtime_version.prepare(builder, &0);
                    let prepared_serialized_multi_executable =
                        field_serialized_multi_executable.prepare(builder);
                    let prepared_signature = field_signature.prepare(builder);
                    let prepared_keypair_version = field_keypair_version.prepare(builder, &0);
                    let prepared_compiler_version = field_compiler_version.prepare(builder);
                    let prepared_virtual_chip_id = field_virtual_chip_id.prepare(builder, &0);
                    let prepared_multi_chip_package = field_multi_chip_package.prepare(builder);
                    let prepared_model_identifier = field_model_identifier.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<20> =
                        ::core::default::Default::default();
                    if prepared_min_runtime_version.is_some() {
                        table_writer.write_entry::<i32>(0);
                    }
                    if prepared_serialized_multi_executable.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(1);
                    }
                    if prepared_signature.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(2);
                    }
                    if prepared_keypair_version.is_some() {
                        table_writer.write_entry::<i32>(3);
                    }
                    if prepared_compiler_version.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(4);
                    }
                    if prepared_virtual_chip_id.is_some() {
                        table_writer.write_entry::<i32>(5);
                    }
                    if prepared_multi_chip_package.is_some() {
                        table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::SerializedPackage>]>>(6);
                    }
                    if prepared_model_identifier.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(7);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_min_runtime_version) =
                                prepared_min_runtime_version
                            {
                                object_writer.write::<_, _, 4>(&prepared_min_runtime_version);
                            }
                            if let ::core::option::Option::Some(
                                prepared_serialized_multi_executable,
                            ) = prepared_serialized_multi_executable
                            {
                                object_writer
                                    .write::<_, _, 4>(&prepared_serialized_multi_executable);
                            }
                            if let ::core::option::Option::Some(prepared_signature) =
                                prepared_signature
                            {
                                object_writer.write::<_, _, 4>(&prepared_signature);
                            }
                            if let ::core::option::Option::Some(prepared_keypair_version) =
                                prepared_keypair_version
                            {
                                object_writer.write::<_, _, 4>(&prepared_keypair_version);
                            }
                            if let ::core::option::Option::Some(prepared_compiler_version) =
                                prepared_compiler_version
                            {
                                object_writer.write::<_, _, 4>(&prepared_compiler_version);
                            }
                            if let ::core::option::Option::Some(prepared_virtual_chip_id) =
                                prepared_virtual_chip_id
                            {
                                object_writer.write::<_, _, 4>(&prepared_virtual_chip_id);
                            }
                            if let ::core::option::Option::Some(prepared_multi_chip_package) =
                                prepared_multi_chip_package
                            {
                                object_writer.write::<_, _, 4>(&prepared_multi_chip_package);
                            }
                            if let ::core::option::Option::Some(prepared_model_identifier) =
                                prepared_model_identifier
                            {
                                object_writer.write::<_, _, 4>(&prepared_model_identifier);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<Package>> for Package {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Package> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<Package>> for Package {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Package>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<Package> for Package {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Package> {
                    Package::create(
                        builder,
                        self.min_runtime_version,
                        &self.serialized_multi_executable,
                        &self.signature,
                        self.keypair_version,
                        &self.compiler_version,
                        self.virtual_chip_id,
                        &self.multi_chip_package,
                        &self.model_identifier,
                    )
                }
            }

            /// Builder for serializing an instance of the [Package] type.
            ///
            /// Can be created using the [Package::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct PackageBuilder<State>(State);

            impl PackageBuilder<()> {
                /// Setter for the [`min_runtime_version` field](Package#structfield.min_runtime_version).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn min_runtime_version<T0>(self, value: T0) -> PackageBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<i32, i32>,
                {
                    PackageBuilder((value,))
                }

                /// Sets the [`min_runtime_version` field](Package#structfield.min_runtime_version) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn min_runtime_version_as_default(
                    self,
                ) -> PackageBuilder<(::planus::DefaultValue,)> {
                    self.min_runtime_version(::planus::DefaultValue)
                }
            }

            impl<T0> PackageBuilder<(T0,)> {
                /// Setter for the [`serialized_multi_executable` field](Package#structfield.serialized_multi_executable).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_multi_executable<T1>(self, value: T1) -> PackageBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    let (v0,) = self.0;
                    PackageBuilder((v0, value))
                }

                /// Sets the [`serialized_multi_executable` field](Package#structfield.serialized_multi_executable) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn serialized_multi_executable_as_null(self) -> PackageBuilder<(T0, ())> {
                    self.serialized_multi_executable(())
                }
            }

            impl<T0, T1> PackageBuilder<(T0, T1)> {
                /// Setter for the [`signature` field](Package#structfield.signature).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn signature<T2>(self, value: T2) -> PackageBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    let (v0, v1) = self.0;
                    PackageBuilder((v0, v1, value))
                }

                /// Sets the [`signature` field](Package#structfield.signature) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn signature_as_null(self) -> PackageBuilder<(T0, T1, ())> {
                    self.signature(())
                }
            }

            impl<T0, T1, T2> PackageBuilder<(T0, T1, T2)> {
                /// Setter for the [`keypair_version` field](Package#structfield.keypair_version).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn keypair_version<T3>(self, value: T3) -> PackageBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2) = self.0;
                    PackageBuilder((v0, v1, v2, value))
                }

                /// Sets the [`keypair_version` field](Package#structfield.keypair_version) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn keypair_version_as_default(
                    self,
                ) -> PackageBuilder<(T0, T1, T2, ::planus::DefaultValue)> {
                    self.keypair_version(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3> PackageBuilder<(T0, T1, T2, T3)> {
                /// Setter for the [`compiler_version` field](Package#structfield.compiler_version).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn compiler_version<T4>(self, value: T4) -> PackageBuilder<(T0, T1, T2, T3, T4)>
                where
                    T4: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0, v1, v2, v3) = self.0;
                    PackageBuilder((v0, v1, v2, v3, value))
                }

                /// Sets the [`compiler_version` field](Package#structfield.compiler_version) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn compiler_version_as_null(self) -> PackageBuilder<(T0, T1, T2, T3, ())> {
                    self.compiler_version(())
                }
            }

            impl<T0, T1, T2, T3, T4> PackageBuilder<(T0, T1, T2, T3, T4)> {
                /// Setter for the [`virtual_chip_id` field](Package#structfield.virtual_chip_id).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn virtual_chip_id<T5>(
                    self,
                    value: T5,
                ) -> PackageBuilder<(T0, T1, T2, T3, T4, T5)>
                where
                    T5: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0, v1, v2, v3, v4) = self.0;
                    PackageBuilder((v0, v1, v2, v3, v4, value))
                }

                /// Sets the [`virtual_chip_id` field](Package#structfield.virtual_chip_id) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn virtual_chip_id_as_default(
                    self,
                ) -> PackageBuilder<(T0, T1, T2, T3, T4, ::planus::DefaultValue)> {
                    self.virtual_chip_id(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5> PackageBuilder<(T0, T1, T2, T3, T4, T5)> {
                /// Setter for the [`multi_chip_package` field](Package#structfield.multi_chip_package).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn multi_chip_package<T6>(
                    self,
                    value: T6,
                ) -> PackageBuilder<(T0, T1, T2, T3, T4, T5, T6)>
                where
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::SerializedPackage>]>,
                    >,
                {
                    let (v0, v1, v2, v3, v4, v5) = self.0;
                    PackageBuilder((v0, v1, v2, v3, v4, v5, value))
                }

                /// Sets the [`multi_chip_package` field](Package#structfield.multi_chip_package) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn multi_chip_package_as_null(
                    self,
                ) -> PackageBuilder<(T0, T1, T2, T3, T4, T5, ())> {
                    self.multi_chip_package(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6> PackageBuilder<(T0, T1, T2, T3, T4, T5, T6)> {
                /// Setter for the [`model_identifier` field](Package#structfield.model_identifier).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn model_identifier<T7>(
                    self,
                    value: T7,
                ) -> PackageBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
                where
                    T7: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0, v1, v2, v3, v4, v5, v6) = self.0;
                    PackageBuilder((v0, v1, v2, v3, v4, v5, v6, value))
                }

                /// Sets the [`model_identifier` field](Package#structfield.model_identifier) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn model_identifier_as_null(
                    self,
                ) -> PackageBuilder<(T0, T1, T2, T3, T4, T5, T6, ())> {
                    self.model_identifier(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7> PackageBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Package].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Package>
                where
                    Self: ::planus::WriteAsOffset<Package>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T5: ::planus::WriteAsDefault<i32, i32>,
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::SerializedPackage>]>,
                    >,
                    T7: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAs<::planus::Offset<Package>>
                for PackageBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
            {
                type Prepared = ::planus::Offset<Package>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Package> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T5: ::planus::WriteAsDefault<i32, i32>,
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::SerializedPackage>]>,
                    >,
                    T7: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAsOptional<::planus::Offset<Package>>
                for PackageBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
            {
                type Prepared = ::planus::Offset<Package>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<Package>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<i32, i32>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T3: ::planus::WriteAsDefault<i32, i32>,
                    T4: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T5: ::planus::WriteAsDefault<i32, i32>,
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::SerializedPackage>]>,
                    >,
                    T7: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAsOffset<Package>
                for PackageBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
            {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Package> {
                    let (v0, v1, v2, v3, v4, v5, v6, v7) = &self.0;
                    Package::create(builder, v0, v1, v2, v3, v4, v5, v6, v7)
                }
            }

            /// Reference to a deserialized [Package].
            #[derive(Copy, Clone)]
            pub struct PackageRef<'a>(#[allow(dead_code)] ::planus::table_reader::Table<'a>);

            impl<'a> PackageRef<'a> {
                /// Getter for the [`min_runtime_version` field](Package#structfield.min_runtime_version).
                #[inline]
                pub fn min_runtime_version(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "Package", "min_runtime_version")?
                            .unwrap_or(0),
                    )
                }

                /// Getter for the [`serialized_multi_executable` field](Package#structfield.serialized_multi_executable).
                #[inline]
                pub fn serialized_multi_executable(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(1, "Package", "serialized_multi_executable")
                }

                /// Getter for the [`signature` field](Package#structfield.signature).
                #[inline]
                pub fn signature(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(2, "Package", "signature")
                }

                /// Getter for the [`keypair_version` field](Package#structfield.keypair_version).
                #[inline]
                pub fn keypair_version(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0.access(3, "Package", "keypair_version")?.unwrap_or(0),
                    )
                }

                /// Getter for the [`compiler_version` field](Package#structfield.compiler_version).
                #[inline]
                pub fn compiler_version(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(4, "Package", "compiler_version")
                }

                /// Getter for the [`virtual_chip_id` field](Package#structfield.virtual_chip_id).
                #[inline]
                pub fn virtual_chip_id(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0.access(5, "Package", "virtual_chip_id")?.unwrap_or(0),
                    )
                }

                /// Getter for the [`multi_chip_package` field](Package#structfield.multi_chip_package).
                #[inline]
                pub fn multi_chip_package(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::SerializedPackageRef<'a>>>,
                    >,
                > {
                    self.0.access(6, "Package", "multi_chip_package")
                }

                /// Getter for the [`model_identifier` field](Package#structfield.model_identifier).
                #[inline]
                pub fn model_identifier(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(7, "Package", "model_identifier")
                }
            }

            impl<'a> ::core::fmt::Debug for PackageRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("PackageRef");
                    f.field("min_runtime_version", &self.min_runtime_version());
                    if let ::core::option::Option::Some(field_serialized_multi_executable) =
                        self.serialized_multi_executable().transpose()
                    {
                        f.field(
                            "serialized_multi_executable",
                            &field_serialized_multi_executable,
                        );
                    }
                    if let ::core::option::Option::Some(field_signature) =
                        self.signature().transpose()
                    {
                        f.field("signature", &field_signature);
                    }
                    f.field("keypair_version", &self.keypair_version());
                    if let ::core::option::Option::Some(field_compiler_version) =
                        self.compiler_version().transpose()
                    {
                        f.field("compiler_version", &field_compiler_version);
                    }
                    f.field("virtual_chip_id", &self.virtual_chip_id());
                    if let ::core::option::Option::Some(field_multi_chip_package) =
                        self.multi_chip_package().transpose()
                    {
                        f.field("multi_chip_package", &field_multi_chip_package);
                    }
                    if let ::core::option::Option::Some(field_model_identifier) =
                        self.model_identifier().transpose()
                    {
                        f.field("model_identifier", &field_model_identifier);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<PackageRef<'a>> for Package {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: PackageRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        min_runtime_version: ::core::convert::TryInto::try_into(
                            value.min_runtime_version()?,
                        )?,
                        serialized_multi_executable: value
                            .serialized_multi_executable()?
                            .map(|v| v.to_vec()),
                        signature: value.signature()?.map(|v| v.to_vec()),
                        keypair_version: ::core::convert::TryInto::try_into(
                            value.keypair_version()?,
                        )?,
                        compiler_version: value
                            .compiler_version()?
                            .map(::core::convert::Into::into),
                        virtual_chip_id: ::core::convert::TryInto::try_into(
                            value.virtual_chip_id()?,
                        )?,
                        multi_chip_package: if let ::core::option::Option::Some(
                            multi_chip_package,
                        ) = value.multi_chip_package()?
                        {
                            ::core::option::Option::Some(multi_chip_package.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        model_identifier: value
                            .model_identifier()?
                            .map(::core::convert::Into::into),
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for PackageRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for PackageRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[PackageRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            /// # Safety
            /// The planus compiler generates implementations that initialize
            /// the bytes in `write_values`.
            unsafe impl ::planus::VectorWrite<::planus::Offset<Package>> for Package {
                type Value = ::planus::Offset<Package>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<Package>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for PackageRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[PackageRef]", "read_as_root", 0)
                    })
                }
            }
        }
    }
}
