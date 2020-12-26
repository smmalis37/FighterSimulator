use enum_map::EnumMap;

pub type StatValue = u8;
pub type SignedStatValue = i16;

#[derive(Debug, Enum, Copy, Clone)]
pub enum Stat {
    Health,
    Skill,
    Speed,
    Strength,
    Resist,
}

#[derive(Debug)]
pub(crate) struct StatMap(EnumMap<Stat, StatValue>);

impl StatMap {
    pub(crate) fn new(
        health: StatValue,
        skill: StatValue,
        speed: StatValue,
        strength: StatValue,
        resist: StatValue,
    ) -> Self {
        let mut map = EnumMap::new();
        map[Stat::Health] = health;
        map[Stat::Skill] = skill;
        map[Stat::Speed] = speed;
        map[Stat::Strength] = strength;
        map[Stat::Resist] = resist;
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
    let x = x as usize;
    (match stat {
        Stat::Health => [15, 30, 45, 60, 75, 90],
        Stat::Skill => [6, 8, 10, 12, 14, 20],
        Stat::Speed => [1, 1, 2, 3, 4, 5],
        Stat::Strength => [0, 1, 2, 3, 4, 5],
        Stat::Resist => [0, 1, 2, 3, 4, 5],
    })[x]
}

pub const TOTAL_POINTS: StatValue = 7;
pub const MAX_STAT_POINTS: StatValue = 5;
