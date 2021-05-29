use std::cmp::min;
use std::cmp::Ordering;

use rand::distributions::Uniform;
use rand::prelude::*;

use crate::fighter::*;
use crate::observer::*;
use crate::stats::Stat::*;
use crate::stats::*;

const MAX_HEALTH: StatValue = 40;

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [&'a Fighter; 2],
    current_health: [StatValue; 2],
    speed_penalty: [u8; 2],
    attack_count: [u8; 2],
    rng: SmallRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter) -> Fight<'a> {
        Fight {
            fighters: [f1, f2],
            current_health: [MAX_HEALTH, MAX_HEALTH],
            speed_penalty: [0, 0],
            attack_count: [0, 0],
            rng: SmallRng::from_rng(&mut thread_rng()).unwrap(),
        }
    }

    pub fn run(mut self, o: &mut impl FightObserver<'a>) -> Option<&'a Fighter> {
        let d6 = Uniform::new_inclusive(1, 6);

        for round in 1..=12 {
            o.new_round(round);
            for turn in 1..=3 {
                o.new_turn(turn);
                let speed_rolls = [self.speed_roll(0, o), self.speed_roll(1, o)];

                let (attacker, defender) = match speed_rolls[0].cmp(&speed_rolls[1]) {
                    Ordering::Less => (1, 0),
                    Ordering::Greater => (0, 1),
                    Ordering::Equal => {
                        o.clinch();
                        const CLINCH_HEAL: StatValue = 2;
                        for h in self.current_health.iter_mut() {
                            *h = min(*h + CLINCH_HEAL, MAX_HEALTH);
                        }
                        continue;
                    }
                };
                o.declare_attacker(self.fighters[attacker]);

                let attack_roll = [self.rng.sample(d6), self.rng.sample(d6)];
                let damage =
                    (attack_roll[0] + attack_roll[1] + self.fighters[attacker].stat(Power))
                        .saturating_sub(self.fighters[defender].stat(Toughness));
                let special_move = attack_roll[0] == 6 && attack_roll[1] >= 5;

                self.current_health[defender] =
                    self.current_health[defender].saturating_sub(damage);
                self.attack_count[attacker] += 1;

                o.attack_roll(
                    attack_roll[0],
                    attack_roll[1],
                    damage,
                    self.fighters[defender],
                    self.current_health[defender],
                );

                let (downed, getup_value) = match self.current_health[defender] {
                    15..=40 if (damage > 14 || special_move) => (true, 3),
                    9..=14 if (damage > 12 || special_move) => (true, 4),
                    1..=8 if (damage > 7 || special_move) => (true, 5),
                    0 => return Some(self.fighters[attacker]),
                    _ => (false, 0),
                };

                if downed {
                    o.downed(self.fighters[defender]);
                    let r1 = self.rng.sample(d6);
                    let r2 = self.rng.sample(d6);
                    let rolls = r1 + r2;

                    let heal = if rolls < getup_value {
                        return Some(self.fighters[attacker]);
                    } else if rolls <= 6 {
                        1
                    } else {
                        2
                    };
                    self.current_health[defender] += heal;
                    o.getup_roll(r1, r2, heal, self.current_health[defender]);

                    let speed_penalty = match attack_roll[0] {
                        1 | 2 => 2,
                        3 => 3,
                        4 | 5 => 4,
                        6 => match attack_roll[1] {
                            1 | 2 | 3 | 4 => 4,
                            5 | 6 => 5,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    self.speed_penalty[attacker] += speed_penalty;
                }
            }

            match self.attack_count[0].cmp(&self.attack_count[1]) {
                Ordering::Less => self.speed_penalty[1] += 1,
                Ordering::Equal => {}
                Ordering::Greater => self.speed_penalty[0] += 1,
            }
            self.attack_count = [0, 0];

            for h in self.current_health.iter_mut() {
                *h = min(*h + 2, MAX_HEALTH);
            }

            for h in self.speed_penalty.iter_mut() {
                *h = h.saturating_sub(1);
            }

            for i in 0..=1 {
                o.interval(
                    self.fighters[i],
                    self.current_health[i],
                    self.speed_penalty[i],
                );
            }
        }

        match self.current_health[0].cmp(&self.current_health[1]) {
            Ordering::Less => Some(self.fighters[1]),
            Ordering::Equal => None,
            Ordering::Greater => Some(self.fighters[0]),
        }
    }

    fn speed_roll(&mut self, fighter: usize, o: &mut impl FightObserver<'a>) -> StatValue {
        let d12 = Uniform::new_inclusive(1, 12);
        let r1 = self.rng.sample(d12);
        let r2 = self.rng.sample(d12);
        let result = r1
            + r2
            + self.fighters[fighter]
                .stat(Speed)
                .saturating_sub(self.speed_penalty[fighter]);

        o.speed_roll(
            self.fighters[fighter],
            r1,
            r2,
            self.speed_penalty[fighter],
            result,
        );

        result
    }
}
