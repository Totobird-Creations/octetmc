use core::marker::PhantomData;
use bevy_ecs::world::{ World, DeferredWorld };
use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;
use bevy_ecs::component::Tick;
use bevy_ecs::archetype::Archetype;
use bevy_ecs::system::{
    SystemParam,
    ReadOnlySystemParam,
    SystemMeta,
    SystemParamValidationError,
    ParallelCommands,
    ResMut
};
use bevy_ecs::event::{ Event, Events };


/// An [`EventWriter`](bevy_ecs::event::EventWriter) which can be used in parallel contexts.
pub struct ParallelEventWriter<'w, 's, E : Event> {
    pcmds   : ParallelCommands<'w, 's>,
    _marker : PhantomData<fn(E) -> ()>
}


unsafe impl<'w, 's, E : Event> SystemParam for ParallelEventWriter<'w, 's, E> {
    type State                = <ParallelCommands::<'w, 's> as SystemParam>::State;
    type Item<'world, 'state> = ParallelEventWriter<'world, 'state, E>;

    #[inline(always)]
    fn init_state(world : &mut World, meta : &mut SystemMeta) -> Self::State {
        <ParallelCommands as SystemParam>::init_state(world, meta)
    }

    #[inline(always)]
    unsafe fn new_archetype(
        state : &mut Self::State,
        arche : &Archetype,
        meta  : &mut SystemMeta,
    ) { unsafe { <ParallelCommands as SystemParam>::new_archetype(state, arche, meta) } }

    #[inline(always)]
    fn apply(state : &mut Self::State, meta : &SystemMeta, world : &mut World) {
        <ParallelCommands as SystemParam>::apply(state, meta, world)
    }

    #[inline(always)]
    fn queue(state : &mut Self::State, meta : &SystemMeta, world : DeferredWorld) {
        <ParallelCommands as SystemParam>::queue(state, meta, world)
    }

    #[inline(always)]
    unsafe fn validate_param(
        state : &Self::State,
        meta  : &SystemMeta,
        world : UnsafeWorldCell
    ) -> Result<(), SystemParamValidationError> {
        unsafe { <ParallelCommands as SystemParam>::validate_param(state, meta, world) }
    }

    #[inline]
    unsafe fn get_param<'world, 'state>(
        state  : &'state mut Self::State,
        meta   : &SystemMeta,
        world  : UnsafeWorldCell<'world>,
        change : Tick,
    ) -> Self::Item<'world, 'state> {
        ParallelEventWriter {
            pcmds   : unsafe { <ParallelCommands as SystemParam>::get_param(state, meta, world, change) },
            _marker : PhantomData
        }
    }
}

unsafe impl<'w, 's, E : Event> ReadOnlySystemParam for ParallelEventWriter<'w, 's, E>
where
    ResMut<'w, Events<E>> : ReadOnlySystemParam
{ }


impl<'w, 's, E : Event> ParallelEventWriter<'w, 's, E> {

    /// Analogous to [`EventWriter::write`](bevy_ecs::event::EventWriter::write).
    #[track_caller]
    pub fn write(&self, event : E) {
        self.pcmds.command_scope(move |mut cmds| {
            cmds.queue(move |world : &mut World| {
                let mut events = world.resource_mut::<Events<E>>();
                events.send(event);
            });
        });
    }

    /// Analogous to [`EventWriter::write_batch`](bevy_ecs::event::EventWriter::write_batch).
    #[track_caller]
    pub fn write_batch(&self, event : impl IntoIterator<Item = E> + Clone + Send + 'static) {
        self.pcmds.command_scope(move |mut cmds| {
            cmds.queue(move |world : &mut World| {
                let mut events = world.resource_mut::<Events<E>>();
                events.send_batch(event);
            });
        });
    }

    /// Analogous to [`EventWriter::write_default`](bevy_ecs::event::EventWriter::write_default).
    #[track_caller]
    pub fn write_default(&self)
    where
        E : Default
    {
        self.pcmds.command_scope(move |mut cmds| {
            cmds.queue(move |world : &mut World| {
                let mut events = world.resource_mut::<Events<E>>();
                events.send_default();
            });
        });
    }

}
