use serde::{Deserialize, Serialize};
use super::common::{deserialize_string_or_vec, DateTimeString, UuidString};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MapDataResponse {
    #[serde(rename = "allData")]
    pub all_data: Vec<MapData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MapData {
    pub id: UuidString,
    pub lat: f32,
    pub lng: f32,
    pub zlayers: i32,
    #[serde(rename = "mapID")]
    pub map_id: Map,
    pub category: Category,
    pub subcategory: Option<SubCategory>,
    #[serde(rename = "instanceName", deserialize_with = "deserialize_string_or_vec")]
    pub instance_name: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub added_by: Option<Vec<String>>,
    #[serde(rename = "behindLockedDoor")]
    pub behind_locked_door: bool,
    #[serde(deserialize_with = "deserialize_string_or_vec")]
    pub last_edited_by: Option<Vec<String>>,
    pub updated_at: Option<DateTimeString>,
    #[serde(rename = "eventConditionMask")]
    pub event_condition_mask: i32,
    #[serde(rename = "lootAreas", deserialize_with = "deserialize_string_or_vec")]
    pub loot_areas: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Map {
    Dam,
    Spaceport,
    BuriedCity,
    BlueGate,
    StellaMontis,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Containers,
    Quests,
    Nature,
    Locations,
    Events,
    #[serde(rename = "arc")]
    Bots,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum LootArea {
    #[serde(rename = "Old World")]
    OldWorld,
    Electrical,
    Medical,
    Commercial,
    Industrial,
    Mechanical,
    Security,
    Technological,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum SubCategory {
    Tick,
    Pop,
    #[serde(rename = "base_container")]
    BaseContainer,
    #[serde(rename = "ammo_crate")]
    AmmoCrate,
    #[serde(rename = "player_spawn")]
    PlayerSpawn,
    #[serde(rename = "breachable_container")]
    BreachableContainer,
    Bison,
    #[serde(rename = "utility_crate")]
    UtilityCrate,
    #[serde(rename = "baron_husk")]
    BaronHusk,
    #[serde(rename = "field_depot")]
    FieldDepot,
    Rocketeer,
    Rollbot,
    Sentinel,
    Turret,
    #[serde(rename = "supply_station")]
    SupplyStation,
    #[serde(rename = "raider_camp")]
    RaiderCamp,
    Queen,
    Locker,
    Fireball,
    #[serde(rename = "med_crate")]
    MedCrate,
    Hatch,
    #[serde(rename = "arc_courier")]
    ArcCourier,
    Harvester,
    #[serde(rename = "raider_cache")]
    RaiderCache,
    Bastion,
    // The trailing-space alias works around an upstream MetaForge data error
    // observed on Buried City rows, where `subcategory` is stored as
    // `"hornet "` (length 7, UTF-8 byte 0x20 at the end) instead of `"hornet"`.
    // Delete the alias once MetaForge cleans up the affected rows.
    #[serde(alias = "hornet ")]
    Hornet,
    #[serde(rename = "weapon_case")]
    WeaponCase,
    #[serde(rename = "arc_husk")]
    ArcHusk,
    Extraction,
    #[serde(rename = "locked_room")]
    LockedRoom,
    Wasp,
    #[serde(rename = "field_crate")]
    FieldCrate,
    Car,
    ABalancedHarvest,
    WaterTroubles,
    SourceOfTheContamination,
    UntendedGarden,
    OurPresenceUpThere,
    FlickeringThreat,
    Bombardier,
    Candleberries,
    Comet,
    #[serde(rename = "arc_probe")]
    ArcProbe,
    PricklyPear,
    Basket,
    Apricot,
    Agave,
    Mushroom,
    GreatMullein,
    Bag,
    WhatWeLeftBehind,
    MedicalMerchandise,
    StraightRecord,
    GreasingHerPalms,
    BrokenMonument,
    EchoesOfVictoryRidge,
    KeepingTheMemory,
    ASymbolOfUnification,
    TheMajorsFootlocker,
    CelestesJournals,
    DormantBarons,
    EyesInTheSky,
    BackOnTop,
    Snitch,
    ANewTypeOfPlant,
    #[serde(rename = "security_breach")]
    SecurityBreach,
    Matriarch,
    Moss,
    Fertilizer,
    Roots,
    #[serde(rename = "breach_room")]
    BreachRoom,
    Espresso,
    AToxicTrail,
    PavingTheWay,
    Button,
    #[serde(rename = "snow_pile")]
    SnowPile,
    ABadFeeling,
    TheLeague,
    SwitchingTheSupply,
    PrescriptionsOfThePast,
    Turnabout,
    Lemons,
    ALayOfTheLand,
    LostInTransmission,
    PowerOut,
    FuelCell,
    Bunker,
    Antenna,
    TheCleanDream,
    TheStenchOfCorruption,
    DecipheringTheData,
    OnTheMap,
    BuildingALibrary,
    TheRootOfTheMatter,
    DiggingUpDirt,
    CommunicationHideout,
    AfterRainComes,
    #[serde(rename = "metro_entrance")]
    MetroEntrance,
    #[serde(rename = "metro_station")]
    MetroStation,
    MarketCorrection,
    EyesOnThePrize,
    EsrAnalyzer,
    Bird,
    MarkedForDeath,
    IndustrialEspionage,
    UnexpectedInitiative,
    AWarmPlaceToRest,
    LifeOfAPharmacist,
    #[serde(rename = "puzzle_button")]
    PuzzleButton,
    CombatRecon,
    Olive,
    AFirstFoothold,
    WithATrace,
    Box,
    ReducedToRubble,
    Bees,
    ArmoredTransports,
    Groundbreaking,
    LockedGateKey,
    APrimeSpecimen,
    OnDeafEars,
    SnapAndSalvage,
    ColdStorage,
    Android,
    Shredder,
    InMyImage,
    WithAView,
    MovieNight,

    /// Catch-all for variants we have not modeled yet (e.g. newly added
    /// game elements) or for malformed values returned by the upstream
    /// API (e.g. trailing whitespace). Lets deserialization succeed
    /// instead of failing the entire response.
    #[serde(other)]
    Unknown,
}
