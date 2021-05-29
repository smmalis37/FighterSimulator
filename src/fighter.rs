use crate::stats::*;

#[derive(Debug)]
pub struct Fighter {
    name: String,
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
        speed_points: StatValue,
        power_points: StatValue,
        tough_points: StatValue,
    ) -> Result<Fighter, FighterStatError> {
        let stats = StatMap::new(speed_points, power_points, tough_points);

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

        Ok(Fighter { name, stats })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn stat(&self, stat: Stat) -> StatValue {
        self.stats.value(stat)
    }
}
