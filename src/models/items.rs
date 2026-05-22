use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use super::common::{DateTimeString, UriString};

/// Deserializes a JSON value as `f32`, treating `null` as `0.0`.
///
/// The MetaForge API sometimes sends `null` for stat fields that are
/// normally numeric (e.g. `reducedDurabilityBurnRate`).
fn nullable_f32<'de, D: Deserializer<'de>>(de: D) -> Result<f32, D::Error> {
    Option::<f32>::deserialize(de).map(|o| o.unwrap_or(0.0))
}

/// Deserializes a JSON value that may be a string, an integer, or null
/// into `Option<String>`.
///
/// The MetaForge API sends `shieldCompatibility` and `compatibleWeapons`
/// as a string when populated but as the integer `0` when absent.
fn string_or_int<'de, D: Deserializer<'de>>(de: D) -> Result<Option<String>, D::Error> {
    let v = Option::<Value>::deserialize(de)?;
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) if s.is_empty() => Ok(None),
        Some(Value::String(s)) => Ok(Some(s)),
        Some(Value::Number(n)) => {
            // API sends 0 for "not applicable" — treat as None
            if n.as_f64() == Some(0.0) {
                Ok(None)
            } else {
                Ok(Some(n.to_string()))
            }
        }
        Some(other) => Ok(Some(other.to_string())),
    }
}


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

    /// Long-form article body for the item. Appears to be a reserved
    /// MetaForge schema slot for future guide content; observed `null` on
    /// 100% of items sampled (page 1 of 6, 100/581). The concrete shape
    /// has not been observed yet, so this is typed as `Option<Value>` to
    /// stay forward-compatible. Will be tightened to a proper struct (or
    /// `Option<String>` if markdown) once a populated value appears.
    #[serde(default)]
    pub article: Option<Value>,

    /// External guide URL for the item. Appears to be a reserved
    /// MetaForge schema slot for future guide links; observed `null` on
    /// 100% of items sampled. Typed as `Option<String>` on the
    /// expectation that it will be a URL string when populated.
    #[serde(default)]
    pub guide_url: Option<String>,

    /// Internal MetaForge game asset identifier. Currently absent from the
    /// items endpoint response but kept as an optional field so the model
    /// stays compatible if the upstream API restores it.
    #[serde(default)]
    pub game_asset_id: Option<i64>,

    // --- Fields added by `includeComponents=true` ---

    // --- Fields added by `includeComponents=true` ---

    /// Components required to craft this item.
    /// Shape: `[{"component": {...}, "quantity": N}]`
    #[serde(default)]
    pub components: Vec<Value>,

    /// Items that use this item as a component.
    /// Shape: `[{"item": {...}, ...}]`
    #[serde(default)]
    pub used_in: Vec<Value>,

    /// Components received when recycling this item.
    /// Shape: `[{"component": {...}, "quantity": N}]`
    #[serde(default)]
    pub recycle_components: Vec<Value>,

    /// Items that yield this item when recycled.
    /// Shape: `[{"item": {...}, ...}]`
    #[serde(default)]
    pub recycle_from: Vec<Value>,

    /// Modifications applicable to this item.
    #[serde(default)]
    pub mods: Vec<Value>,

    /// ARC enemies that drop this item.
    /// Shape: `[{"arc": {...}, ...}]`
    #[serde(default)]
    pub dropped_by: Vec<Value>,

    /// Traders that sell this item, with their prices.
    #[serde(default)]
    pub sold_by: Vec<SoldBy>,
}

/// A trader that sells an item at a given price.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoldBy {
    pub price: i32,
    pub trader_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StatBlock {
    #[serde(deserialize_with = "nullable_f32")]
    pub range: f32,
    pub value: Option<f32>,
    #[serde(deserialize_with = "nullable_f32")]
    pub damage: f32,
    #[serde(deserialize_with = "nullable_f32")]
    pub health: f32,
    #[serde(deserialize_with = "nullable_f32")]
    pub radius: f32,
    #[serde(deserialize_with = "nullable_f32")]
    pub shield: f32,
    #[serde(deserialize_with = "nullable_f32")]
    pub weight: f32,
    #[serde(deserialize_with = "nullable_f32")]
    pub agility: f32,

    #[serde(rename = "arcStun", deserialize_with = "nullable_f32")]
    pub arc_stun: f32,

    #[serde(deserialize_with = "nullable_f32")]
    pub healing: f32,
    #[serde(deserialize_with = "nullable_f32")]
    pub stamina: f32,
    #[serde(deserialize_with = "nullable_f32")]
    pub stealth: f32,

    #[serde(rename = "useTime", deserialize_with = "nullable_f32")]
    pub use_time: f32,

    #[serde(deserialize_with = "nullable_f32")]
    pub duration: f32,

    #[serde(rename = "fireRate", deserialize_with = "nullable_f32")]
    pub fire_rate: f32,

    #[serde(deserialize_with = "nullable_f32")]
    pub stability: f32,

    #[serde(rename = "stackSize", deserialize_with = "nullable_f32")]
    pub stack_size: f32,

    #[serde(rename = "damageMult", deserialize_with = "nullable_f32")]
    pub damage_mult: f32,

    #[serde(rename = "raiderStun", deserialize_with = "nullable_f32")]
    pub raider_stun: f32,

    #[serde(rename = "weightLimit", deserialize_with = "nullable_f32")]
    pub weight_limit: f32,

    #[serde(rename = "augmentSlots", deserialize_with = "nullable_f32")]
    pub augment_slots: f32,

    #[serde(rename = "healingSlots", deserialize_with = "nullable_f32")]
    pub healing_slots: f32,

    #[serde(rename = "magazineSize", deserialize_with = "nullable_f32")]
    pub magazine_size: f32,

    #[serde(rename = "reducedNoise", deserialize_with = "nullable_f32")]
    pub reduced_noise: f32,

    #[serde(rename = "shieldCharge", deserialize_with = "nullable_f32")]
    pub shield_charge: f32,

    #[serde(rename = "backpackSlots", deserialize_with = "nullable_f32")]
    pub backpack_slots: f32,

    #[serde(rename = "quickUseSlots", deserialize_with = "nullable_f32")]
    pub quick_use_slots: f32,

    #[serde(rename = "damagePerSecond", deserialize_with = "nullable_f32")]
    pub damage_per_second: f32,

    #[serde(rename = "movementPenalty", deserialize_with = "nullable_f32")]
    pub movement_penalty: f32,

    #[serde(rename = "safePocketSlots", deserialize_with = "nullable_f32")]
    pub safe_pocket_slots: f32,

    #[serde(rename = "damageMitigation", deserialize_with = "nullable_f32")]
    pub damage_mitigation: f32,

    #[serde(rename = "healingPerSecond", deserialize_with = "nullable_f32")]
    pub healing_per_second: f32,

    #[serde(rename = "reducedEquipTime", deserialize_with = "nullable_f32")]
    pub reduced_equip_time: f32,

    #[serde(rename = "staminaPerSecond", deserialize_with = "nullable_f32")]
    pub stamina_per_second: f32,

    #[serde(rename = "increasedADSSpeed", deserialize_with = "nullable_f32")]
    pub increased_ads_speed: f32,

    #[serde(rename = "increasedFireRate", deserialize_with = "nullable_f32")]
    pub increased_fire_rate: f32,

    #[serde(rename = "reducedReloadTime", deserialize_with = "nullable_f32")]
    pub reduced_reload_time: f32,

    #[serde(rename = "illuminationRadius", deserialize_with = "nullable_f32")]
    pub illumination_radius: f32,

    #[serde(rename = "increasedEquipTime", deserialize_with = "nullable_f32")]
    pub increased_equip_time: f32,

    #[serde(rename = "reducedUnequipTime", deserialize_with = "nullable_f32")]
    pub reduced_unequip_time: f32,

    #[serde(default, rename = "shieldCompatibility", deserialize_with = "string_or_int")]
    pub shield_compatibility: Option<String>,

    #[serde(default, rename = "compatibleWeapons", deserialize_with = "string_or_int")]
    pub compatible_weapons: Option<String>,

    #[serde(rename = "increasedUnequipTime", deserialize_with = "nullable_f32")]
    pub increased_unequip_time: f32,

    #[serde(rename = "reducedVerticalRecoil", deserialize_with = "nullable_f32")]
    pub reduced_vertical_recoil: f32,

    #[serde(rename = "increasedBulletVelocity", deserialize_with = "nullable_f32")]
    pub increased_bullet_velocity: f32,

    #[serde(rename = "increasedVerticalRecoil", deserialize_with = "nullable_f32")]
    pub increased_vertical_recoil: f32,

    #[serde(rename = "reducedMaxShotDispersion", deserialize_with = "nullable_f32")]
    pub reduced_max_shot_dispersion: f32,

    #[serde(rename = "reducedPerShotDispersion", deserialize_with = "nullable_f32")]
    pub reduced_per_shot_dispersion: f32,

    #[serde(rename = "reducedDurabilityBurnRate", deserialize_with = "nullable_f32")]
    pub reduced_durability_burn_rate: f32,

    #[serde(rename = "reducedRecoilRecoveryTime", deserialize_with = "nullable_f32")]
    pub reduced_recoil_recovery_time: f32,

    #[serde(rename = "increasedRecoilRecoveryTime", deserialize_with = "nullable_f32")]
    pub increased_recoil_recovery_time: f32,

    #[serde(rename = "reducedDispersionRecoveryTime", deserialize_with = "nullable_f32")]
    pub reduced_dispersion_recovery_time: f32,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
