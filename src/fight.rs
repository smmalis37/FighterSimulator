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

    pub fn run(mut self) -> &'a Fighter {
        while match self.run_tick() {
            Some(x) => return x,
            None => true,
        } {}
        unreachable!()
    }

    fn run_tick(&mut self) -> Option<&'a Fighter> {
        let [attacker, defender] = match self.speed_roll[0].cmp(&self.speed_roll[1]) {
            std::cmp::Ordering::Less => [0, 1],
            std::cmp::Ordering::Equal => {
                let x = self.rng.gen_bool(0.5) as usize;
                [x, 1 - x]
            }
            std::cmp::Ordering::Greater => [1, 0],
        };

        if self.d20.sample(&mut self.rng) + self.fighters[attacker].stat(Accuracy)
            >= self.fighters[defender].stat(Dodge)
        {
            let damage = std::cmp::max(
                1,
                (self.d8.sample(&mut self.rng) + self.fighters[attacker].stat(Attack))
                    .saturating_sub(self.fighters[defender].stat(Defense)),
            );
            let dead;
            (self.current_health[defender], dead) =
                self.current_health[defender].overflowing_sub(damage);

            if dead {
                return Some(self.fighters[attacker]);
            }
        }

        self.speed_roll[defender] =
            self.speed_roll[defender].saturating_sub(self.speed_roll[attacker]);
        self.do_speed_roll(attacker);

        None
    }

    fn do_speed_roll(&mut self, fi: usize) {
        self.speed_roll[fi] = self.d20.sample(&mut self.rng) - self.fighters[fi].stat(Speed);
    }
}
