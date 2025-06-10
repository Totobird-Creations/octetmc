use super::Ident;
use const_panic::concat_panic as cpanic;


impl Ident<'_> {

    /// Validates an identifier namespace and path.
    ///
    /// Namespaces must follow the pattern `[a-z0-9.-_]+`.
    ///  Namespaces must not be empty.
    ///
    /// A path consists of multiple parts separated by `/` characters.
    ///
    /// Path parts must follow the pattern `[a-z0-9.-_]+`.
    ///  Path parts must not be empty.
    #[inline(always)]
    pub const fn validate(nspace : &str, path : &str) -> Result<(), IdentValidateError> {
        if let Err(err) = Self::validate_nspace(nspace) {
            return Err(err.as_merged());
        }
        if let Err(err) = Self::validate_path(path) {
            return Err(err.as_merged());
        };
        Ok(())
    }

    /// Validates an identifier namespace.
    ///
    /// Namespaces must follow the pattern `[a-z0-9.-_]+`.
    ///  Namespaces must not be empty.
    pub const fn validate_nspace(nspace : &str) -> Result<(), IdentNspaceValidateError> {
        if (nspace.is_empty()) { Err(IdentNspaceValidateError::Empty) }
        else {
            let nspace = nspace.as_bytes();
            let mut i = 0;
            while (i < nspace.len()) {
                let ch = nspace[i] as char;
                i += 1;
                if (! Self::is_valid_char(ch)) {
                    return Err(IdentNspaceValidateError::BadChar(ch));
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
    pub const fn validate_path(path : &str) -> Result<(), IdentPathValidateError> {
        let mut part_len = 0;
        let path = path.as_bytes();
        let mut i = 0;
        while (i < path.len()) {
            let ch = path[i] as char;
            i += 1;
            if (ch == '/') {
                if (part_len == 0) {
                    return Err(IdentPathValidateError::EmptyPart);
                }
            } else if (! Self::is_valid_char(ch)) {
                return Err(IdentPathValidateError::BadChar(ch));
            } else {
                part_len += 1;
            }
        }
        if (part_len == 0) { Err(IdentPathValidateError::EmptyPart) }
        else { Ok(()) }
    }

    /// Returns `true` if the given `char` is valid in namespaces and path parts.
    ///
    /// Valid characters are `[a-z0-9.-_]`.
    #[inline(always)]
    pub const fn is_valid_char(ch : char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9') || ch == '.' || ch == '-' || ch == '_'
    }

}


/// Identifier namespace or path validation failed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IdentValidateError {

    /// The identifier has an empty namespace.
    EmptyNspace,

    /// The identifier namespace contains an invalid character.
    BadNspaceChar(char),

    /// The identifier has an empty path part.
    EmptyPathPart,

    /// The identifier path contains an invalid character.
    BadPathChar(char)

}

impl IdentValidateError {

    #[inline]
    #[track_caller]
    pub(super) const fn panic(&self, call : &'static str) -> &'static str { match (self) {
        Self::EmptyNspace       => IdentNspaceValidateError::Empty.panic(call),
        Self::BadNspaceChar(ch) => IdentNspaceValidateError::BadChar(*ch).panic(call),
        Self::EmptyPathPart     => IdentPathValidateError::EmptyPart.panic(call),
        Self::BadPathChar(ch)   => IdentPathValidateError::BadChar(*ch).panic(call)
    } }

}


/// Identifier namespace validation failed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IdentNspaceValidateError {

    /// The identifier has an empty namespace.
    Empty,

    /// The identifier namespace contains an invalid character.
    BadChar(char)

}

impl IdentNspaceValidateError {

    #[inline]
    #[track_caller]
    const fn panic(&self, call : &'static str) -> &'static str { match (self) {
        Self::Empty       => cpanic!("called `Ident::", call, "` with empty namespace"),
        Self::BadChar(ch) => cpanic!("called `Ident::", call, "` with invalid character `", ch, "` in namespace"),
    } }

    #[inline(always)]
    const fn as_merged(&self) -> IdentValidateError { match (self) {
        Self::Empty       => IdentValidateError::EmptyNspace,
        Self::BadChar(ch) => IdentValidateError::BadNspaceChar(*ch),
    } }

}


/// Identifier path validation failed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IdentPathValidateError {

    /// The identifier path contains an invalid character.
    EmptyPart,

    /// The identifier path contains an invalid character.
    BadChar(char)

}

impl IdentPathValidateError {

    #[inline]
    #[track_caller]
    pub(super) const fn panic(&self, call : &'static str) -> &'static str { match (self) {
        Self::EmptyPart   => cpanic!("called `Ident::", call, "` with empty path part"),
        Self::BadChar(ch) => cpanic!("called `Ident::", call, "` with invalid character `", ch, "` in path part"),
    } }

    #[inline(always)]
    pub(super) const fn as_merged(&self) -> IdentValidateError { match (self) {
        Self::EmptyPart   => IdentValidateError::EmptyPathPart,
        Self::BadChar(ch) => IdentValidateError::BadPathChar(*ch),
    } }

}
