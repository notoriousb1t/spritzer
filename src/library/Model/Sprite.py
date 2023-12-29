from attr import dataclass

from .SpriteId import SpriteId
from .SpriteSubclassId import SpriteSubclassId


@dataclass
class Sprite:
    id: SpriteId
    """Entity Id."""
    hp: int = None
    """Amount of health it has."""
    ignore_recoil_collision: bool = None
    """True if collision should be ignored while Link is recoiling."""
    high_priority: bool = None
    """Mostly determines what bees should target."""
    immune_to_powder: bool = None
    """True if the creature is immune to Magic Powder."""
    boss_prep_preserved: bool = None
    """True if the creature is not deleted as part of boss prep."""
    subclass: SpriteSubclassId = None
    """Damage profile (determines damage table)."""
    display_allocation: int = None
    """oam allocation"""
    collisions_alt: bool = None
    """Reduces hitbox collisions, etc."""
    master_sword_only: bool = None
    """Only used by the master sword animation."""
    harmless: bool = None
    """Considered harmless (changes collisions)."""
    name_table: int = None
    """Copied to OAM"""
    palette: int = None
    """Copied to OAM"""
    impervious: bool = None
    """Cannot take any damage"""
    big_shadow: bool = None
    """Enable large shadow"""
    draw_shadow: bool = None
    """Has a visible shadow"""
    custom_death_animation: bool = None
    """Has custom logic on death. Octoballoon for example."""
    hitbox: int = None
    """Size of the hitbox."""
    preserved_offscreen: bool = None
    """True if moving off camera should NOT kill the Sprite."""
    statis: bool = None
    """True if the Sprite should be ignored for kill rooms count."""
    collision_on_single_layer: bool = None
    """Only process collisions on a single layer (canonballs, etc.)"""
    allow_pits: bool = None
    """Thrown sprites ignore pits and crash on pits. Non-thrown sprites fall."""
    boss_death_animation: bool = None
    """Plays a boss death animation on death."""
    slashable: bool = None
    """Special flag for non-harmable sprites who should react to slashing"""
    deflect_arrows: bool = None
    """True if arrows should bounce off the Sprite."""
    tile_hitbox: int = None

    def set_invisible(self) -> None:
        self.display_allocation = 0

    def set_invincible(self) -> None:
        self.impervious = True
