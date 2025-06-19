//! Game mode of players.


use crate::mapping_check;


mapping_check!("net.minecraft.world.level.GameType", "005b27e2684e950db0692d6c011f404103c92098ecf7f14824de949685aab867");


/// A player's game mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GameMode {

    /// The survival game mode.
    Survival  = 0,

    /// The creative game mode.
    Creative  = 1,

    /// The adventure game mode.
    ///
    /// Breaking and placing blocks is disabled in adventure mode.
    Adventure = 2,

    /// The spectator game mode.
    ///
    /// All world interactions are disabled in spectator mode.
    ///  Players can noclip and click on entities to view the world
    ///  as the target.
    Spectator = 3

}
