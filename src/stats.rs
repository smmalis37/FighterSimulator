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
        match stat {
            Stat::Health => (x + 1) * 20,
            Stat::Skill => x * 2 + 4,
            Stat::Speed => x + 1,
            Stat::Strength => x,
            Stat::Resist => x,
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Stat, &StatValue)> {
        self.0.iter()
    }
}

pub const TOTAL_POINTS: StatValue = 10;
pub const MAX_STAT_POINTS: StatValue = 4;
