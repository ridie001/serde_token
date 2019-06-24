//! Utility for [transcoding] a [deserializer] into a sink of `Token`s.
//!
//! [transcoding]: https://docs.serde.rs/serde_transcode/index.html
//! [deserializer]: https://docs.serde.rs/serde/trait.Deserializer.html
//!
//! # Example:
//!
//! ```no_run
//! use serde_token::tokenize;
//!
//! let json_str = r#" [1, "hello", 3] "#;
//! let expected = vec![
//!     Token::Seq { len: None },
//!     Token::U64(1),
//!     Token::Str("hello"),
//!     Token::U64(3),
//!     Token::SeqEnd,
//! ];
//!
//! let (token_sink, token_stream) = std::unsync::mpsc::unbounded::<Token>();
//! let mut de = serde_json::de::Deserializer::from_str(json_str);
//!
//! tokenize(&mut de, token_sink).unwrap();
//! let actual = token_stream.collect().wait().unwrap();
//! assert_eq!(expected, actual)
//! ```
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/serde_token/0.0.1")]

mod error;
mod ser;

pub use ser::tokenize;

/// A token corresponding to one of the types defined in the [Serde data model].
///
/// [Serde data model]: https://serde.rs/data-model.html
#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// A serialized `bool`.
    Bool(bool),

    /// A serialized `i8`.
    I8(i8),

    /// A serialized `i16`.
    I16(i16),

    /// A serialized `i32`.
    I32(i32),

    /// A serialized `i64`.
    I64(i64),

    /// A serialized `i128`.
    I128(i128),

    /// A serialized `u8`.
    U8(u8),

    /// A serialized `u16`.
    U16(u16),

    /// A serialized `u32`.
    U32(u32),

    /// A serialized `u64`.
    U64(u64),

    /// A serialized `u128`.
    U128(u128),

    /// A serialized `f32`.
    F32(f32),

    /// A serialized `f64`.
    F64(f64),

    /// A serialized `char`.
    Char(char),

    /// A borrowed `str`.
    Str(&'a str),

    /// A borrowed `[u8]`.
    Bytes(&'a [u8]),

    /// A serialized `Option<T>` containing none.
    None,

    /// The header to a serialized `Option<T>` containing some value.
    Some,

    /// A serialized `()`.
    Unit,

    /// A serialized unit struct of the given name.
    UnitStruct {
        #[doc(hidden)]
        name: &'a str,
    },

    /// A unit variant of an enum.
    UnitVariant {
        #[doc(hidden)]
        name: &'a str,

        #[doc(hidden)]
        variant: &'a str,
    },

    /// The header to a serialized newtype struct of the given name.
    NewtypeStruct {
        #[doc(hidden)]
        name: &'a str,
    },

    /// The header to a newtype variant of an enum.
    NewtypeVariant {
        #[doc(hidden)]
        name: &'a str,

        #[doc(hidden)]
        variant: &'a str,
    },

    /// The header to a sequence.
    Seq {
        #[doc(hidden)]
        len: Option<usize>,
    },

    /// An indicator of the end of a sequence.
    SeqEnd,

    /// The header to a tuple.
    Tuple {
        #[doc(hidden)]
        len: usize,
    },

    /// An indicator of the end of a tuple.
    TupleEnd,

    /// The header to a tuple struct.
    TupleStruct {
        #[doc(hidden)]
        name: &'a str,

        #[doc(hidden)]
        len: usize,
    },

    /// An indicator of the end of a tuple struct.
    TupleStructEnd,

    /// The header to a tuple variant of an enum.
    TupleVariant {
        #[doc(hidden)]
        name: &'a str,

        #[doc(hidden)]
        variant: &'a str,

        #[doc(hidden)]
        len: usize,
    },

    /// An indicator of the end of a tuple variant.
    TupleVariantEnd,

    /// The header to a map.
    Map {
        #[doc(hidden)]
        len: Option<usize>,
    },

    /// An indicator of the end of a map.
    MapEnd,

    /// The header of a struct.
    Struct {
        #[doc(hidden)]
        name: &'a str,

        #[doc(hidden)]
        len: usize,
    },

    /// An indicator of the end of a struct.
    StructEnd,

    /// The header of a struct variant of an enum.
    StructVariant {
        #[doc(hidden)]
        name: &'a str,

        #[doc(hidden)]
        variant: &'a str,

        #[doc(hidden)]
        len: usize,
    },

    /// An indicator of the end of a struct variant.
    StructVariantEnd,

    /// The header to an enum of the given name.
    Enum {
        #[doc(hidden)]
        name: &'a str,
    },
}
