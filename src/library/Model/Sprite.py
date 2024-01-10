from attr import dataclass

from library.Model.SpriteId import SpriteId
from library.Model.SpriteSubclassId import SpriteSubclassId


@dataclass
class Sprite:
    id: SpriteId
    """Entity Id."""

    allow_pits: bool = None
    """Thrown sprites ignore pits and crash on pits. Non-thrown sprites fall."""
    big_shadow: bool = None
    """Enable large shadow"""
    blockable: bool = None
    """True if blockable by shield"""
    boss_damage_sfx: bool = None
    """True if boss sfx should be used"""
    boss_death_animation: bool = None
    """Plays a boss death animation on death."""
    boss_prep_preserved: bool = None
    """True if the creature is not deleted as part of boss prep."""
    collision_on_single_layer: bool = None
    """Only process collisions on a single layer (canonballs, etc.)"""
    collisions_alt: bool = None
    """Reduces hitbox collisions, etc."""
    custom_death_animation: bool = None
    """Has custom logic on death. Octoballoon for example."""
    deflect_arrows: bool = None
    """True if arrows should bounce off the Sprite."""
    die_off_screen: bool = None
    display_allocation: int = None
    """oam allocation"""
    draw_shadow: bool = None
    """Has a visible shadow"""
    harmless: bool = None
    """Considered harmless (changes collisions)."""
    high_priority: bool = None
    """Mostly determines what bees should target."""
    hitbox: int = None
    """Size of the hitbox."""
    hp: int = None
    """Amount of health it has."""
    ignore_floor: bool = None
    """True if pit and conveyer interactions are limited"""
    ignore_recoil_collision: bool = None
    """True if collision should be ignored while Link is recoiling."""
    immune_to_sword_hammer: bool = None
    immune_to_arrows: bool = None
    immune_to_powder: bool = None
    """True if the creature is immune to Magic Powder."""
    impervious: bool = None
    """Cannot take any damage"""
    master_sword_only: bool = None
    """Only used by the master sword animation."""
    moveable_unused: bool = None
    name_table: int = None
    """Copied to OAM"""
    palette: int = None
    """Copied to OAM"""
    prevent_permadeath: bool = None
    projectile: bool = None
    projectiles_unused: bool = None
    prize_pack: int = None
    """Prize pack id"""
    preserved_offscreen: bool = None
    """True if moving off camera should NOT kill the Sprite."""
    special_water_check: bool = None
    """True if special considerations are required for interacting with water."""
    slashable: bool = None
    """Special flag for non-harmable sprites who should react to slashing"""
    subclass: SpriteSubclassId = None
    """Damage profile (determines damage table)."""
    statis: bool = None
    """True if the Sprite should be ignored for kill rooms count."""
    stay_active_offscreen: bool = None
    tile_hitbox: int = None
