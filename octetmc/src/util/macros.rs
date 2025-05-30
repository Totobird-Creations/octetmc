pub(crate) macro deref_single(
    $( #[ $( $attr:tt )+ ] )*
    $vis:vis struct $ident:ident ( $inner:ty $(,)? ) ;
    $( Default { $( $default:tt )* } )?
) {

    $( #[ $( $attr )+ ] )*
    #[derive(bevy_ecs::resource::Resource)]
    $vis struct $ident {
        value : $inner,
        dirty : bool
    }

    impl $crate::util::dirty::Dirtyable for $ident {
        #[inline]
        fn is_dirty(self : &Self) -> bool { self.dirty }
        #[inline]
        fn dirty_mut(self : &mut Self) -> &mut bool { &mut self.dirty }

    }

    impl core::ops::Deref for $ident {
        type Target = $inner;
        fn deref(&self) -> &Self::Target { &self.value }
    }

    impl core::ops::DerefMut for $ident {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.dirty = true;
            &mut self.value
        }
    }

    $( impl Default for $ident {
        fn default() -> Self { Self { value : { $( $default )* }, dirty : false } }
    } )?

}
