use strum_macros::Display;
use strum_macros::EnumIter;

/// Describes the general archetype of the Sprite. This is only used for logic and is not
/// consistently present anywhere in the game.
#[derive(Clone, Copy, Display, Debug, Eq, PartialEq, Hash, EnumIter, PartialOrd, Ord)]
pub(crate) enum SpriteType {
    /// Unclear or not applicable to randomization
    None,
    /// Boss instances. Some bosses also have enemy spawns as well.
    Boss,
    /// Creatures like Cuccos and rabbits.
    Creature,
    /// Invincible characters such as townfolk.
    Npc,
    /// Most things that harm you.
    Enemy,
    /// Fire bars, spikes, etc.
    Hazard,
    /// Basic category for interactive objects such as projectiles.
    Object,
    /// Consumed by Link or temporary, such as rupees.
    Absorbable,
    /// Screen events like Bee traps.
    Overlord,
}
