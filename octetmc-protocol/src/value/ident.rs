//! Resource location identifiers.


use crate::value::varint::VarIntDecodeError;
use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use crate::packet::decode::str::StringDecodeError;
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };
use core::{ str, fmt };
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


const VANILLA_NAMESPACE : &str = "minecraft";


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Identifier>
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ident<'l> {
    nspace : Cow<'l, str>,
    path   : Cow<'l, str>
}

impl<'l> Ident<'l> {

    /// Create a new `Ident` from a namespace and path.
    ///
    /// ### Panics
    /// Panics if the given namespace or path are invalid.
    #[inline]
    pub const fn new(nspace : &'l str, path : &'l str) -> Self {
        if (Self::check_nspace(nspace).is_err()) { panic!("called `Ident::new` with invalid namespace") }
        if (Self::check_path(path).is_err()) { panic!("called `Ident::new` with invalid path") }
        unsafe { Self::new_unchecked(nspace, path) }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// ### Panics
    /// Panics if the given path is invalid.
    #[inline]
    pub const fn new_vanilla(path : &'l str) -> Self {
        if (Self::check_path(path).is_err()) { panic!("called `Ident::new` with invalid path") }
        unsafe { Self::new_unchecked(VANILLA_NAMESPACE, path) }
    }

    /// Create a new `Ident` from a namespace and path.
    #[inline]
    pub const fn new_checked(nspace : &'l str, path : &'l str) -> Result<Self, IdentDecodeError> {
        if let Err(err) = Self::check_nspace(nspace) { return Err(err); }
        if let Err(err) = Self::check_path(path) { return Err(err); }
        Ok(unsafe { Self::new_unchecked(nspace, path) })
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    #[inline]
    pub const fn new_vanilla_checked(path : &'l str) -> Result<Self, IdentDecodeError> {
        if let Err(err) = Self::check_path(path) { return Err(err); }
        Ok(unsafe { Self::new_unchecked(VANILLA_NAMESPACE, path) })
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// ### Safety
    /// The given namespace and path must be valid. See [`Self::check_nspace`] and [`Self::check_path`].
    #[inline(always)]
    pub const unsafe fn new_unchecked(nspace : &'l str, path : &'l str) -> Self {
        Self { nspace : Cow::Borrowed(nspace), path : Cow::Borrowed(path) }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// ### Safety
    /// The given path must be valid. See [`Self::check_path`].
    #[inline(always)]
    pub const unsafe fn new_vanilla_unchecked(path : &'l str) -> Self {
        Self { nspace : Cow::Borrowed(VANILLA_NAMESPACE), path : Cow::Borrowed(path) }
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// ### Panics
    /// Panics if the given namespace or path are invalid.
    #[inline]
    pub fn new_cow(nspace : Cow<'l, str>, path : Cow<'l, str>) -> Self {
        if (Self::check_nspace(&nspace).is_err()) { panic!("called `Ident::new_cow` with invalid namespace") }
        if (Self::check_path(&path).is_err()) { panic!("called `Ident::new_cow` with invalid path") }
        unsafe { Self::new_cow_unchecked(nspace, path) }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// ### Panics
    /// Panics if the given path is invalid.
    #[inline]
    pub fn new_vanilla_cow(path : Cow<'l, str>) -> Self {
        if (Self::check_path(&path).is_err()) { panic!("called `Ident::new_cow` with invalid path") }
        unsafe { Self::new_cow_unchecked(Cow::Borrowed(VANILLA_NAMESPACE), path) }
    }

    /// Create a new `Ident` from a namespace and path.
    #[inline]
    pub fn new_cow_checked(nspace : Cow<'l, str>, path : Cow<'l, str>) -> Result<Self, IdentDecodeError> {
        Self::check_nspace(&nspace)?;
        Self::check_path(&path)?;
        Ok(unsafe { Self::new_cow_unchecked(nspace, path) })
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    #[inline]
    pub fn new_vanilla_cow_checked(path : Cow<'l, str>) -> Result<Self, IdentDecodeError> {
        Self::check_path(&path)?;
        Ok(unsafe { Self::new_cow_unchecked(Cow::Borrowed(VANILLA_NAMESPACE), path) })
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// ### Safety
    /// The given namespace and path must be valid. See [`Self::check_nspace`] and [`Self::check_path`].
    #[inline(always)]
    pub const unsafe fn new_cow_unchecked(nspace : Cow<'l, str>, path : Cow<'l, str>) -> Self {
        Self { nspace, path }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// ### Safety
    /// The given path must be valid. See [`Self::check_path`].
    #[inline(always)]
    pub const unsafe fn new_vanilla_cow_unchecked(path : Cow<'l, str>) -> Self {
        Self { nspace : Cow::Borrowed(VANILLA_NAMESPACE), path }
    }

    /// Create a new `Ident` by parsing a `&str`.
    ///
    /// ### Panics
    /// Panics if the given string is not a valid identifier.
    pub const fn parse(s : &'l str) -> Self {
        let s1 = s.as_bytes();
        let mut i = 0;
        let split_at = loop {
            if (i > s1.len()) { panic!("called `Ident::parse` with missing separator"); }
            let ch = s1[i] as char;
            if (ch == ':') { break i; }
            i += 1;
        };
        let (nspace, colon_and_path,) = s.split_at(split_at);
        let (_, path,) = colon_and_path.split_at(1);
        Self::new(nspace, path)
    }

    /// Create a new `Ident` by parsing a `&str`.
    pub const fn parse_checked(s : &'l str) -> Result<Self, IdentDecodeError> {
        let s1 = s.as_bytes();
        let mut i = 0;
        let split_at = loop {
            if (i > s1.len()) { return Err(IdentDecodeError::NoSeparator); }
            let ch = s1[i] as char;
            if (ch == ':') { break i; }
            i += 1;
        };
        let (nspace, colon_and_path,) = s.split_at(split_at);
        let (_, path,) = colon_and_path.split_at(1);
        Self::new_checked(nspace, path)
    }

    /// Create a new `Ident` by parsing a `&str`.
    ///
    /// ### Safety
    /// The given string and path must be a valid identifier with a namespace, one `:`, and a path.
    ///  See [`Self::check_nspace`] and [`Self::check_path`].
    ///
    /// This function may loop forever if given an invalid identifier.
    pub const unsafe fn parse_unchecked(s : &'l str) -> Self {
        let s1 = s.as_bytes();
        let mut i = 0;
        let split_at = loop {
            let ch = s1[i] as char;
            if (ch == ':') { break i; }
            i += 1;
        };
        let (nspace, colon_and_path,) = s.split_at(split_at);
        let (_, path,) = colon_and_path.split_at(1);
        unsafe { Self::new_unchecked(nspace, path) }
    }

}

impl Ident<'static> {

    /// Create a new `Ident` from a namespace and path.
    ///
    /// ### Panics
    /// Panics if the given namespace or path are invalid.
    pub fn new_owned(nspace : String, path : String) -> Self {
        if (Self::check_nspace(&nspace).is_err()) { panic!("called `Ident::new_cow` with invalid namespace") }
        if (Self::check_path(&path).is_err()) { panic!("called `Ident::new_cow` with invalid path") }
        unsafe { Self::new_owned_unchecked(nspace, path) }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// ### Panics
    /// Panics if the given path is invalid.
    pub fn new_vanilla_owned(path : String) -> Self {
        if (Self::check_path(&path).is_err()) { panic!("called `Ident::new_cow` with invalid path") }
        unsafe { Self::new_cow_unchecked(Cow::Borrowed(VANILLA_NAMESPACE), Cow::Owned(path)) }
    }

    /// Create a new `Ident` from a namespace and path.
    pub fn new_owned_checked(nspace : String, path : String) -> Result<Self, IdentDecodeError> {
        Self::check_nspace(&nspace)?;
        Self::check_path(&path)?;
        Ok(unsafe { Self::new_owned_unchecked(nspace, path) })
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    pub fn new_vanilla_owned_checked(path : String) -> Result<Self, IdentDecodeError> {
        Self::check_path(&path)?;
        Ok(unsafe { Self::new_cow_unchecked(Cow::Borrowed(VANILLA_NAMESPACE), Cow::Owned(path)) })
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// ### Safety
    /// The given namespace and path must be valid. See [`Self::check_nspace`] and [`Self::check_path`].
    #[inline(always)]
    pub const unsafe fn new_owned_unchecked(nspace : String, path : String) -> Self {
        Self { nspace : Cow::Owned(nspace), path : Cow::Owned(path) }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// ### Safety
    /// The given path must be valid. See [`Self::check_path`].
    #[inline(always)]
    pub const unsafe fn new_vanilla_owned_unchecked(path : String) -> Self {
        Self { nspace : Cow::Borrowed(VANILLA_NAMESPACE), path : Cow::Owned(path) }
    }

}

impl<'l> Ident<'l> {

    /// Convert the inner parts of this `Ident` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `Ident<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> Ident<'static> {
        Ident { nspace : Cow::Owned(self.nspace.into_owned()), path : Cow::Owned(self.path.into_owned()) }
    }

    /// Convert the inner parts of this `Ident` to their owned counterparts.
    ///  Returns the newly created `Ident<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> Ident<'static> {
        Ident { nspace : Cow::Owned((*self.nspace).to_owned()), path : Cow::Owned((*self.path).to_owned()) }
    }

    /// Convert the inner parts of this `Ident` to their borrowed counterparts.
    ///  Returns the newly created `Ident` with the same lifetime.
    #[inline]
    pub fn as_ref(&self) -> Ident<'_> {
        Ident { nspace : Cow::Borrowed(&self.nspace), path : Cow::Borrowed(&self.path) }
    }

    /// Returns the inner parts of this `Ident`.
    #[inline]
    pub fn into_inner(self) -> (Cow<'l, str>, Cow<'l, str>,) { (self.nspace, self.path,) }

    /// Returns a reference to the namespace part of this `Ident`.
    #[inline]
    pub fn nspace(&self) -> &str { &self.nspace }

    /// Returns a reference to the path part of this `Ident`.
    #[inline]
    pub fn path(&self) -> &str { &self.path }

    /// Returns an iterator over the parts of this `Ident`.
    #[inline]
    pub fn path_parts(&self) -> str::Split<'_, char> { self.path.split('/') }

}

impl Ident<'_> {

    /// Validates an identifier namespace.
    ///
    /// Namespaces must follow the pattern `[a-z0-9.-_]+`.
    ///  Namespaces must not be empty.
    pub const fn check_nspace(nspace : &str) -> Result<(), IdentDecodeError> {
        if (nspace.is_empty()) { Err(IdentDecodeError::ZeroLengthNamespace) }
        else {
            let nspace = nspace.as_bytes();
            let mut i = 0;
            while (i < nspace.len()) {
                let ch = nspace[i] as char;
                i += 1;
                if (! Self::is_valid_char(ch)) {
                    return Err(IdentDecodeError::InvalidNamespaceChar(ch));
                }
            }
            Ok(())
        }
    }

    /// Validates an identifier path.
    ///
    /// A path consists of multiple parts separated by `/` characters.
    ///
    /// Path parts must follow the pattern `[a-z0-9.-_]+`.
    ///  Path parts must not be empty.
    pub const fn check_path(path : &str) -> Result<(), IdentDecodeError> {
        let mut part_len = 0;
        let path = path.as_bytes();
        let mut i = 0;
        while (i < path.len()) {
            let ch = path[i] as char;
            i += 1;
            if (ch == '/') {
                if (part_len == 0) {
                    return Err(IdentDecodeError::ZeroLengthPathPart);
                }
            } else if (! Self::is_valid_char(ch)) {
                return Err(IdentDecodeError::InvalidPathChar(ch));
            } else {
                part_len += 1;
            }
        }
        if (part_len == 0) { Err(IdentDecodeError::ZeroLengthPathPart) }
        else { Ok(()) }
    }

    /// Returns `true` if the given `char` is valid in namespaces and path parts.
    ///
    /// Valid characters are `[a-z0-9.-_]`.
    pub const fn is_valid_char(ch : char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9') || ch == '.' || ch == '-' || ch == '_'
    }

}

impl fmt::Display for Ident<'_> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.nspace, self.path)
    }
}

impl Ser for Ident<'_> {

    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    {
        let mut s = String::with_capacity(self.nspace.len() + self.path.len());
        s += &self.nspace;
        s += &self.path;
        s.serialize(serer)
    }

}


impl PacketPartDecode for Ident<'_> {

    type Output<'l> = Ident<'l>;
    type Error<'l>  = IdentDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead) -> Result<Self::Output<'l>, Self::Error<'l>> {
        let s = buf.read_decode::<&str>(head)?;
        Self::Output::parse_checked(s)
    }

}


impl PacketPartEncode for Ident<'_> {

    fn predict_size(&self) -> usize {
        self.nspace.predict_size() + 1 + self.path.predict_size()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&self.nspace);
        buf.encode_write(":");
        buf.encode_write(&self.path);
    }

}


/// An `Ident` failed to decode.
pub enum IdentDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// A string contained invalid UTF-8.
    InvalidUtf8,

    /// The identifier is missing a colon namespace-path separator.
    NoSeparator,

    /// The identifier has a zero=length namespace.
    ZeroLengthNamespace,

    /// The namespace contained invalid characters.
    InvalidNamespaceChar(char),

    /// The path contained invalid characters.
    InvalidPathChar(char),

    /// The path contained a zero-length part.
    ZeroLengthPathPart

}

impl From<IdentDecodeError> for Cow<'static, str> {
    fn from(value : IdentDecodeError) -> Self { match (value) {
        IdentDecodeError::IncompleteData          => IncompleteData.into(),
        IdentDecodeError::VarIntTooLong           => VarIntDecodeError::TooLong.into(),
        IdentDecodeError::InvalidUtf8             => Self::Borrowed("invalid utf8"),
        IdentDecodeError::NoSeparator             => Self::Borrowed("no splitter"),
        IdentDecodeError::ZeroLengthNamespace     => Self::Borrowed("no namspace"),
        IdentDecodeError::InvalidNamespaceChar(_) => Self::Borrowed("invalid namespace character"),
        IdentDecodeError::InvalidPathChar(_)      => Self::Borrowed("invalid path character"),
        IdentDecodeError::ZeroLengthPathPart      => Self::Borrowed("zero length path part")
    } }
}

impl From<IncompleteData> for IdentDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

impl From<StringDecodeError> for IdentDecodeError {
    fn from(value : StringDecodeError) -> Self { match (value) {
        StringDecodeError::IncompleteData => Self::IncompleteData,
        StringDecodeError::VarIntTooLong  => Self::VarIntTooLong,
        StringDecodeError::InvalidUtf8    => Self::InvalidUtf8
    } }
}

impl From<str::Utf8Error> for IdentDecodeError {
    #[inline(always)]
    fn from(_ : str::Utf8Error) -> Self { Self::InvalidUtf8}
}
