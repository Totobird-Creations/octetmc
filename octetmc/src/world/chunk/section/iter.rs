use super::{ ChunkSection, min_bits };
use octetmc_protocol::value::block_state::BlockState;
use bitptr::{ BitPtr, BitPtrMut };


impl ChunkSection {

    /// Returns an iterator over the 4096 blocks in this
    #[inline]
    pub fn iter_blocks(&self) -> ChunkSectionIterator {
        if (self.run_bits == 0) {
            // Single-block optimisation.
            ChunkSectionIterator {
                section        : self,
                data_ptr       : bitptr::null(),
                run_len        : 4096,
                run_palette_id : 0,
                consumed       : 0
            }
        } else {
            ChunkSectionIterator {
                section        : self,
                data_ptr       : BitPtr::new_on_byte((unsafe { self.data.assume_init_ref() }).as_ptr()),
                run_len        : 0,
                run_palette_id : 0,
                consumed       : 0
            }
        }
    }

}


/// An `Iterator` over the blocks in a [`ChunkSection`].
pub struct ChunkSectionIterator<'l> {
    section        : &'l ChunkSection,
    data_ptr       : BitPtr,
    run_len        : u16,
    run_palette_id : u8,
    consumed       : u16
}

impl Iterator for ChunkSectionIterator<'_> {
    type Item = BlockState;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.consumed >= 4096) { return None; }
        else { self.consumed += 1; }

        // No more blocks in the current run.
        if (self.run_len == 0) {
            let run_bits        = self.section.run_bits as usize;
            let palette_id_bits = min_bits(self.section.palette.len());

            // Read the next run length.
            let mut next_run_len     = 0u16;
            let     next_run_ignored = (size_of::<u16>() * 8) - run_bits;
            let     next_run_len_ptr = unsafe { BitPtrMut::new_with_offset((&mut next_run_len) as *mut _ as *mut _, next_run_ignored as isize) };
            unsafe { bitptr::copy_nonoverlapping(self.data_ptr, next_run_len_ptr, run_bits); }
            self.data_ptr = unsafe { self.data_ptr.bit_offset(run_bits as isize) };
            let     next_run_len     = u16::from_be(next_run_len);

            // Read the next run palette id.
            let mut next_run_palette_id         = 0u8;
            let     next_run_palette_id_ignored = (size_of::<u8>() * 8) - palette_id_bits;
            let     next_run_palette_id_ptr     = unsafe { BitPtrMut::new_with_offset((&mut next_run_palette_id) as *mut _ as *mut _, next_run_palette_id_ignored as isize) };
            unsafe { bitptr::copy_nonoverlapping(self.data_ptr, next_run_palette_id_ptr, palette_id_bits); }
            self.data_ptr = unsafe { self.data_ptr.bit_offset(palette_id_bits as isize) };
            let     next_run_palette_id     = u8::from_be(next_run_palette_id);

            self.run_len        = next_run_len - 1;
            self.run_palette_id = next_run_palette_id;
        }

        // More blocks in the current run.
        else {
            self.run_len -= 1;
        }

        Some(self.section.palette[self.run_palette_id as usize])
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (4096 - self.consumed) as usize;
        (remaining, Some(remaining),)
    }
}
