use rand::*;

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
    rng: XorShiftRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter) -> Fight<'a> {
        Fight {
            fighters: [f1, f2],
            current_health: [*f1.max_health(), *f2.max_health()],
            ticks_per_round: f1.stats()[Speed] * f2.stats()[Speed],
            next_tick: 0,
            current_round: 1,
            rng: weak_rng(),
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

        let (first_attacker, second_attacker) = if f0.stats()[Speed] == f1.stats()[Speed] {
            *self.rng.choose(&[(0, 1), (1, 0)]).unwrap()
        } else if f0.stats()[Speed] > f1.stats()[Speed] {
            (0, 1)
        } else {
            (1, 0)
        };

        self.run_half_tick(&mut report, first_attacker, second_attacker, 0);

        if report.get_winner().is_none() {
            self.run_half_tick(&mut report, second_attacker, first_attacker, 1);
        }

        self.next_tick += 1;
        if self.next_tick == self.ticks_per_round {
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
        half_tick_index: usize,
    ) {
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];

        // The inverting of who attacks based on whose speed is weird but it's right
        let is_attacking = self.next_tick % defender.stats()[Speed] == 0;
        if is_attacking {
            report.set_attack(half_tick_index, attacker, defender);
            let damage =
                self.compute_damage(report, half_tick_index, attacker_index, defender_index);
            let new_health = self.current_health[defender_index].saturating_sub(damage);
            self.current_health[defender_index] = new_health;
            if new_health == 0 {
                report.set_winner(attacker);
            }
            report.finalize_attack(half_tick_index, new_health);
        }
    }

    fn compute_damage<R: Report<'a>>(
        &mut self,
        report: &mut R,
        half_tick_index: usize,
        attacker_index: usize,
        defender_index: usize,
    ) -> StatValue {
        const DICE_SIZE: StatValue = 6;
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];
        let mut surviving_rolls = 0;
        let mut damage = 0;

        for roll_index in 0..attacker.stats()[Stat::Attack] {
            let roll = self.rng.gen_range(0, DICE_SIZE) + 1;
            report.set_first_roll(half_tick_index, roll_index, roll);
            if roll > defender.stats()[Stat::Endurance] {
                let roll = self.rng.gen_range(0, DICE_SIZE) + 1;
                report.set_second_roll(half_tick_index, surviving_rolls, roll);
                surviving_rolls += 1;
                damage += roll;
            }
        }

        damage
    }
}
