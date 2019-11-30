use rand::distributions::Uniform;
use rand::prelude::*;

use crate::fighter::*;
use crate::observer::*;
use crate::stats::Stat::*;
use crate::stats::*;

pub type Round = u32;

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

    pub fn run<O: FightObserver<'a>>(mut self, observer: &mut O) {
        while !self.run_tick(observer) {}
    }

    fn run_tick<O: FightObserver<'a>>(&mut self, observer: &mut O) -> bool {
        if self.next_tick == 0 {
            observer.new_round(self.current_round);
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

        let mut over = self.run_half_tick(observer, first_attacker, second_attacker);

        if !over {
            over = self.run_half_tick(observer, second_attacker, first_attacker);
        }

        self.next_tick += 1;
        if self.next_tick == self.ticks_per_round {
            self.next_tick = 0;
            self.current_round += 1;
        }

        over
    }

    fn run_half_tick<O: FightObserver<'a>>(
        &mut self,
        observer: &mut O,
        attacker_index: usize,
        defender_index: usize,
    ) -> bool {
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];
        let mut over = false;

        // The inverting of who attacks based on whose speed is weird but it's right
        let is_attacking = self.next_tick % defender.stats()[Speed] == 0;
        if is_attacking {
            observer.attack_starting(attacker, defender);
            let damage = self.compute_damage(observer, attacker_index, defender_index);
            let new_health = self.current_health[defender_index].saturating_sub(damage);
            self.current_health[defender_index] = new_health;
            observer.finalize_attack(damage, new_health);

            if new_health == 0 {
                observer.winner(attacker);
                over = true;
            }
        }

        over
    }

    fn compute_damage<O: FightObserver<'a>>(
        &mut self,
        observer: &mut O,
        attacker_index: usize,
        defender_index: usize,
    ) -> StatValue {
        let dice_range = Uniform::new_inclusive(1, DICE_SIZE);
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];
        let mut damage = 0;

        for _ in 0..attacker.stats()[Stat::Attack] {
            let roll = self.rng.sample(dice_range);
            let roll_success = roll > defender.stats()[Stat::Endurance];
            observer.first_roll(roll, roll_success);

            if roll_success {
                let roll = self.rng.sample(dice_range);
                observer.second_roll(roll);
                damage += roll;
            }
        }

        damage
    }
}
