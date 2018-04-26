use rand;
use rand::Rng;

use fighter::*;
use stats::Stat::*;
use stats::*;

const DICE_SIZE: StatValue = 6;

type Round = u32;

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [&'a Fighter; 2],
    current_health: [StatValue; 2],
    ticks_per_round: StatValue,
    next_tick: StatValue,
    current_round: Round,
}

#[derive(Debug)]
pub struct Report<'a> {
    pub new_round: Option<Round>,
    pub attacks: [Option<Attack<'a>>; 2],
    pub remaining_healths: [Option<StatValue>; 2],
    pub winner: Option<&'a Fighter>,
}

#[derive(Debug)]
pub struct Attack<'a> {
    pub attacker: &'a Fighter,
    pub defender: &'a Fighter,
    pub first_rolls: Vec<StatValue>,
    pub second_rolls: Vec<StatValue>,
    pub damage: StatValue,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter) -> Fight<'a> {
        Fight {
            fighters: [f1, f2],
            current_health: [f1.max_health, f2.max_health],
            ticks_per_round: f1.stats[&Speed] * f2.stats[&Speed],
            next_tick: 1,
            current_round: 1,
        }
    }

    pub fn run_with_reporting<F: Fn(&Report)>(mut self, report_handler: F) -> &'a Fighter {
        loop {
            let report = self.run_tick();
            report_handler(&report);
            if let Some(winner) = report.winner {
                break winner;
            }
        }
    }

    pub fn run(self) -> &'a Fighter {
        self.run_with_reporting(|_| ())
    }

    fn run_tick(&mut self) -> Report<'a> {
        let mut report = Report {
            new_round: if self.next_tick == 1 {
                Some(self.current_round)
            } else {
                None
            },
            attacks: [None, None],
            remaining_healths: [None, None],
            winner: None,
        };

        let f0 = self.fighters[0];
        let f1 = self.fighters[1];

        let (first_attacker, second_attacker) = if f0.stats[&Speed] == f1.stats[&Speed] {
            *rand::thread_rng().choose(&[(0, 1), (1, 0)]).unwrap()
        } else if f0.stats[&Speed] > f1.stats[&Speed] {
            (0, 1)
        } else {
            (1, 0)
        };

        self.run_half_tick(&mut report, first_attacker, second_attacker, 0);

        if report.winner.is_none() {
            self.run_half_tick(&mut report, second_attacker, first_attacker, 1);
        }

        self.next_tick += 1;
        if self.next_tick > self.ticks_per_round {
            self.next_tick = 1;
            self.current_round += 1;
        }

        report
    }

    fn run_half_tick(
        &mut self,
        report: &mut Report<'a>,
        attacker_index: usize,
        defender_index: usize,
        attack_index: usize,
    ) {
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];

        // The inverting of who attacks based on whose speed is weird but it's right
        let is_attacking = self.next_tick % defender.stats[&Speed] == 0;
        if is_attacking {
            let attack = self.generate_attack(attacker, defender);
            report.winner = self.apply_attack(&attack, defender_index);
            report.remaining_healths[attack_index] = Some(self.current_health[defender_index]);
            report.attacks[attack_index] = Some(attack);
        }
    }

    fn generate_attack(&self, attacker: &'a Fighter, defender: &'a Fighter) -> Attack<'a> {
        let first_rolls: Vec<_> = (0..attacker.stats[&Attack])
            .map(|_| rand::thread_rng().gen_range(0, DICE_SIZE) + 1)
            .collect();
        let second_rolls: Vec<_> = first_rolls
            .iter()
            .filter(|roll| **roll > defender.stats[&Endurance])
            .map(|_| rand::thread_rng().gen_range(0, DICE_SIZE) + 1)
            .collect();
        let damage = second_rolls.iter().sum();

        Attack {
            attacker,
            defender,
            first_rolls,
            second_rolls,
            damage,
        }
    }

    fn apply_attack(&mut self, attack: &Attack<'a>, defender_index: usize) -> Option<&'a Fighter> {
        self.current_health[defender_index] =
            self.current_health[defender_index].saturating_sub(attack.damage);

        if self.current_health[defender_index] == 0 {
            Some(attack.attacker)
        } else {
            None
        }
    }
}
