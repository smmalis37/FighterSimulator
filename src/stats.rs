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
            Stat::Health => (x * 200) + 1000,
            Stat::Attack => x * 8,
            Stat::Defense => x * 8,
            Stat::Speed => x * 3,
            Stat::Accuracy => x * 80,
            Stat::Dodge => (x * 90) + 250,
            Stat::Conviction => x,
        }
    }
}

pub const TOTAL_POINTS: StatValue = 15;
pub const MAX_STAT_POINTS: StatValue = 5;
