//! Change detection traits.


use core::mem;


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
    fn mark_clean(&mut self) { let _ = self.take_dirty(); }

    /// Marks this `Dirtyable` as not dirty, returning the previous dirty state.
    #[inline]
    fn take_dirty(&mut self) -> bool { mem::replace(self.dirty_mut(), false) }

}
