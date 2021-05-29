use enum_map::EnumMap;

pub type StatValue = u8;

#[derive(Debug, Enum, Copy, Clone)]
pub enum Stat {
    Speed,
    Power,
    Toughness,
}

#[derive(Debug)]
pub(crate) struct StatMap(EnumMap<Stat, StatValue>);

impl StatMap {
    pub(crate) fn new(speed: StatValue, power: StatValue, toughness: StatValue) -> Self {
        let map = enum_map! {
            Stat::Speed => speed,
            Stat::Power => power,
            Stat::Toughness => toughness,
        };
        Self(map)
    }

    pub(crate) fn value(&self, stat: Stat) -> StatValue {
        self.0[stat]
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Stat, &StatValue)> {
        self.0.iter()
    }
}

pub const TOTAL_POINTS: StatValue = 15;
pub const MAX_STAT_VALUE: StatValue = 10;
pub const MIN_STAT_VALUE: StatValue = 1;
