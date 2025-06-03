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
