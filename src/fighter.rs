use crate::stats::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fighter {
    name: String,
    health: StatValue,
    stats: StatMap,
}

#[derive(Debug)]
pub enum FighterStatError {
    IncorrectPointTotal(StatValue),
    StatAboveMax(Stat),
    StatBelowMin(Stat),
}

impl Fighter {
    pub fn new(
        name: String,
        health: StatValue,
        jab: StatValue,
        hook: StatValue,
        straight: StatValue,
        uppercut: StatValue,
        special: StatValue,
        recovery: StatValue,
    ) -> Result<Fighter, FighterStatError> {
        let stats = StatMap::new(jab, hook, straight, uppercut, special, recovery);

        let mut total_cost = 0;

        for (stat, &value) in stats.iter() {
            if value > MAX_STAT_VALUE {
                return Err(FighterStatError::StatAboveMax(stat));
            }
            if value < MIN_STAT_VALUE {
                return Err(FighterStatError::StatBelowMin(stat));
            }
            total_cost += value;
        }

        if total_cost != TOTAL_POINTS {
            return Err(FighterStatError::IncorrectPointTotal(total_cost));
        }

        Ok(Fighter {
            name,
            health,
            stats,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn stat(&self, stat: Stat) -> StatValue {
        self.stats.value(stat)
    }

    pub(crate) fn health(&self) -> StatValue {
        self.health
    }
}
