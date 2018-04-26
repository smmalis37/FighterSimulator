use stats::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Fighter {
    pub name: String,
    pub stats: HashMap<Stat, StatValue>,
    pub max_health: StatValue,
}

#[derive(Debug)]
pub enum FighterStatError {
    IncorrectPointTotal(StatValue),
    ZeroStat(Stat),
    StatAboveMax(Stat),
}

impl Fighter {
    pub fn new(
        name: String,
        attack: StatValue,
        speed: StatValue,
        endurance: StatValue,
        max_health: StatValue,
    ) -> Result<Fighter, FighterStatError> {
        let stats = {
            let mut map = HashMap::with_capacity(3);
            map.insert(Stat::Attack, attack);
            map.insert(Stat::Speed, speed);
            map.insert(Stat::Endurance, endurance);
            map
        };

        Self::validate_stats(&stats, max_health)?;

        Ok(Fighter {
            name,
            stats,
            max_health,
        })
    }

    fn validate_stats(
        stats: &HashMap<Stat, StatValue>,
        max_health: StatValue,
    ) -> Result<(), FighterStatError> {
        let mut total_cost = 0;
        for (&stat, &value) in stats.iter() {
            if value == 0 && !stat.zero_allowed() {
                return Err(FighterStatError::ZeroStat(stat));
            }
            if value >= stat.costs().len() {
                return Err(FighterStatError::StatAboveMax(stat));
            }
            total_cost += stat.costs()[value];
        }

        total_cost += (max_health - BASE_HEALTH) / HEALTH_PER_POINT;

        if total_cost != TOTAL_POINTS {
            return Err(FighterStatError::IncorrectPointTotal(total_cost));
        }

        Ok(())
    }
}
