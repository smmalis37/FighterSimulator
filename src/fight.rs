use rand::distributions::Uniform;
use rand::prelude::*;

use crate::fighter::*;
use crate::stats::Stat::*;
use crate::stats::*;

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [&'a Fighter; 2],
    current_health: [StatValue; 2],
    speed_roll: [StatValue; 2],
    d8: Uniform<StatValue>,
    d20: Uniform<StatValue>,
    rng: SmallRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter) -> Fight<'a> {
        let mut f = Fight {
            fighters: [f1, f2],
            current_health: [f1.stat(Health), f2.stat(Health)],
            speed_roll: [0, 0],
            d8: Uniform::new_inclusive(1, 8),
            d20: Uniform::new_inclusive(1, 20),
            rng: SmallRng::from_rng(&mut thread_rng()).unwrap(),
        };
        f.do_speed_roll(0);
        f.do_speed_roll(1);
        f
    }

    pub fn run(mut self, mut logger: impl FnMut(&str)) -> &'a Fighter {
        while match self.run_tick(&mut logger) {
            Some(x) => return x,
            None => true,
        } {}
        unreachable!()
    }

    fn run_tick(&mut self, logger: &mut impl FnMut(&str)) -> Option<&'a Fighter> {
        logger(&format!(
            "Speed rolls are {}: {}, {}: {}",
            self.fighters[0].name(),
            self.speed_roll[0],
            self.fighters[1].name(),
            self.speed_roll[1]
        ));
        let [attacker, defender] = match self.speed_roll[0].cmp(&self.speed_roll[1]) {
            std::cmp::Ordering::Less => [0, 1],
            std::cmp::Ordering::Equal => {
                logger("It's a tie!");
                let x = self.rng.gen_bool(0.5) as usize;
                [x, 1 - x]
            }
            std::cmp::Ordering::Greater => [1, 0],
        };
        logger(&format!("{} is attacking!", self.fighters[attacker].name()));

        let hit_roll = self.d20.sample(&mut self.rng);
        logger(&format!(
            "A roll of {} + {} against {}'s dodge of {}.",
            hit_roll,
            self.fighters[attacker].stat(Accuracy),
            self.fighters[defender].name(),
            self.fighters[defender].stat(Dodge)
        ));

        if hit_roll + self.fighters[attacker].stat(Accuracy) >= self.fighters[defender].stat(Dodge)
        {
            let damage_roll = self.d8.sample(&mut self.rng);
            let damage = std::cmp::max(
                1,
                (damage_roll + self.fighters[attacker].stat(Attack))
                    .saturating_sub(self.fighters[defender].stat(Defense)),
            );
            logger(&format!(
                "A roll of {} + {} against a defense of {} means {} damage.",
                damage_roll,
                self.fighters[attacker].stat(Attack),
                self.fighters[defender].stat(Defense),
                damage
            ));

            let dead;
            (self.current_health[defender], dead) =
                self.current_health[defender].overflowing_sub(damage);

            if dead {
                logger(&format!(
                    "{} goes down! The fight is over! {} wins!",
                    self.fighters[defender].name(),
                    self.fighters[attacker].name()
                ));
                return Some(self.fighters[attacker]);
            } else {
                logger(&format!(
                    "{} is now down to {} health.",
                    self.fighters[defender].name(),
                    self.current_health[defender]
                ));
            }
        } else {
            logger("Miss!");
        }

        self.speed_roll[defender] =
            self.speed_roll[defender].saturating_sub(self.speed_roll[attacker]);
        self.do_speed_roll(attacker);

        None
    }

    fn do_speed_roll(&mut self, fi: usize) {
        self.speed_roll[fi] = self
            .d20
            .sample(&mut self.rng)
            .saturating_sub(self.fighters[fi].stat(Speed));
    }
}
