//! Change detection traits.


use core::mem;
use core::ops::{ Deref, DerefMut };


/// A value which can be flagged as 'dirty', or edited.
///
/// Typically, `Dirtyable`s are marked as dirty on `DerefMut`, even if not changed.
pub trait Dirtyable {

    /// Returns `true` if this `Dirtyable` is dirty.
    fn is_dirty(&self) -> bool;

    /// Returns a mutable reference to the dirty state of this `Dirtyable`.
    fn dirty_mut(&mut self) -> &mut bool;

    /// Returns `true` if this `Dirtyable` is not dirty.
    #[inline(always)]
    fn is_clean(&self) -> bool { ! self.is_dirty() }

    /// Marks this `Dirtyable` as dirty.
    #[inline]
    fn mark_dirty(&mut self) { *self.dirty_mut() = true; }

    /// Marks this `Dirtyable` as not dirty.
    #[inline]
    fn mark_clean(&mut self) { _ = self.take_dirty(); }

    /// Marks this `Dirtyable` as not dirty, returning the previous dirty state.
    #[inline]
    fn take_dirty(&mut self) -> bool { mem::replace(self.dirty_mut(), false) }

}


/// A wrapper around a value which can be flagged as 'dirty', or edited.
///
/// `Dirty`s are marked as dirty on `DerefMut`, even if not changed.
pub struct Dirty<T> {
    inner : T,
    dirty : bool
}

impl<T> Dirtyable for Dirty<T> {
    #[inline]
    fn is_dirty(&self) -> bool { self.dirty }
    #[inline]
    fn dirty_mut(&mut self) -> &mut bool { &mut self.dirty }
}

impl<T> Deref for Dirty<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T> DerefMut for Dirty<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mark_dirty();
        &mut self.inner
    }
}
