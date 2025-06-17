macro_rules! deref_single { (
    $( #[ $( $attr:tt )+ ] )*
    $vis:vis struct $ident:ident ( $inner:ty $(,)? ) ;
    $( $impls:ident $( { $( $implinner:tt )* } )? ; )*
) => {

    $( $crate::util::macros::macro_deref_single_check_impls!{ $impls $( { $( $implinner )* } )? } )*

    $crate::util::macros::macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
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

    impl $crate::util::CratePrivateNew<$inner> for $ident {
        fn crate_private_new(value : $inner) -> Self {
            $crate::util::macros::macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
                Self { value, dirty : false }
            }, {
                Self { value }
            } }
        }
    }

    $crate::util::macros::macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
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
            $crate::util::macros::macro_deref_single_if_has_dirty!{ { $( $impls , )* }, {
                self.dirty = true;
            }, { } }
            &mut self.value
        }
    }

    $crate::util::macros::macro_deref_single_if_has_from!{ { $( $impls , )* }, {
        impl From<$inner> for $ident {
            fn from(value : $inner) -> Self { $crate::util::CratePrivateNew::crate_private_new(value) }
        }
    }, { } }

    $( $crate::util::macros::macro_deref_single_if_has_default!{ { $impls }, {
        impl Default for $ident {
            fn default() -> Self { $crate::util::CratePrivateNew::crate_private_new({ $( $( $implinner )* )? }) }
        }
    }, { } } )*

} }
pub(crate) use deref_single;


macro_rules! macro_deref_single_check_impls {
    ( Dirtyable ) => { };
    ( From ) => { };
    ( Default { $( $tt:tt )* } ) => { };
}
pub(crate) use macro_deref_single_check_impls;

macro_rules! macro_deref_single_if_has_dirty {
    ( { Dirtyable $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $_:tt )* } $(,)? )    => { $( $true )* };
    ( { $_:ident $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $false:tt )* } $(,)? ) => { $crate::util::macros::macro_deref_single_if_has_dirty!{ { $( $ident , )* }, { $( $true )* }, { $( $false )* }, } };
    ( { $(,)? }, { $( $_:tt )* }, { $( $false:tt )* } $(,)? )                                  => { $( $false )* };
}
pub(crate) use macro_deref_single_if_has_dirty;

macro_rules! macro_deref_single_if_has_from {
    ( { From $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $_:tt )* } $(,)? )         => { $( $true )* };
    ( { $_:ident $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $false:tt )* } $(,)? ) => { $crate::util::macros::macro_deref_single_if_has_from!{ { $( $ident , )* }, { $( $true )* }, { $( $false )* }, } };
    ( { $(,)? }, { $( $_:tt )* }, { $( $false:tt )* } $(,)? )                                  => { $( $false )* };
}
pub(crate) use macro_deref_single_if_has_from;

macro_rules! macro_deref_single_if_has_default {
    ( { Default $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $_:tt )* } $(,)? )      => { $( $true )* };
    ( { $_:ident $( , $ident:ident )* $(,)? }, { $( $true:tt )* }, { $( $false:tt )* } $(,)? ) => { $crate::util::macros::macro_deref_single_if_has_default!{ { $( $ident , )* }, { $( $true )* }, { $( $false )* }, } };
    ( { $(,)? }, { $( $_:tt )* }, { $( $false:tt )* } $(,)? )                                  => { $( $false )* };
}
pub(crate) use macro_deref_single_if_has_default;
