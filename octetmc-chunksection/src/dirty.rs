pub(super) enum ChunkSectionDirty {
    Few([u16; 4]),
    Many
}

impl ChunkSectionDirty {

    pub(super) fn add(&mut self, i : u16) {
        if let Self::Few(few) = self {
            for slot in few {
                if (*slot == i) {
                    return;
                } else if (*slot == u16::MAX) {
                    *slot = i;
                    return;
                }
            }
            *self = Self::Many;
        }
    }

}

impl ChunkSectionDirty {
    pub(super) const NONE : Self = Self::Few([0; 4]);
}
