use enum_map::EnumMap;

pub type StatValue = u16;

#[derive(Debug, Enum, Copy, Clone)]
pub enum Stat {
    Health,
    Attack,
    Defense,
    Speed,
    Accuracy,
    Dodge,
}

#[derive(Debug)]
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
        Stat::Health => (x * 50) + 100,
        Stat::Attack => (x * 4) + 1,
        Stat::Defense => x * 3,
        Stat::Speed => x,
        Stat::Accuracy => x * 10,
        Stat::Dodge => (x * 13) + 25,
    }
}

pub const TOTAL_POINTS: StatValue = 15;
pub const MAX_STAT_POINTS: StatValue = 5;
