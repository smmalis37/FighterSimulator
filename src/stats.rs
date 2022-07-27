use enum_map::{Enum, EnumMap};
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StatMap(EnumMap<Stat, StatValue>);

impl StatMap {
    pub(crate) fn new(
        health: StatValue,
        attack: StatValue,
        defense: StatValue,
        speed: StatValue,
        accuracy: StatValue,
        dodge: StatValue,
    ) -> Self {
        let mut map = EnumMap::default();
        map[Stat::Health] = health;
        map[Stat::Attack] = attack;
        map[Stat::Defense] = defense;
        map[Stat::Speed] = speed;
        map[Stat::Accuracy] = accuracy;
        map[Stat::Dodge] = dodge;
        Self(map)
    }

    pub(crate) fn value(&self, stat: Stat) -> StatValue {
        let x = self.0[stat];
        stat_value(stat, x)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Stat, &StatValue)> {
        self.0.iter()
    }
}

pub(crate) const fn stat_value(stat: Stat, x: StatValue) -> StatValue {
    match stat {
        Stat::Health => (x * 25) + 100,
        Stat::Attack => (x * 2) + 1,
        Stat::Defense => x * 2,
        Stat::Speed => x,
        Stat::Accuracy => x * 10,
        Stat::Dodge => (x * 12) + 25,
    }
}

pub const TOTAL_POINTS: StatValue = 15;
pub const MAX_STAT_POINTS: StatValue = 5;
