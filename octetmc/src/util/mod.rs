//! Utility data structures.


pub mod future;

pub mod dirty;

pub(crate) mod macros;

pub mod ecs;


pub(crate) enum Never { }

pub(crate) trait CratePrivateNew<T> {
    fn crate_private_new(value : T) -> Self;
}
