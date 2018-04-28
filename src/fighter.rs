use stats::*;

use enum_map::EnumMap;

#[derive(Debug)]
pub struct Fighter {
    pub name: String,
    pub stats: EnumMap<Stat, StatValue>,
    pub max_health: StatValue,
}

#[derive(Debug)]
pub enum FighterStatError {
    IncorrectPointTotal(StatValue),
    ZeroStat(Stat),
    StatAboveMax(Stat),
    HealthBelowBase,
    HealthNotCleanlyDivisible,
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
            let mut map = EnumMap::new();
            map[Stat::Attack] = attack;
            map[Stat::Speed] = speed;
            map[Stat::Endurance] = endurance;
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
        stats: &EnumMap<Stat, StatValue>,
        max_health: StatValue,
    ) -> Result<(), FighterStatError> {
        let mut total_cost = 0;
        for (stat, &value) in stats.iter() {
            if value == 0 && !stat.zero_allowed() {
                Err(FighterStatError::ZeroStat(stat))?;
            }
            if value >= stat.costs().len() {
                Err(FighterStatError::StatAboveMax(stat))?;
            }
            total_cost += stat.costs()[value];
        }

        if max_health < BASE_HEALTH {
            Err(FighterStatError::HealthBelowBase)?;
        }

        let additional_health = max_health - BASE_HEALTH;

        if additional_health % HEALTH_PER_POINT != 0 {
            Err(FighterStatError::HealthNotCleanlyDivisible)?;
        }

        total_cost += additional_health / HEALTH_PER_POINT;

        if total_cost != TOTAL_POINTS {
            Err(FighterStatError::IncorrectPointTotal(total_cost))?;
        }

        Ok(())
    }
}
