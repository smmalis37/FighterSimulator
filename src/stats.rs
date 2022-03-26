use enum_map::EnumMap;

pub type StatValue = u8;

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
        Stat::Health => (x + 1) * 20,
        Stat::Attack => x,
        Stat::Defense => x,
        Stat::Speed => x,
        Stat::Accuracy => x,
        Stat::Dodge => x + 10,
    }
}

pub const TOTAL_POINTS: StatValue = 15;
pub const MAX_STAT_POINTS: StatValue = 5;
