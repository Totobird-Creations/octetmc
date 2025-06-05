use super::{ NbtCompound, NbtTag };
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };
use std::borrow::Cow;
use cesu8;


/// An NBT element.
#[derive(Clone, Debug)]
pub enum NbtElement<'l> {

    /// A signed 8-bit integer element.
    Byte(i8),

    /// A signed 16-bit integer element.
    Short(i16),

    /// A signed 32-bit integer element.
    Int(i32),

    /// A signed 64-bit integer element.
    Long(i64),

    /// A 32-bit float element.
    Float(f32),

    /// A 64-bit float element.
    Double(f64),

    /// A string element.
    String(Cow<'l, str>),

    /// A signed 8-bit integer array element.
    BArray(Cow<'l, [i8]>),

    /// A signed 32-bit integer array element.
    IArray(Cow<'l, [i32]>),

    /// A signed 64-bit integer array element.
    LArray(Cow<'l, [i64]>),

    /// A list element.
    ///
    /// All elements in a list must be of the same type.
    List(Cow<'l, [NbtElement<'l>]>),

    /// A compound/map element.
    Compound(NbtCompound<'l>)

}


impl NbtElement<'_> {

    /// Convert the inner parts of this `NbtElement` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `NbtElement<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> NbtElement<'static> { match (self) {
        Self::Byte     (v) => NbtElement::Byte     (v),
        Self::Short    (v) => NbtElement::Short    (v),
        Self::Int      (v) => NbtElement::Int      (v),
        Self::Long     (v) => NbtElement::Long     (v),
        Self::Float    (v) => NbtElement::Float    (v),
        Self::Double   (v) => NbtElement::Double   (v),
        Self::String   (v) => NbtElement::String   (Cow::Owned(v.into_owned())),
        Self::BArray   (v) => NbtElement::BArray   (Cow::Owned(match (v) {
            Cow::Borrowed(u) => u.iter().map(|t| *t).collect(),
            Cow::Owned(u)    => u,
        })),
        Self::IArray   (v) => NbtElement::IArray   (Cow::Owned(match (v) {
            Cow::Borrowed(u) => u.iter().map(|t| *t).collect(),
            Cow::Owned(u)    => u,
        })),
        Self::LArray   (v) => NbtElement::LArray   (Cow::Owned(match (v) {
            Cow::Borrowed(u) => u.iter().map(|t| *t).collect(),
            Cow::Owned(u)    => u,
        })),
        Self::List     (v) => NbtElement::List     (Cow::Owned(match (v) {
            Cow::Borrowed(u) => u.iter().map(|t| t.to_static_owned()).collect(),
            Cow::Owned(u)    => u.into_iter().map(|t| t.into_static_owned()).collect(),
        })),
        Self::Compound (v) => NbtElement::Compound (v.into_static_owned())
    } }

    /// Convert the inner parts of this `NbtElement` to their owned counterparts.
    ///  Returns the newly created `NbtElement<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> NbtElement<'static> { match (self) {
        Self::Byte     (v) => NbtElement::Byte     (*v),
        Self::Short    (v) => NbtElement::Short    (*v),
        Self::Int      (v) => NbtElement::Int      (*v),
        Self::Long     (v) => NbtElement::Long     (*v),
        Self::Float    (v) => NbtElement::Float    (*v),
        Self::Double   (v) => NbtElement::Double   (*v),
        Self::String   (v) => NbtElement::String   (Cow::Owned((**v).to_owned())),
        Self::BArray   (v) => NbtElement::BArray   (Cow::Owned(v.iter().map(|t| *t).collect())),
        Self::IArray   (v) => NbtElement::IArray   (Cow::Owned(v.iter().map(|t| *t).collect())),
        Self::LArray   (v) => NbtElement::LArray   (Cow::Owned(v.iter().map(|t| *t).collect())),
        Self::List     (v) => NbtElement::List     (Cow::Owned(v.iter().map(|t| t.to_static_owned()).collect())),
        Self::Compound (v) => NbtElement::Compound (v.to_static_owned())
    } }

}


impl NbtElement<'_> {

    /// Returns the tag of this element.
    #[inline]
    pub fn tag(&self) -> NbtTag { match (self) {
        NbtElement::Byte     (_) => NbtTag::Byte,
        NbtElement::Short    (_) => NbtTag::Short,
        NbtElement::Int      (_) => NbtTag::Int,
        NbtElement::Long     (_) => NbtTag::Long,
        NbtElement::Float    (_) => NbtTag::Float,
        NbtElement::Double   (_) => NbtTag::Double,
        NbtElement::String   (_) => NbtTag::String,
        NbtElement::BArray   (_) => NbtTag::BArray,
        NbtElement::IArray   (_) => NbtTag::IArray,
        NbtElement::LArray   (_) => NbtTag::LArray,
        NbtElement::List     (_) => NbtTag::List,
        NbtElement::Compound (_) => NbtTag::Compound
    } }

}


impl PacketPartEncode for NbtElement<'_> {

    fn predict_size(&self) -> usize { match (self) {
        Self::Byte     (v) => v.predict_size(),
        Self::Short    (v) => v.predict_size(),
        Self::Int      (v) => v.predict_size(),
        Self::Long     (v) => v.predict_size(),
        Self::Float    (v) => v.predict_size(),
        Self::Double   (v) => v.predict_size(),
        Self::String   (v) => size_of::<u16>() + cesu8::to_java_cesu8(v).len(),
        Self::BArray   (v) => size_of::<u32>() + v.len(),
        Self::IArray   (v) => size_of::<u32>() + (size_of::<i32>() * v.len()),
        Self::LArray   (v) => size_of::<u32>() + (size_of::<i64>() * v.len()),
        Self::List     (v) => 1 + size_of::<u32>() + v.iter().map(|u| u.predict_size()).sum::<usize>(),
        Self::Compound (v) => v.predict_size(),
    } }

    fn encode(&self, buf : &mut EncodeBuf) { match (self) {
        Self::Byte     (v) => buf.write(v),
        Self::Short    (v) => buf.encode_write(v),
        Self::Int      (v) => buf.encode_write(v),
        Self::Long     (v) => buf.encode_write(v),
        Self::Float    (v) => buf.encode_write(v),
        Self::Double   (v) => buf.encode_write(v),
        Self::String   (v) => {
            let jv = cesu8::to_java_cesu8(v);
            buf.encode_write(jv.len() as u16);
            buf.write_n(&jv);
        },
        Self::BArray   (v) => {
            buf.encode_write(v.len() as u32);
            buf.write_iter(v.iter().map(|u| *u as u8));
        },
        Self::IArray   (v) => {
            buf.encode_write(v.len() as u32);
            for u in &**v {
                buf.encode_write(u);
            }
        },
        Self::LArray   (v) => {
            buf.encode_write(v.len() as u32);
            for u in &**v {
                buf.encode_write(u);
            }
        },
        Self::List     (v) => {
            buf.write(v.first().map_or(NbtTag::End, |u| u.tag()) as u8);
            buf.encode_write(v.len() as u32);
            for u in &**v {
                buf.encode_write(u);
            }
        },
        Self::Compound (v) => buf.encode_write(v)
    } }

}
