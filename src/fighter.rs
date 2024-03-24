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
        conviction: StatValue,
    ) -> Fighter {
        let stats = {
            let mut map = EnumMap::default();
            map[Stat::Health] = health;
            map[Stat::Attack] = attack;
            map[Stat::Defense] = defense;
            map[Stat::Speed] = speed;
            map[Stat::Accuracy] = accuracy;
            map[Stat::Dodge] = dodge;
            map[Stat::Conviction] = conviction;
            map
        };

        Fighter { name, stats }
    }

    pub fn validate(&self, print: bool) -> bool {
        let mut total_cost = 0;
        let mut valid = true;

        for (stat, &value) in self.stats.iter() {
            if value > MAX_STAT_POINTS {
                if print {
                    println!(
                        "Warning, {}'s {:?} is above the normal maximum.",
                        self.name(),
                        stat
                    );
                }
                valid = false;
            }
            total_cost += value;
        }

        if total_cost != TOTAL_POINTS {
            if print {
                println!(
                    "Warning, {}'s stat total of {} is not equal to the normal maximum of {}.",
                    self.name(),
                    total_cost,
                    TOTAL_POINTS
                );
            }
            valid = false;
        }

        valid
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
