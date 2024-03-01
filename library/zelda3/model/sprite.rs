use std::fmt::Display;

use super::palette_index::PaletteIndex;
use super::sprite_id::SpriteId;

#[derive(Clone, Debug)]
pub(crate) struct Sprite {
    /// Entity Id
    pub id: SpriteId,
    /// Thrown sprites ignore pits and crash on pits. Non-thrown sprites fall
    pub allow_pits: bool,
    /// Enable large shadow
    pub big_shadow: bool,
    /// True if blockable by shield
    pub blockable: bool,
    /// True if boss sfx should be used
    pub boss_damage_sfx: bool,
    /// Plays a boss death animation on death
    pub boss_death_animation: bool,
    /// True if the creature is not deleted as part of boss prep
    pub boss_prep_preserved: bool,
    /// Only process collisions on a single layer (canonballs, etc.)
    pub collision_on_single_layer: bool,
    /// Reduces hitbox collisions, etc
    pub collisions_alt: bool,
    /// Has custom logic on death. Octoballoon for example
    pub custom_death_animation: bool,
    /// True if arrows should bounce off the Sprite
    pub deflect_arrows: bool,
    pub die_off_screen: bool,
    /// oam allocation
    pub display_allocation: u8,
    /// Has a visible shadow
    pub draw_shadow: bool,
    /// Considered harmless (changes collisions)
    pub harmless: bool,
    /// Mostly determines what bees should target
    pub high_priority: bool,
    /// Size of the hitbox
    pub hitbox: u8,
    /// Amount of health it has
    pub hp: u8,
    /// True if pit and conveyer interactions are limited
    pub limit_moving_floor_pit_interaction: bool,
    /// True if collision should be ignored while Link is recoiling
    pub ignore_recoil_collision: bool,
    pub immune_to_sword_hammer: bool,
    pub immune_to_arrows: bool,
    /// True if the creature is immune to Magic Powder
    pub immune_to_powder: bool,
    /// Cannot take any damage
    pub impervious: bool,
    /// Only used by the master sword animation
    pub master_sword_only: bool,
    pub moveable_unused: bool,
    /// Copied to OAM
    pub name_table: u8,
    /// Controls which palette index is used (changes the hue more or less.)
    /// This is how soldier colors are determined.
    pub palette: PaletteIndex,
    pub prevent_permadeath: bool,
    pub projectile: bool,
    pub projectiles_unused: bool,
    /// Prize pack id
    pub prize_pack: u8,
    /// True if moving off camera should NOT kill the Sprite
    pub preserved_offscreen: bool,
    /// True if special considerations are required for interacting with water
    pub special_water_check: bool,
    /// Special flag for non-harmable sprites who should react to slashing
    pub slashable: bool,
    /// Damage profile (determines damage table)
    pub subclass: u8,
    /// True if the Sprite should be ignored for kill rooms count
    pub statis: bool,
    pub stay_active_offscreen: bool,
    pub tile_hitbox: u8,
}

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Sprite:
  id={} 
  allow_pits={} 
  big_shadow={} 
  blockable={} 
  boss_damage_sfx={} 
  boss_death_animation={} 
  boss_prep_preserved={} 
  collision_on_single_layer={} 
  collisions_alt={} 
  custom_death_animation={} 
  deflect_arrows={} 
  die_off_screen={} 
  display_allocation={} 
  draw_shadow={} 
  harmless={} 
  high_priority={} 
  hitbox={} 
  hp={} 
  ignore_floor={} 
  ignore_recoil_collision={} 
  immune_to_sword_hammer={} 
  immune_to_arrows={} 
  immune_to_powder={} 
  impervious={} 
  master_sword_only={} 
  moveable_unused={} 
  name_table={} 
  palette={} 
  prevent_permadeath={} 
  projectile={} 
  projectiles_unused={} 
  prize_pack={} 
  preserved_offscreen={} 
  special_water_check={} 
  slashable={} 
  subclass={} 
  statis={} 
  stay_active_offscreen={} 
  tile_hitbox={} "#,
            self.id,
            self.allow_pits,
            self.big_shadow,
            self.blockable,
            self.boss_damage_sfx,
            self.boss_death_animation,
            self.boss_prep_preserved,
            self.collision_on_single_layer,
            self.collisions_alt,
            self.custom_death_animation,
            self.deflect_arrows,
            self.die_off_screen,
            self.display_allocation,
            self.draw_shadow,
            self.harmless,
            self.high_priority,
            self.hitbox,
            self.hp,
            self.limit_moving_floor_pit_interaction,
            self.ignore_recoil_collision,
            self.immune_to_sword_hammer,
            self.immune_to_arrows,
            self.immune_to_powder,
            self.impervious,
            self.master_sword_only,
            self.moveable_unused,
            self.name_table,
            self.palette,
            self.prevent_permadeath,
            self.projectile,
            self.projectiles_unused,
            self.prize_pack,
            self.preserved_offscreen,
            self.special_water_check,
            self.slashable,
            self.subclass,
            self.statis,
            self.stay_active_offscreen,
            self.tile_hitbox,
        )
    }
}
