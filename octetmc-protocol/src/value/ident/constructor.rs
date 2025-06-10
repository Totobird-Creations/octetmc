use super::{ Ident, RawIdent, IdentValidateError, IdentPathValidateError };
use std::borrow::Cow;


impl<'l> Ident<'l> {

    /// The vanilla namespace, `minecraft`.
    pub const VANILLA_NAMESPACE : &'static str = "minecraft";

}


impl<'l> Ident<'l> {

    #[inline]
    const fn colon_position(s : &str) -> Option<usize> {
        let     s = s.as_bytes();
        let mut i = 0;
        loop {
            if (i > s.len()) { return None; }
            let b = s[i];
            if (b == b':') { return Some(i); }
            i += 1;
        }
    }


    /// Create a new `Ident` from a namespace and path.
    ///
    /// See also: [`Ident::new_str_checked`], [`Ident::new`], and [`Ident::new_checked`].
    ///
    /// ### Panics
    /// Panics if the given namespace or path are invalid.
    #[track_caller]
    pub const fn new_str(nspace : &'l str, path : &'l str) -> Self {
        if let Err(err) = Self::validate(nspace, path) { err.panic("new_str"); }
        unsafe { Self::new_str_unchecked(nspace, path) }
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// See also: [`Ident::new_str`], [`Ident::new`], and [`Ident::new_checked`].
    #[track_caller]
    pub const fn new_str_checked(nspace : &'l str, path : &'l str) -> Result<Self, IdentValidateError> {
        if let Err(err) = Self::validate(nspace, path) { return Err(err); }
        Ok(unsafe { Self::new_str_unchecked(nspace, path) })
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// See also: [`Ident::new_unchecked`].
    ///
    /// ### Safety
    /// The given namespace and path must be valid.
    ///  See [`Ident::validate_nspace`] and [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub const unsafe fn new_str_unchecked(nspace : &'l str, path : &'l str) -> Self {
        Self { raw : RawIdent::Split { nspace : Cow::Borrowed(nspace), path : Cow::Borrowed(path) } }
    }


    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// See also: [`Ident::vanilla_str_checked`], [`Ident::vanilla`], and [`Ident::vanilla_checked`].
    ///
    /// ### Panics
    /// Panics if the given path is invalid.
    #[track_caller]
    pub const fn vanilla_str(path : &'l str) -> Self {
        if let Err(err) = Self::validate_path(path) { err.panic("vanilla_str"); }
        unsafe { Self::vanilla_str_unchecked(path) }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// See also: [`Ident::vanilla_str`], [`Ident::vanilla`], and [`Ident::vanilla_checked`].
    #[track_caller]
    pub const fn vanilla_str_checked(path : &'l str) -> Result<Self, IdentPathValidateError> {
        if let Err(err) = Self::validate_path(path) { return Err(err); }
        Ok(unsafe { Self::vanilla_str_unchecked(path) })
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// See also: [`Ident::vanilla_unchecked`].
    ///
    /// ### Safety
    /// The given path must be valid.
    ///  See [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub const unsafe fn vanilla_str_unchecked(path : &'l str) -> Self {
        Self { raw : RawIdent::Split { nspace : Cow::Borrowed(Self::VANILLA_NAMESPACE), path : Cow::Borrowed(path) } }
    }


    /// Create a new `Ident` from a namespace and path.
    ///
    /// See also: [`Ident::new_checked`], [`Ident::new_str`], and [`Ident::new_str_checked`].
    ///
    /// ### Panics
    /// Panics if the given namespace or path are invalid.
    #[track_caller]
    pub fn new<N, P>(nspace : N, path : P) -> Self
    where
        N : Into<Cow<'l, str>>,
        P : Into<Cow<'l, str>>
    {
        let nspace = nspace.into();
        let path   = path.into();
        if let Err(err) = Self::validate(&nspace, &path) { err.panic("new"); }
        unsafe { Self::new_unchecked(nspace, path) }
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// See also: [`Ident::new`], [`Ident::new_str`], and [`Ident::new_str_checked`].
    #[track_caller]
    pub fn new_checked<N, P>(nspace : N, path : P) -> Result<Self, IdentValidateError>
    where
        N : Into<Cow<'l, str>>,
        P : Into<Cow<'l, str>>
    {
        let nspace = nspace.into();
        let path   = path.into();
        if let Err(err) = Self::validate(&nspace, &path) { return Err(err); }
        Ok(unsafe { Self::new_unchecked(nspace, path) })
    }

    /// Create a new `Ident` from a namespace and path.
    ///
    /// See also: [`Ident::new_str_unchecked`].
    ///
    /// ### Safety
    /// The given namespace and path must be valid.
    ///  See [`Ident::validate_nspace`] and [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub unsafe fn new_unchecked<N, P>(nspace : N, path : P) -> Self
    where
        N : Into<Cow<'l, str>>,
        P : Into<Cow<'l, str>>
    { Self { raw : RawIdent::Split { nspace : nspace.into(), path : path.into() } } }


    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// See also: [`Ident::vanilla_checked`], [`Ident::vanilla_str`], and [`Ident::vanilla_str_checked`].
    ///
    /// ### Panics
    /// Panics if the given path is invalid.
    #[track_caller]
    pub fn vanilla<P>(path : P) -> Self
    where
        P : Into<Cow<'l, str>>
    {
        let path = path.into();
        if let Err(err) = Self::validate_path(&path) { err.panic("vanilla"); }
        unsafe { Self::vanilla_unchecked(path) }
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// See also: [`Ident::vanilla`], [`Ident::vanilla_str`], and [`Ident::vanilla_str_checked`].
    #[track_caller]
    pub fn vanilla_checked<P>(path : P) -> Result<Self, IdentPathValidateError>
    where
        P : Into<Cow<'l, str>>
    {
        let path = path.into();
        if let Err(err) = Self::validate_path(&path) { return Err(err); }
        Ok(unsafe { Self::vanilla_unchecked(path) })
    }

    /// Create a new `Ident` from the vanilla namespace and a path.
    ///
    /// See also: [`Ident::vanilla_str_unchecked`].
    ///
    /// ### Safety
    /// The given path must be valid.
    ///  See [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub unsafe fn vanilla_unchecked<P>(path : P) -> Self
    where
        P : Into<Cow<'l, str>>
    { Self { raw : RawIdent::Split { nspace : Cow::Borrowed(Self::VANILLA_NAMESPACE), path : path.into() } } }


    #[inline(always)]
    #[track_caller]
    const unsafe fn from_joined_str_unchecked_inner(s : &'l str, sep_idx : usize) -> Self {
        Self { raw : RawIdent::Joined { string : Cow::Borrowed(s), sep_idx } }
    }

    /// Create a new `Ident` from a joined `&str`.
    ///
    /// See also: [`Ident::from_joined_unchecked`].
    ///
    /// ### Safety
    /// The given identifier must be valid, and must contain exactly **one** separator (`:`).
    ///  See [`Ident::validate_nspace`] and [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub const unsafe fn from_joined_str_unchecked(s : &'l str) -> Self {
        unsafe { Self::from_joined_str_unchecked_inner(s, Self::colon_position(s).unwrap_unchecked()) }
    }

    /// Create a new `Ident` by parsing a `&str`.
    ///  If there is no separator (`:`), the vanilla namespace is assumed.
    ///
    /// See also: [`Ident::parse_str_checked`], [`Ident::parse`], and [`Ident::parse_checked`].
    ///
    /// ### Panics
    /// Panics if the given string is not a valid identifier.
    #[inline(always)]
    #[track_caller]
    pub const fn parse_str(s : &'l str) -> Self {
        match (Self::colon_position(s)) {
            None => {
                if let Err(err) = Self::validate_path(s) { err.panic("parse_str"); }
                unsafe { Self::vanilla_str_unchecked(s) }
            },
            Some(sep_idx) => {
                let (nspace, other,) = s.split_at(sep_idx);
                let (_, path,) = other.split_at(1);
                if let Err(err) = Self::validate(nspace, path) { err.panic("parse_str"); }
                unsafe { Self::from_joined_str_unchecked_inner(s, sep_idx) }
            }
        }
    }

    /// Create a new `Ident` by parsing a `&str`.
    ///  If there is no separator (`:`), the vanilla namespace is assumed.
    ///
    /// See also: [`Ident::parse_str`], [`Ident::parse`], and [`Ident::parse_checked`].
    #[inline(always)]
    #[track_caller]
    pub const fn parse_str_checked(s : &'l str) -> Result<Self, IdentValidateError> {
        match (Self::colon_position(s)) {
            None => {
                if let Err(err) = Self::validate_path(s) { return Err(err.as_merged()); }
                Ok(unsafe { Self::vanilla_str_unchecked(s) })
            },
            Some(sep_idx) => {
                let (nspace, other,) = s.split_at(sep_idx);
                let (_, path,) = other.split_at(1);
                if let Err(err) = Self::validate(nspace, path) { return Err(err); }
                Ok(unsafe { Self::from_joined_str_unchecked_inner(s, sep_idx) })
            }
        }
    }

    /// Create a new `Ident` by parsing a `&str`.
    ///  If there is no separator (`:`), the vanilla namespace is assumed.
    ///
    /// See also: [`Ident::parse_unchecked`].
    ///
    /// ### Safety
    /// The given identifier must be valid.
    ///  See [`Ident::validate_nspace`] and [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub const unsafe fn parse_str_unchecked(s : &'l str) -> Self {
        match (Self::colon_position(s)) {
            None          => unsafe { Self::vanilla_str_unchecked(s) },
            Some(sep_idx) => unsafe { Self::from_joined_str_unchecked_inner(s, sep_idx) }
        }
    }


    #[inline(always)]
    #[track_caller]
    unsafe fn from_joined_unchecked_inner<S>(s : S, sep_idx : usize) -> Self
    where
        S : Into<Cow<'l, str>>
    { Self { raw : RawIdent::Joined { string : s.into(), sep_idx } } }

    /// Create a new `Ident` from a joined `str`-like.
    ///
    /// See also: [`Ident::from_joined_str_unchecked`].
    ///
    /// ### Safety
    /// The given identifier must be valid, and must contain exactly **one** separator (`:`).
    ///  See [`Ident::validate_nspace`] and [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub unsafe fn from_joined_unchecked<S>(s : S) -> Self
    where
        S : Into<Cow<'l, str>>
    {
        let s       = s.into();
        let sep_idx = Self::colon_position(&s);
        unsafe { Self::from_joined_unchecked_inner(s, sep_idx.unwrap_unchecked()) }
    }

    /// Create a new `Ident` by parsing a `str`-like.
    ///  If there is no separator (`:`), the vanilla namespace is assumed.
    ///
    /// See also: [`Ident::parse_checked`], [`Ident::parse_str`], and [`Ident::parse_str_checked`].
    ///
    /// ### Panics
    /// Panics if the given string is not a valid identifier.
    #[inline(always)]
    #[track_caller]
    pub fn parse<S>(s : S) -> Self
    where
        S : Into<Cow<'l, str>>
    {
        let s = s.into();
        match (Self::colon_position(&s)) {
            None => {
                if let Err(err) = Self::validate_path(&s) { err.panic("parse_str"); }
                unsafe { Self::vanilla_unchecked(s) }
            },
            Some(sep_idx) => {
                let (nspace, other,) = s.split_at(sep_idx);
                let (_, path,) = other.split_at(1);
                if let Err(err) = Self::validate(nspace, path) { err.panic("parse_str"); }
                unsafe { Self::from_joined_unchecked_inner(s, sep_idx) }
            }
        }
    }

    /// Create a new `Ident` by parsing a `str`-like.
    ///  If there is no separator (`:`), the vanilla namespace is assumed.
    ///
    /// See also: [`Ident::parse`], [`Ident::parse_str`], and [`Ident::parse_str_checked`].
    #[inline(always)]
    #[track_caller]
    pub fn parse_checked<S>(s : S) -> Result<Self, IdentValidateError>
    where
        S : Into<Cow<'l, str>>
    {
        let s = s.into();
        match (Self::colon_position(&s)) {
            None => {
                if let Err(err) = Self::validate_path(&s) { return Err(err.as_merged()); }
                Ok(unsafe { Self::vanilla_unchecked(s) })
            },
            Some(sep_idx) => {
                let (nspace, other,) = s.split_at(sep_idx);
                let (_, path,) = other.split_at(1);
                if let Err(err) = Self::validate(nspace, path) { return Err(err); }
                Ok(unsafe { Self::from_joined_unchecked_inner(s, sep_idx) })
            }
        }
    }

    /// Create a new `Ident` by parsing a `str`-like.
    ///  If there is no separator (`:`), the vanilla namespace is assumed.
    ///
    /// See also: [`Ident::parse_str_unchecked`].
    ///
    /// ### Safety
    /// The given identifier must be valid.
    ///  See [`Ident::validate_nspace`] and [`Ident::validate_path`].
    #[inline(always)]
    #[track_caller]
    pub unsafe fn parse_unchecked<S>(s : S) -> Self
    where
        S : Into<Cow<'l, str>>
    {
        let s = s.into();
        match (Self::colon_position(&s)) {
            None          => unsafe { Self::vanilla_unchecked(s) },
            Some(sep_idx) => unsafe { Self::from_joined_unchecked_inner(s, sep_idx) }
        }
    }


}
