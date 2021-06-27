use std::cmp::Ordering;

use rand::distributions::Uniform;
use rand::prelude::*;

use crate::fighter::*;
use crate::observer::*;
use crate::stats::Stat::*;
use crate::stats::*;

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [&'a Fighter; 2],
    current_health: [StatValue; 2],
    stunned: [bool; 2],
    knockdowns: [u8; 2],
    rng: SmallRng,
    round_count: usize,
    turn_count: usize,
}

impl<'a> Fight<'a> {
    pub fn new(
        f1: &'a Fighter,
        f2: &'a Fighter,
        round_count: usize,
        turn_count: usize,
    ) -> Fight<'a> {
        Fight {
            fighters: [f1, f2],
            current_health: [f1.health(), f2.health()],
            stunned: [false, false],
            knockdowns: [0, 0],
            rng: SmallRng::from_rng(&mut thread_rng()).unwrap(),
            round_count,
            turn_count,
        }
    }

    pub fn run(mut self, o: &mut impl FightObserver<'a>) -> Option<&'a Fighter> {
        let die = Uniform::new_inclusive(1, 8);
        let getup_heal_die = Uniform::new_inclusive(1, 20);

        for round in 1..=self.round_count {
            o.new_round(round);
            for turn in 1..=self.turn_count {
                o.new_turn(turn);
                for (attacker_index, defender_index) in [(0, 1), (1, 0)] {
                    let damaged_health = &mut self.current_health[defender_index];
                    let attacker = self.fighters[attacker_index];
                    let defender = self.fighters[defender_index];

                    if self.stunned[attacker_index] {
                        self.stunned[attacker_index] = false;
                        o.stunned(attacker);
                        continue;
                    }

                    let roll = self.rng.sample(die);
                    let attack = match roll {
                        1 | 7 => Jab,
                        2 => Straight,
                        3 => Hook,
                        4 => Uppercut,
                        5 | 8 => Recovery,
                        6 => Special,
                        0 | 9..=StatValue::MAX => unreachable!(),
                    };
                    let damage = attacker.stat(attack);
                    *damaged_health = damaged_health.saturating_sub(damage);

                    o.attack(attacker, defender, attack, damage, *damaged_health);

                    if *damaged_health == 0 {
                        o.down(defender);

                        let knockdowns = &mut self.knockdowns[defender_index];
                        *knockdowns += 1;

                        if self.rng.gen_bool(1.0 / (*knockdowns as f64 + 1.0)) {
                            *damaged_health =
                                self.rng.sample(getup_heal_die) + self.rng.sample(getup_heal_die);
                            if *knockdowns >= 2 {
                                *damaged_health /= 2;
                            }
                            o.getup(defender, *damaged_health);
                        } else {
                            return Some(attacker);
                        }
                    }

                    match attack {
                        Recovery => {
                            self.heal(o, attacker_index);
                        }
                        Uppercut => {
                            self.stunned[defender_index] = true;
                        }
                        _ => {}
                    }
                }
            }

            for i in 0..=1 {
                self.heal(o, i);
            }
        }

        match self.current_health[0].cmp(&self.current_health[1]) {
            Ordering::Less => Some(self.fighters[1]),
            Ordering::Equal => None,
            Ordering::Greater => Some(self.fighters[0]),
        }
    }

    fn heal(&mut self, o: &mut impl FightObserver<'a>, i: usize) {
        self.current_health[i] = std::cmp::min(
            self.current_health[i] + self.fighters[i].stat(Recovery),
            self.fighters[i].health(),
        );
        o.recovery(self.fighters[i], self.current_health[i]);
    }
}
