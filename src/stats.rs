use enum_map::Enum;
use serde::{Deserialize, Serialize};

pub type StatValue = u16;
pub type SignedStatValue = i16;

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

impl Stat {
    pub(crate) const fn effective_value(self, x: StatValue) -> StatValue {
        match self {
            Stat::Health => (x * 325) + 1000,
            Stat::Attack => x * 15,
            Stat::Defense => x * 15,
            Stat::Speed => x * 8,
            Stat::Accuracy => x * 100,
            Stat::Dodge => (x * 100) + 250,
            Stat::Conviction => x,
        }
    }
}

pub const TOTAL_POINTS: StatValue = 18;
pub const MAX_STAT_POINTS: StatValue = 5;
