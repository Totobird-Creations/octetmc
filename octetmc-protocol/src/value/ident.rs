use std::borrow::Cow;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ident<'l> {
    nspace : Cow<'l, str>,
    path   : Cow<'l, [Cow<'l, str>]>
}
