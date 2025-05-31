pub(crate) trait CratePrivateNew<T> {
    fn crate_private_new(value : T) -> Self;
}


pub(crate) macro deref_single(
    $( #[ $( $attr:tt )+ ] )*
    $vis:vis struct $ident:ident ( $inner:ty $(,)? ) ;
    $( $impls:ident $( { $( $implinner:tt )* } )? ; )*
) {

    $( macro_deref_single_check_impls!{ $impls $( { $( $implinner )* } )? } )*

    macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
        $( #[ $( $attr )+ ] )*
        $vis struct $ident {
            value : $inner,
            dirty : bool
        }
    }, {
        $( #[ $( $attr )+ ] )*
        $vis struct $ident {
            value : $inner
        }
    } }

    impl $crate::util::macros::CratePrivateNew<$inner> for $ident {
        fn crate_private_new(value : $inner) -> Self {
            macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
                Self { value, dirty : false }
            }, {
                Self { value }
            } }
        }
    }

    macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
        impl $crate::util::dirty::Dirtyable for $ident {
            #[inline]
            fn is_dirty(self : &Self) -> bool { self.dirty }
            #[inline]
            fn dirty_mut(self : &mut Self) -> &mut bool { &mut self.dirty }
        }
    }, { } }

    impl core::ops::Deref for $ident {
        type Target = $inner;
        fn deref(&self) -> &Self::Target { &self.value }
    }

    impl core::ops::DerefMut for $ident {
        fn deref_mut(&mut self) -> &mut Self::Target {
            macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
                self.dirty = true;
            }, { } }
            &mut self.value
        }
    }

    macro_deref_single_if_has_from!{ { $( $impls , )* }, {
        impl From<$inner> for $ident {
            fn from(value : $inner) -> Self { Self::crate_private_new(value) }
        }
    }, { } }

}


pub(crate) macro macro_deref_single_check_impls {
    ( Dirtyable ) => { },
    ( From ) => { },
    ( Default { $( $tt:tt )* } ) => { }
}

pub(crate) macro macro_deref_single_if_has_dirty {
    ( { Dirtyable $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $_:tt )* } $(,)? )    => { $( $true )* },
    ( { $_:ident $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $false:tt )* } $(,)? ) => { macro_deref_single_if_has_dirty!{ { $( $ident , )* }, { $( $true )* }, { $( $false )* }, } },
    ( { $(,)? }, { $( $_:tt )* }, { $( $false:tt )* } $(,)? )                                  => { $( $false )* }
}

pub(crate) macro macro_deref_single_if_has_from {
    ( { From $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $_:tt )* } $(,)? )    => { $( $true )* },
    ( { $_:ident $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $false:tt )* } $(,)? ) => { macro_deref_single_if_has_from!{ { $( $ident , )* }, { $( $true )* }, { $( $false )* }, } },
    ( { $(,)? }, { $( $_:tt )* }, { $( $false:tt )* } $(,)? )                                  => { $( $false )* }
}
