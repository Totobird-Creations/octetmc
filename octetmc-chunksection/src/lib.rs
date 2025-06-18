use octetmc_protocol::value::block_state::BlockState;
use octetmc_protocol::registry::block::air::Air;
use core::mem::MaybeUninit;
use std::borrow::Cow;
use bevy_ecs::component::Component;
use bitptr::{ BitPtr, BitPtrMut };


mod dirty;
use dirty::ChunkSectionDirty;

mod iter;
pub use iter::ChunkSectionIterator;

mod edit;
pub use edit::ChunkSectionEdit;


/// A 16x16x16 region in a player's world.
#[derive(Component)]
pub struct ChunkSection {
    palette  : Cow<'static, [BlockState]>,
    run_bits : u8,
    data     : MaybeUninit<Box<[u8]>>,
    dirty    : ChunkSectionDirty
}


impl ChunkSection {

    /// A `ChunkSection` entirely filled with [`Air`].
    pub const AIR : Self = Self {
        palette  : Cow::Borrowed(&[Air.to_block_state()]),
        run_bits : 0,
        data     : MaybeUninit::uninit(),
        dirty    : ChunkSectionDirty::NONE
    };

    /// Creates a new `ChunkSection` completely filled with `block`.
    pub fn filled<B>(block : B) -> Self
    where
        B : Into<BlockState>
    { Self {
        palette  : Cow::Owned(vec![ block.into() ]),
        run_bits : 0,
        data     : MaybeUninit::uninit(),
        dirty    : ChunkSectionDirty::NONE
    } }

}


impl ChunkSection {

    /// Convert this `ChunkSection` to a linear block array.
    pub fn into_array(&self) -> [BlockState; 4096] {
        let mut arr = [Air.to_block_state(); 4096];
        for (i, block,) in self.iter_blocks().enumerate() {
            *(unsafe { arr.get_unchecked_mut(i) }) = block;
        }
        arr
    }

    #[inline]
    pub fn dirty(&mut self, i : u16) {
        self.dirty.add(i);
    }

    #[inline]
    pub fn dirty_many(&mut self) {
        self.dirty = ChunkSectionDirty::Many;
    }

}


impl From<[BlockState; 4096]> for ChunkSection {
    fn from(blocks : [BlockState; 4096]) -> Self {
        type RunLen = u32;

        // Split by runs.
        let mut runs    = [(0 as RunLen, Air.to_block_state()); 4096];
        let mut run_i   = 0;
        let mut block_i = 0;
        while (block_i < blocks.len()) {
            let     block     = blocks[block_i];
            let mut run       = unsafe { runs.get_unchecked_mut(run_i) };
            let     run_empty = run.0 == 0;
            if (run_empty) {
                run.1 = block;
            } else if (run.1 != block) {
                run_i += 1;
                run   = unsafe { runs.get_unchecked_mut(run_i) };
                run.1 = block;
            }
            run.0   += 1;
            block_i += 1;
        }
        if (unsafe { runs.get_unchecked(run_i) }.0 > 0) {
            run_i += 1;
        }
        let runs = unsafe { runs.get_unchecked(0..run_i) };

        // Single-block optimisation.
        if (runs.len() == 1) {
            let run = unsafe { runs.get_unchecked(0) };
            debug_assert_eq!(run.0, 4096);
            return Self {
                palette  : Cow::Owned(vec![ run.1 ]),
                run_bits : 0,
                data     : MaybeUninit::uninit(),
                dirty    : ChunkSectionDirty::NONE
            };
        }

        let     max_run              = unsafe { runs.iter().map(|(len, _,)| *len).max().unwrap_unchecked() } as usize;
        let     run_bits             = min_bits(max_run);
        debug_assert_ne!(run_bits, 0);
        let     run_ignored_bits     = (size_of::<RunLen>() * 8) - run_bits;
        let mut palette              = runs.iter().map(|(_, block,)| *block).collect::<Vec<_>>();
        palette.sort_by_key(|block| block.id());
        palette.dedup_by_key(|block| block.id());
        let     palette_bits         = min_bits(palette.len());
        let     palette_ignored_bits = (size_of::<usize>() * 8) - palette_bits;

        // Pack into `data` vector.
        let mut data     = Box::<[u8]>::new_uninit_slice(((run_bits + palette_bits) * runs.len()).div_ceil(8));
        let mut data_ptr = BitPtrMut::new_on_byte(data.as_mut_ptr() as *mut _);
        for (run_len, block,) in runs {
            let run_len     = run_len.to_be();
            let run_len_ptr = unsafe { BitPtr::new_with_offset(&run_len as *const _ as *const _, run_ignored_bits as isize) };
            unsafe { bitptr::copy_nonoverlapping(run_len_ptr, data_ptr, run_bits); }
            data_ptr = unsafe { data_ptr.bit_offset(run_bits as isize) };
            let palette_id     = (unsafe { palette.iter().position(|p| p.id() == block.id()).unwrap_unchecked() }).to_be();
            let palette_id_ptr = unsafe { BitPtr::new_with_offset(&palette_id as *const _ as *const _, palette_ignored_bits as isize) };
            unsafe { bitptr::copy_nonoverlapping(palette_id_ptr, data_ptr, palette_bits); }
            data_ptr = unsafe { data_ptr.bit_offset(palette_bits as isize) };
        }

        Self {
            palette  : Cow::Owned(palette),
            run_bits : run_bits as u8,
            data     : MaybeUninit::new(unsafe { data.assume_init() }),
            dirty    : ChunkSectionDirty::NONE
        }
    }
}


impl Drop for ChunkSection {
    #[inline]
    fn drop(&mut self) {
        if (self.run_bits != 0) {
            unsafe { self.data.assume_init_drop(); }
        }
    }
}


#[inline]
const fn min_bits(x : usize) -> usize {
    match (x.checked_ilog2()) {
        Some(s) => (s + 1) as usize,
        None    => 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use octetmc_protocol::registry::block::stone::Stone;
    use octetmc_protocol::registry::block::blue_carpet::BlueCarpet;
    use octetmc_protocol::registry::block::clay::Clay;
    use octetmc_protocol::registry::block::lapis_ore::LapisOre;
    use core::array;


    #[test]
    fn from_array_and_iter() {
        let air         = Air.to_block_state();
        let blue_carpet = BlueCarpet.to_block_state();
        let clay        = Clay.to_block_state();

        let mut blocks = [air; 4096];
        blocks[0..2048].copy_from_slice(&[blue_carpet; 2048]);
        blocks[3072..4096].copy_from_slice(&[clay; 1024]);

        let section = ChunkSection::from(blocks);
        assert_eq!(section.palette.len(), 3);
        assert!(section.palette.contains(&air));
        assert!(section.palette.contains(&blue_carpet));
        assert!(section.palette.contains(&clay));
        assert_eq!(section.run_bits, 12);

        for (i, block,) in section.iter_blocks().enumerate() {
            if (i < 2048) { assert_eq!(block, blue_carpet); }
            else if (i < 3072) { assert_eq!(block, air); }
            else if (i < 4096) { assert_eq!(block, clay); }
            else { panic!("more than 4096 entries in section iterator"); }
        }
    }


    #[test]
    fn single_block_optimisation() {
        let lapis = LapisOre.to_block_state();

        let section = ChunkSection::from([lapis; 4096]);
        assert_eq!(section.palette.len(), 1);
        assert!(section.palette.contains(&lapis));
        assert_eq!(section.run_bits, 0);

        for (i, block,) in section.iter_blocks().enumerate() {
            if (i < 4096) { assert_eq!(block, lapis); }
            else { panic!("more than 4096 entries in section iterator"); }
        }

    }


    #[test]
    fn all_unique_blocks() {

        let section = ChunkSection::from(array::from_fn(|i| BlockState::from_raw_id(i as u32)));
        assert_eq!(section.palette.len(), 4096);
        assert_eq!(section.run_bits, 1);

        for (i, block,) in section.iter_blocks().enumerate() {
            if (i < 4096) { assert_eq!(block, BlockState::from_raw_id(i as u32)); }
            else { panic!("more than 4096 entries in section iterator"); }
        }

    }


    #[test]
    fn long_run() {
        let air   = Air.to_block_state();
        let stone = Stone.to_block_state();

        let section = ChunkSection::from(array::from_fn(|i| {
            if (i < 4095) { stone } else { air }
        }));
        assert_eq!(section.palette.len(), 2);
        assert_eq!(section.run_bits, 12);

        for (i, block,) in section.iter_blocks().enumerate() {
            if (i < 4095) { assert_eq!(block, stone); }
            else if (i == 4095) { assert_eq!(block, air); }
            else { panic!("more than 4096 entries in section iterator"); }
        }

    }


}
