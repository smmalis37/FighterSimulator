use rand::distributions::Uniform;
use rand::prelude::*;

use fighter::*;
use report::*;
use stats::Stat::*;
use stats::*;

pub(crate) type Round = u32;

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [&'a Fighter; 2],
    current_health: [StatValue; 2],
    ticks_per_round: StatValue,
    next_tick: StatValue,
    current_round: Round,
    rng: SmallRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter) -> Fight<'a> {
        Fight {
            fighters: [f1, f2],
            current_health: [*f1.max_health(), *f2.max_health()],
            ticks_per_round: f1.stats()[Speed] * f2.stats()[Speed],
            next_tick: 0,
            current_round: 1,
            rng: SmallRng::from_rng(&mut thread_rng()).unwrap(),
        }
    }

    pub fn run<R: Report<'a>>(mut self, reporter: &mut R) {
        while !self.run_tick(reporter) {}
    }

    fn run_tick<R: Report<'a>>(&mut self, reporter: &mut R) -> bool {
        if self.next_tick == 0 {
            reporter.new_round(self.current_round);
        }

        let f0 = self.fighters[0];
        let f1 = self.fighters[1];

        let (first_attacker, second_attacker) = if f0.stats()[Speed] == f1.stats()[Speed] {
            if self.rng.gen() {
                (0, 1)
            } else {
                (1, 0)
            }
        } else if f0.stats()[Speed] > f1.stats()[Speed] {
            (0, 1)
        } else {
            (1, 0)
        };

        let mut over = self.run_half_tick(reporter, first_attacker, second_attacker);

        if !over {
            over = self.run_half_tick(reporter, second_attacker, first_attacker);
        }

        self.next_tick += 1;
        if self.next_tick == self.ticks_per_round {
            self.next_tick = 0;
            self.current_round += 1;
        }

        over
    }

    fn run_half_tick<R: Report<'a>>(
        &mut self,
        reporter: &mut R,
        attacker_index: usize,
        defender_index: usize,
    ) -> bool {
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];
        let mut over = false;

        // The inverting of who attacks based on whose speed is weird but it's right
        let is_attacking = self.next_tick % defender.stats()[Speed] == 0;
        if is_attacking {
            reporter.attack(attacker, defender);
            let damage = self.compute_damage(reporter, attacker_index, defender_index);
            let new_health = self.current_health[defender_index].saturating_sub(damage);
            self.current_health[defender_index] = new_health;
            if new_health == 0 {
                reporter.winner(attacker);
                over = true;
            }
            reporter.finalize_attack(damage, new_health);
        }

        over
    }

    fn compute_damage<R: Report<'a>>(
        &mut self,
        reporter: &mut R,
        attacker_index: usize,
        defender_index: usize,
    ) -> StatValue {
        let dice_range = Uniform::new_inclusive(1, DICE_SIZE);
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];
        let mut damage = 0;

        for _ in 0..attacker.stats()[Stat::Attack] {
            let roll = self.rng.sample(dice_range);
            reporter.first_roll(roll);

            if roll > defender.stats()[Stat::Endurance] {
                let roll = self.rng.sample(dice_range);
                reporter.second_roll(roll);
                damage += roll;
            }
        }

        damage
    }
}
