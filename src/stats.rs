use enum_map::Enum;
use serde::{Deserialize, Serialize};

pub type StatValue = u16;

#[derive(Debug, Enum, Copy, Clone, Serialize, Deserialize)]
pub enum Stat {
    Health,
    Attack,
    Defense,
    Speed,
    Accuracy,
    Dodge,
    Conviction,
}

pub(crate) const fn stat_value(stat: Stat, x: StatValue) -> StatValue {
    match stat {
        Stat::Health => (x * 325) + 1000,
        Stat::Attack => (x * 15) + 0,
        Stat::Defense => (x * 15) + 0,
        Stat::Speed => x * 8,
        Stat::Accuracy => x * 100,
        Stat::Dodge => (x * 100) + 250,
        Stat::Conviction => x,
    }
}

pub const TOTAL_POINTS: StatValue = 18;
pub const MAX_STAT_POINTS: StatValue = 5;
