use arrayvec::ArrayVec;
use rand;
use rand::Rng;

use fighter::*;
use report::*;
use stats::Stat::*;
use stats::*;

const DICE_SIZE: StatValue = 6;

pub(crate) type Round = u32;

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [&'a Fighter; 2],
    current_health: [StatValue; 2],
    ticks_per_round: StatValue,
    next_tick: StatValue,
    current_round: Round,
    rng: rand::XorShiftRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter) -> Fight<'a> {
        Fight {
            fighters: [f1, f2],
            current_health: [f1.max_health, f2.max_health],
            ticks_per_round: f1.stats[Speed] * f2.stats[Speed],
            next_tick: 0,
            current_round: 1,
            rng: rand::weak_rng(),
        }
    }

    pub fn run_with_reporting<F: Fn(&FullReport)>(mut self, report_handler: F) -> &'a Fighter {
        loop {
            let report = self.run_tick();
            report_handler(&report);
            if let Some(winner) = report.winner {
                break winner;
            }
        }
    }

    pub fn run(mut self) -> &'a Fighter {
        loop {
            let report = self.run_tick::<WinnerOnlyReport>();
            if let Some(winner) = report.winner {
                break winner;
            }
        }
    }

    fn run_tick<R: Report<'a>>(&mut self) -> R {
        let mut report = R::new();
        if self.next_tick == 0 {
            report.set_new_round(self.current_round);
        }

        let f0 = self.fighters[0];
        let f1 = self.fighters[1];

        let (first_attacker, second_attacker) = if f0.stats[Speed] == f1.stats[Speed] {
            *self.rng.choose(&[(0, 1), (1, 0)]).unwrap()
        } else if f0.stats[Speed] > f1.stats[Speed] {
            (0, 1)
        } else {
            (1, 0)
        };

        self.run_half_tick(&mut report, first_attacker, second_attacker, 0);

        if report.get_winner().is_none() {
            self.run_half_tick(&mut report, second_attacker, first_attacker, 1);
        }

        self.next_tick += 1;
        if self.next_tick >= self.ticks_per_round {
            self.next_tick = 0;
            self.current_round += 1;
        }

        report
    }

    fn run_half_tick<R: Report<'a>>(
        &mut self,
        report: &mut R,
        attacker_index: usize,
        defender_index: usize,
        attack_index: usize,
    ) {
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];

        // The inverting of who attacks based on whose speed is weird but it's right
        let is_attacking = self.next_tick % defender.stats[Speed] == 0;
        if is_attacking {
            let attack = self.generate_attack(attacker, defender);
            if let Some(winner) = self.apply_attack(&attack, defender_index) {
                report.set_winner(winner);
            }
            report.set_attack(attack_index, attack, self.current_health[defender_index]);
        }
    }

    fn generate_attack<A: AttackReport<'a>>(
        &mut self,
        attacker: &'a Fighter,
        defender: &'a Fighter,
    ) -> A {
        let first_rolls: ArrayVec<_> = (0..attacker.stats[Attack])
            .map(|_| self.rng.gen_range(0, DICE_SIZE) + 1)
            .collect();
        let second_rolls: ArrayVec<_> = first_rolls
            .iter()
            .filter(|roll| **roll > defender.stats[Endurance])
            .map(|_| self.rng.gen_range(0, DICE_SIZE) + 1)
            .collect();
        let damage = second_rolls.iter().sum();

        A::new(attacker, defender, first_rolls, second_rolls, damage)
    }

    fn apply_attack<A: AttackReport<'a>>(
        &mut self,
        attack: &A,
        defender_index: usize,
    ) -> Option<&'a Fighter> {
        self.current_health[defender_index] =
            self.current_health[defender_index].saturating_sub(attack.get_damage());

        if self.current_health[defender_index] == 0 {
            Some(attack.get_attacker())
        } else {
            None
        }
    }
}
