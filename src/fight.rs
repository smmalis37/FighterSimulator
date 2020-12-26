use arrayvec::ArrayVec;
use rand::distributions::Uniform;
use rand::prelude::*;

use crate::fighter::*;
use crate::observer::*;
use crate::stats::Stat::*;
use crate::stats::*;

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [&'a Fighter; 2],
    current_health: [SignedStatValue; 2],
    dice: [Uniform<StatValue>; 2],
    rng: SmallRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter) -> Fight<'a> {
        Fight {
            fighters: [f1, f2],
            current_health: [
                f1.stat(Health) as SignedStatValue,
                f2.stat(Health) as SignedStatValue,
            ],
            dice: [
                Uniform::new_inclusive(1, f1.stat(Skill)),
                Uniform::new_inclusive(1, f2.stat(Skill)),
            ],
            rng: SmallRng::from_rng(&mut thread_rng()).unwrap(),
        }
    }

    pub fn run<O: FightObserver<'a>>(mut self, observer: &mut O) {
        while self.run_tick(observer) {}
    }

    #[allow(clippy::comparison_chain)]
    fn run_tick<O: FightObserver<'a>>(&mut self, observer: &mut O) -> bool {
        self.run_half_tick(observer, 0, 1);
        self.run_half_tick(observer, 1, 0);

        let f0_health = self.current_health[0];
        let f1_health = self.current_health[1];

        if f0_health <= 0 || f1_health <= 0 {
            observer.winner(if f0_health < f1_health {
                Some(self.fighters[1])
            } else if f1_health < f0_health {
                Some(self.fighters[0])
            } else {
                None
            });
            false
        } else {
            true
        }
    }

    fn run_half_tick<O: FightObserver<'a>>(
        &mut self,
        observer: &mut O,
        attacker_index: usize,
        defender_index: usize,
    ) {
        let attacker = self.fighters[attacker_index];
        let defender = self.fighters[defender_index];
        let dice_size = self.dice[attacker_index];

        observer.attack_starting(attacker, defender);

        let mut rolls = ArrayVec::<[_; MAX_STAT_POINTS as usize + 1]>::new(); // TODO make better
        for _ in 0..attacker.stat(Speed) {
            rolls.push(self.rng.sample(dice_size));
        }

        observer.rolls(&rolls);

        for r in rolls.iter_mut() {
            *r = (*r + attacker.stat(Strength)).saturating_sub(defender.stat(Resist));
        }

        observer.adjusts(&rolls);

        let damage = rolls.into_iter().sum();
        self.current_health[defender_index] -= damage as SignedStatValue;

        observer.finalize_attack(damage, self.current_health[defender_index]);
    }
}
