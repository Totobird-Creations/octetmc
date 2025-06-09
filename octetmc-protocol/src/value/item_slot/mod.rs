//! Item slots, items, and related.


use std::borrow::Cow;


mod item;
pub use item::*;

mod component;
pub use component::*;

mod component_type;
pub use component_type::*;


/// An item slot, possibly empty.
#[derive(Debug, Clone)]
pub struct ItemSlot<'l> {

    /// The item count. Every following field is only present if this value is greater than zero.
    pub count             : u32,

    /// The item type.
    pub item              : Item,

    /// Components added.
    pub add_components    : Cow<'l, [ItemComponent<'l>]>,

    /// Default components removed.
    pub remove_components : Cow<'l, [ItemComponentType]>

}


impl ItemSlot<'_> {

    /// Convert the inner parts of this `ItemSlot` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `ItemSlot<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> ItemSlot<'static> {
        ItemSlot {
            count             : self.count,
            item              : self.item,
            add_components    : Cow::Owned(match (self.add_components) {
                Cow::Borrowed(v) => v.iter().map(|v| v.to_static_owned()).collect::<Vec<_>>(),
                Cow::Owned(v)    => v.into_iter().map(|v| v.into_static_owned()).collect::<Vec<_>>(),
            }),
            remove_components : Cow::Owned(match (self.remove_components) {
                Cow::Borrowed(v) => v.iter().map(|v| *v).collect::<Vec<_>>(),
                Cow::Owned(v)    => v,
            })
        }
    }

    /// Convert the inner parts of this `ItemSlot` to their owned counterparts.
    ///  Returns the newly created `ItemSlot<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> ItemSlot<'static> {
        ItemSlot {
            count             : self.count,
            item              : self.item,
            add_components    : Cow::Owned(self.add_components.iter().map(|v| v.to_static_owned()).collect::<Vec<_>>()),
            remove_components : Cow::Owned(self.remove_components.iter().map(|v| *v).collect::<Vec<_>>())
        }
    }

}
