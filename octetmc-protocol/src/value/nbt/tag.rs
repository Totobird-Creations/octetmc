/// NBT element tags.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum NbtTag {

    /// Used to signal the end of a compound element.
    End      = 0,

    /// A signed 8-bit integer element.
    Byte     = 1,

    /// A signed 16-bit integer element.
    Short    = 2,

    /// A signed 32-bit integer element.
    Int      = 3,

    /// A signed 64-bit integer element.
    Long     = 4,

    /// A 32-bit float element.
    Float    = 5,

    /// A 64-bit float element.
    Double   = 6,

    /// A signed 8-bit integer array element.
    BArray   = 7,

    /// A string element.
    String   = 8,

    /// A list element.
    ///
    /// All elements in a list must be of the same type.
    List     = 9,

    /// A compound/map element.
    Compound = 10,

    /// A signed 32-bit integer array element.
    IArray   = 11,

    /// A signed 64-bit integer array element.
    LArray   = 12

}
