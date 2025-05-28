pub macro deref_single(
    $( #[ $( $attr:tt )+ ] )*
    $vis:vis struct $ident:ident ( $inner:ty $(,)? ) ;
    $( Default { $( $default:tt )* } )?
) {

    $( #[ $( $attr )+ ] )*
    #[derive(bevy_ecs::resource::Resource)]
    $vis struct $ident {
        value : $inner
    }

    impl core::ops::Deref for $ident {
        type Target = $inner;
        fn deref(&self) -> &Self::Target { &self.value }
    }

    impl core::ops::DerefMut for $ident {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
    }

    $( impl Default for $ident {
        fn default() -> Self { Self { value : { $( $default )* } } }
    } )?

}
