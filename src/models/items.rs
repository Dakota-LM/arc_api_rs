use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use super::common::{DateTimeString, UriString};


/// An item returned by the MetaForge ARC Raiders API.
///
/// Source sample provided by you:
/// - /items?page=1&limit=2 returns a raw JSON array of these items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,

    pub item_type: String,
    pub loadout_slots: Vec<String>,

    pub icon: UriString,
    pub rarity: String,
    pub value: i32,

    pub workbench: Option<String>,

    pub stat_block: StatBlock,

    pub flavor_text: Option<String>,
    pub subcategory: Option<String>,

    pub created_at: DateTimeString,
    pub updated_at: DateTimeString,

    pub shield_type: Option<String>,
    pub loot_area: Option<String>,

    /// Nullable in your sample.
    pub sources: Option<Value>,

    pub ammo_type: Option<String>,

    /// Present as arrays (empty in your sample). Element shape unknown so far.
    pub locations: Vec<Value>,
    pub guide_links: Vec<Value>,

    /// Long-form article body. Nullable; concrete shape not yet observed.
    #[serde(default)]
    pub article: Option<Value>,

    /// External guide URL for the item. Nullable.
    #[serde(default)]
    pub guide_url: Option<String>,

    /// Internal MetaForge game asset identifier. Currently absent from the
    /// items endpoint response but kept as an optional field so the model
    /// stays compatible if the upstream API restores it.
    #[serde(default)]
    pub game_asset_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatBlock {
    pub range: f32,
    pub value: Option<f32>,
    pub damage: f32,
    pub health: f32,
    pub radius: f32,
    pub shield: f32,
    pub weight: f32,
    pub agility: f32,

    #[serde(rename = "arcStun")]
    pub arc_stun: f32,

    pub healing: f32,
    pub stamina: f32,
    pub stealth: f32,

    #[serde(rename = "useTime")]
    pub use_time: f32,

    pub duration: f32,

    #[serde(rename = "fireRate")]
    pub fire_rate: f32,

    pub stability: f32,

    #[serde(rename = "stackSize")]
    pub stack_size: f32,

    #[serde(rename = "damageMult")]
    pub damage_mult: f32,

    #[serde(rename = "raiderStun")]
    pub raider_stun: f32,

    #[serde(rename = "weightLimit")]
    pub weight_limit: f32,

    #[serde(rename = "augmentSlots")]
    pub augment_slots: f32,

    #[serde(rename = "healingSlots")]
    pub healing_slots: f32,

    #[serde(rename = "magazineSize")]
    pub magazine_size: f32,

    #[serde(rename = "reducedNoise")]
    pub reduced_noise: f32,

    #[serde(rename = "shieldCharge")]
    pub shield_charge: f32,

    #[serde(rename = "backpackSlots")]
    pub backpack_slots: f32,

    #[serde(rename = "quickUseSlots")]
    pub quick_use_slots: f32,

    #[serde(rename = "damagePerSecond")]
    pub damage_per_second: f32,

    #[serde(rename = "movementPenalty")]
    pub movement_penalty: f32,

    #[serde(rename = "safePocketSlots")]
    pub safe_pocket_slots: f32,

    #[serde(rename = "damageMitigation")]
    pub damage_mitigation: f32,

    #[serde(rename = "healingPerSecond")]
    pub healing_per_second: f32,

    #[serde(rename = "reducedEquipTime")]
    pub reduced_equip_time: f32,

    #[serde(rename = "staminaPerSecond")]
    pub stamina_per_second: f32,

    #[serde(rename = "increasedADSSpeed")]
    pub increased_ads_speed: f32,

    #[serde(rename = "increasedFireRate")]
    pub increased_fire_rate: f32,

    #[serde(rename = "reducedReloadTime")]
    pub reduced_reload_time: f32,

    #[serde(rename = "illuminationRadius")]
    pub illumination_radius: f32,

    #[serde(rename = "increasedEquipTime")]
    pub increased_equip_time: f32,

    #[serde(rename = "reducedUnequipTime")]
    pub reduced_unequip_time: f32,

    #[serde(default, rename = "shieldCompatibility")]
    pub shield_compatibility: Option<String>,

    #[serde(rename = "increasedUnequipTime")]
    pub increased_unequip_time: f32,

    #[serde(rename = "reducedVerticalRecoil")]
    pub reduced_vertical_recoil: f32,

    #[serde(rename = "increasedBulletVelocity")]
    pub increased_bullet_velocity: f32,

    #[serde(rename = "increasedVerticalRecoil")]
    pub increased_vertical_recoil: f32,

    #[serde(rename = "reducedMaxShotDispersion")]
    pub reduced_max_shot_dispersion: f32,

    #[serde(rename = "reducedPerShotDispersion")]
    pub reduced_per_shot_dispersion: f32,

    #[serde(rename = "reducedDurabilityBurnRate")]
    pub reduced_durability_burn_rate: f32,

    #[serde(rename = "reducedRecoilRecoveryTime")]
    pub reduced_recoil_recovery_time: f32,

    #[serde(rename = "increasedRecoilRecoveryTime")]
    pub increased_recoil_recovery_time: f32,

    #[serde(rename = "reducedDispersionRecoveryTime")]
    pub reduced_dispersion_recovery_time: f32,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
