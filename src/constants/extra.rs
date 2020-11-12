//! Additional constants representing internal game mechanics that aren't included in the game's constants

/// Maximum amount of CPU that can be accumulated in your bucket per shard.
pub const CPU_BUCKET_MAX: u32 = 10_000;

/// Maximum value of [`CpuInfo::tick_limit`] if more bucket is available than can be used this tick.
///
/// [`CpuInfo::tick_limit`]: crate::objects::game::CpuInfo::tick_limit
pub const CPU_TICK_LIMIT_MAX: u32 = 500;

/// Hits per creep body part.
pub const CREEP_HITS_PER_PART: u32 = 50;

/// Percentage of energy spent on construction that is list if the construction site is destroyed by being stepped on by a hostile creep.
pub const CONSTRUCTION_SITE_STOMP_RATIO: f32 = 0.5;

/// Owner username of hostile non-player structures and creeps which create strongholds and spawn in rooms due to energy harvesting.
pub const INVADER_USERNAME: &str = "Invader";

/// Fatigue points removed per effective move part per tick.
pub const MOVE_POWER: u32 = 2;

/// Maximum hits of a power creep per level
pub const POWER_CREEP_HITS_PER_LEVEL: u32 = 1000;

/// Exponential scaling rate for terminal energy costs.
///
/// Energy cost to send resources with a terminal is determined by the formula ([source]):
///
/// ```js
/// Math.ceil(resource_amount * (1 - Math.exp(-range / 30)))
/// ```
///
/// [source]: https://github.com/screeps/engine/blob/c682c00b058de21e927c3a6c42fadb34c9745767/src/utils.js#L658
pub const TERMINAL_SEND_COST_SCALE: u32 = 30;

/// Hits of damage per effective ranged attack part per [`Creep::ranged_mass_attack`]
/// action at range 1.
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_1: u32 = 10;
/// Hits of damage per effective ranged attack part per [`Creep::ranged_mass_attack`]
/// action at range 2.
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_2: u32 = 4;
/// Hits of damage per effective ranged attack part per [`Creep::ranged_mass_attack`]
/// action at range 1
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_3: u32 = 1;
