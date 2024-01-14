from attr import dataclass

from library.model.sprite_id import SpriteId
from library.model.damage_subclass_id import DamageSubclassId


@dataclass
class Sprite:
    id: SpriteId
    """Entity Id."""

    allow_pits: bool = False
    """Thrown sprites ignore pits and crash on pits. Non-thrown sprites fall."""
    big_shadow: bool = False
    """Enable large shadow"""
    blockable: bool = False
    """True if blockable by shield"""
    boss_damage_sfx: bool = False
    """True if boss sfx should be used"""
    boss_death_animation: bool = False
    """Plays a boss death animation on death."""
    boss_prep_preserved: bool = False
    """True if the creature is not deleted as part of boss prep."""
    collision_on_single_layer: bool = False
    """Only process collisions on a single layer (canonballs, etc.)"""
    collisions_alt: bool = False
    """Reduces hitbox collisions, etc."""
    custom_death_animation: bool = False
    """Has custom logic on death. Octoballoon for example."""
    deflect_arrows: bool = False
    """True if arrows should bounce off the Sprite."""
    die_off_screen: bool = False
    display_allocation: int = 0
    """oam allocation"""
    draw_shadow: bool = False
    """Has a visible shadow"""
    harmless: bool = False
    """Considered harmless (changes collisions)."""
    high_priority: bool = False
    """Mostly determines what bees should target."""
    hitbox: int = 0
    """Size of the hitbox."""
    hp: int = 0
    """Amount of health it has."""
    ignore_floor: bool = False
    """True if pit and conveyer interactions are limited"""
    ignore_recoil_collision: bool = False
    """True if collision should be ignored while Link is recoiling."""
    immune_to_sword_hammer: bool = False
    immune_to_arrows: bool = False
    immune_to_powder: bool = False
    """True if the creature is immune to Magic Powder."""
    impervious: bool = False
    """Cannot take any damage"""
    master_sword_only: bool = False
    """Only used by the master sword animation."""
    moveable_unused: bool = False
    name_table: int = 0
    """Copied to OAM"""
    palette: int = 0
    """Copied to OAM"""
    prevent_permadeath: bool = False
    projectile: bool = False
    projectiles_unused: bool = False
    prize_pack: int = 0
    """Prize pack id"""
    preserved_offscreen: bool = False
    """True if moving off camera should NOT kill the Sprite."""
    special_water_check: bool = False
    """True if special considerations are required for interacting with water."""
    slashable: bool = False
    """Special flag for non-harmable sprites who should react to slashing"""
    subclass: DamageSubclassId = DamageSubclassId.x0_NO_DAMAGE
    """Damage profile (determines damage table)."""
    statis: bool = False
    """True if the Sprite should be ignored for kill rooms count."""
    stay_active_offscreen: bool = False
    tile_hitbox: int = 0
