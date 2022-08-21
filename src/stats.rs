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
}

pub(crate) const fn stat_value(stat: Stat, x: StatValue) -> StatValue {
    match stat {
        Stat::Health => (x * 20) + 100,
        Stat::Attack => (x * 2) + 1,
        Stat::Defense => x * 2,
        Stat::Speed => x,
        Stat::Accuracy => x * 10,
        Stat::Dodge => (x * 12) + 25,
    }
}

pub const TOTAL_POINTS: StatValue = 15;
pub const MAX_STAT_POINTS: StatValue = 5;
