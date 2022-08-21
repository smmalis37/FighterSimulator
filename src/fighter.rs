use crate::stats::*;
use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fighter {
    name: String,
    stats: EnumMap<Stat, StatValue>,
}

#[derive(Debug)]
pub enum FighterStatError {
    IncorrectPointTotal(StatValue),
    StatAboveMax(Stat),
}

impl Fighter {
    pub fn new(
        name: String,
        health: StatValue,
        attack: StatValue,
        defense: StatValue,
        speed: StatValue,
        accuracy: StatValue,
        dodge: StatValue,
    ) -> Result<Fighter, FighterStatError> {
        let stats = {
            let mut map = EnumMap::default();
            map[Stat::Health] = health;
            map[Stat::Attack] = attack;
            map[Stat::Defense] = defense;
            map[Stat::Speed] = speed;
            map[Stat::Accuracy] = accuracy;
            map[Stat::Dodge] = dodge;
            map
        };

        let mut total_cost = 0;

        for (stat, &value) in stats.iter() {
            if value > MAX_STAT_POINTS {
                return Err(FighterStatError::StatAboveMax(stat));
            }
            total_cost += value;
        }

        if total_cost != TOTAL_POINTS {
            return Err(FighterStatError::IncorrectPointTotal(total_cost));
        }

        Ok(Fighter { name, stats })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn stat(&self, stat: Stat) -> StatValue {
        stat_value(stat, self.raw_stat(stat))
    }

    pub(crate) fn raw_stat(&self, stat: Stat) -> StatValue {
        self.stats[stat]
    }
}
