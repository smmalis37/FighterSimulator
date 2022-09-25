use rand::distributions::Uniform;
use rand::prelude::*;
use static_init::dynamic;

use crate::fighter::*;
use crate::stats::Stat::*;
use crate::stats::*;

#[dynamic]
static D14: Uniform<StatValue> = Uniform::new_inclusive(1, 14);
#[dynamic]
static D20: Uniform<StatValue> = Uniform::new_inclusive(1, 20);
#[dynamic]
static D100: Uniform<StatValue> = Uniform::new_inclusive(1, 100);

#[derive(Debug)]
struct FightFighter<'a> {
    fighter: &'a Fighter,
    current_health: StatValue,
    speed_roll: StatValue,
}

impl<'a> FightFighter<'a> {
    fn new(fighter: &'a Fighter) -> Self {
        Self {
            fighter,
            current_health: fighter.stat(Health),
            speed_roll: 0,
        }
    }
}

impl<'a> std::ops::Deref for FightFighter<'a> {
    type Target = Fighter;

    fn deref(&self) -> &Self::Target {
        self.fighter
    }
}

#[derive(Debug)]
pub struct Fight<'a> {
    fighters: [FightFighter<'a>; 2],
    rng: SmallRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter, seed: u64) -> Fight<'a> {
        let mut f = Self {
            fighters: [FightFighter::new(f1), FightFighter::new(f2)],
            rng: SmallRng::seed_from_u64(seed),
        };
        do_speed_roll(&mut f.fighters[0], &mut f.rng);
        do_speed_roll(&mut f.fighters[1], &mut f.rng);
        f
    }

    pub fn run<L: FnMut(&dyn Fn() -> String)>(mut self, mut logger: L) -> &'a Fighter {
        while match self.run_tick(&mut logger) {
            Some(x) => return x,
            None => true,
        } {}
        unreachable!()
    }

    fn run_tick<L: FnMut(&dyn Fn() -> String)>(&mut self, logger: &mut L) -> Option<&'a Fighter> {
        logger(&|| {
            format!(
                "Speed rolls are {}: {}, {}: {}",
                self.fighters[0].name(),
                self.fighters[0].speed_roll,
                self.fighters[1].name(),
                self.fighters[1].speed_roll
            )
        });

        let [attacker, defender] = match self.fighters[0]
            .speed_roll
            .cmp(&self.fighters[1].speed_roll)
        {
            std::cmp::Ordering::Less => {
                let [x, y] = &mut self.fighters;
                [x, y]
            }
            std::cmp::Ordering::Equal => {
                logger(&|| "It's a tie!".into());
                let [x, y] = &mut self.fighters;
                if self.rng.gen() {
                    [x, y]
                } else {
                    [y, x]
                }
            }
            std::cmp::Ordering::Greater => {
                let [x, y] = &mut self.fighters;
                [y, x]
            }
        };
        logger(&|| format!("{} is attacking!", attacker.name()));

        let hit_roll = D100.sample(&mut self.rng);
        logger(&|| {
            format!(
                "A roll of {} + {} against {}'s dodge of {}.",
                hit_roll,
                attacker.stat(Accuracy),
                defender.name(),
                defender.stat(Dodge)
            )
        });

        if hit_roll + attacker.stat(Accuracy) >= defender.stat(Dodge) {
            let crit_bonus = if hit_roll >= 99 - (attacker.raw_stat(Accuracy) * 3) {
                logger(&|| "It's a crit!".into());
                2
            } else {
                1
            };

            let damage_roll = D20.sample(&mut self.rng);
            let damage = std::cmp::max(
                1,
                ((damage_roll + attacker.stat(Attack)) * crit_bonus)
                    .saturating_sub(defender.stat(Defense)),
            );
            logger(&|| {
                format!(
                    "A roll of {} + {} against a defense of {} means {} damage.",
                    damage_roll * crit_bonus,
                    attacker.stat(Attack) * crit_bonus,
                    defender.stat(Defense),
                    damage
                )
            });

            defender.current_health = defender.current_health.saturating_sub(damage);

            if defender.current_health == 0 {
                logger(&|| {
                    format!(
                        "{} goes down! The fight is over! {} wins with {} health remaining!",
                        defender.name(),
                        attacker.name(),
                        attacker.current_health
                    )
                });
                return Some(attacker.fighter);
            } else {
                logger(&|| {
                    format!(
                        "{} is now down to {} health.",
                        defender.name(),
                        defender.current_health
                    )
                });
            }
        } else {
            logger(&|| "Miss!".into());
        }

        defender.speed_roll = defender.speed_roll.saturating_sub(attacker.speed_roll);
        do_speed_roll(attacker, &mut self.rng);

        None
    }
}

fn do_speed_roll(fi: &mut FightFighter, rng: &mut SmallRng) {
    fi.speed_roll = std::cmp::max(1, D14.sample(rng).saturating_sub(fi.stat(Speed)));
}
