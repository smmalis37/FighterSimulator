use enum_map::EnumMap;

pub type StatValue = u8;

#[derive(Debug, Enum, Copy, Clone)]
pub enum Stat {
    Jab,
    Hook,
    Straight,
    Uppercut,
    Special,
    Recovery,
}

#[derive(Debug)]
pub(crate) struct StatMap(EnumMap<Stat, StatValue>);

impl StatMap {
    pub(crate) fn new(
        jab: StatValue,
        hook: StatValue,
        straight: StatValue,
        uppercut: StatValue,
        special: StatValue,
        recovery: StatValue,
    ) -> Self {
        use Stat::*;
        let map = enum_map! {
                Jab => jab,
                Hook => hook,
                Straight => straight,
                Uppercut => uppercut,
                Special => special,
                Recovery => recovery,
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

pub const TOTAL_POINTS: StatValue = 30;
pub const MAX_STAT_VALUE: StatValue = 25;
pub const MIN_STAT_VALUE: StatValue = 1;
