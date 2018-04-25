use stats::*;

#[derive(Debug)]
pub struct Fighter {
    pub name: String,
    pub attack: Stat,
    pub speed: Stat,
    pub endurance: Stat,
    pub max_health: Stat,
}

#[derive(Debug)]
pub enum FighterStatError {
    IncorrectPointTotal(Stat),
}

impl Fighter {
    pub fn new(
        name: String,
        attack: Stat,
        speed: Stat,
        endurance: Stat,
        max_health: Stat,
    ) -> Result<Fighter, FighterStatError> {
        Self::validate_stats(attack, speed, endurance, max_health)?;
        Ok(Fighter {
            name,
            attack,
            speed,
            endurance,
            max_health,
        })
    }

    fn validate_stats(
        attack: Stat,
        speed: Stat,
        endurance: Stat,
        max_health: Stat,
    ) -> Result<(), FighterStatError> {
        let fighter_cost = ATTACK_COSTS[attack as usize] + SPEED_COSTS[speed as usize]
            + ENDURANCE_COSTS[endurance as usize]
            + ((max_health - BASE_HEALTH) / HEALTH_PER_POINT);

        if fighter_cost != TOTAL_POINTS {
            Err(FighterStatError::IncorrectPointTotal(fighter_cost))
        } else {
            Ok(())
        }
    }
}
