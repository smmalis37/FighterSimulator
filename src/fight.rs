use rand::distributions::Uniform;
use rand::prelude::*;
use static_init::dynamic;

use crate::fighter::*;
use crate::stats::Stat::*;
use crate::stats::*;

#[dynamic]
static D6: Uniform<StatValue> = Uniform::new_inclusive(1, 6);
#[dynamic]
static D10: Uniform<StatValue> = Uniform::new_inclusive(1, 10);
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
    knockdown_count: StatValue,
}

impl<'a> FightFighter<'a> {
    fn new(fighter: &'a Fighter) -> Self {
        Self {
            fighter,
            current_health: fighter.stat(Health),
            speed_roll: 0,
            knockdown_count: 0,
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
pub struct Fight<'a, const TEAM_SIZE: usize> {
    fighters: [[FightFighter<'a>; TEAM_SIZE]; 2],
    rng: SmallRng,
}

impl<'a, const TEAM_SIZE: usize> Fight<'a, TEAM_SIZE> {
    pub fn new(
        t1: [&'a Fighter; TEAM_SIZE],
        t2: [&'a Fighter; TEAM_SIZE],
        seed: u64,
    ) -> Fight<'a, TEAM_SIZE> {
        let mut f = Self {
            fighters: [
                t1.map(|f| FightFighter::new(f)),
                t2.map(|f| FightFighter::new(f)),
            ],
            rng: SmallRng::seed_from_u64(seed),
        };

        for team in f.fighters.iter_mut() {
            for fighter in team.iter_mut() {
                do_speed_roll(fighter, &mut f.rng);
            }
        }
        f
    }

    pub fn run<L: FnMut(&dyn Fn() -> String)>(mut self, mut logger: L) -> &'a Fighter {
        loop {
            self.run_tick(&mut logger);
            for (a, d) in [(0, 1), (1, 0)] {
                if self.fighters[d].iter().all(|f| f.current_health == 0) {
                    logger(&|| {
                        format!(
                            "The fight is over! Remaining healths: {}",
                            self.fighters[a]
                                .each_ref()
                                .map(|f| format!("{} - {}", f.name(), f.current_health))
                                .join(", ")
                        )
                    });

                    return self.fighters[a][0].fighter;
                }
            }
        }
    }

    fn run_tick<L: FnMut(&dyn Fn() -> String)>(&mut self, logger: &mut L) {
        logger(&|| {
            format!(
                "Speed rolls are: {}",
                self.fighters
                    .each_ref()
                    .map(|t| t
                        .each_ref()
                        .map(|f| if f.current_health != 0 {
                            format!("{} - {}", f.name(), f.speed_roll)
                        } else {
                            String::new()
                        })
                        .join(", "))
                    .join(", ")
            )
        });

        let (attacker, defender) = {
            let (mut a, mut def_team) = (None, None);
            for (f, dt) in (0..TEAM_SIZE)
                .map(|f| ((0, f), 1))
                .chain((0..TEAM_SIZE).map(|f| ((1, f), 0)))
            {
                if self.fighters[f.0][f.1].current_health == 0 {
                    continue;
                }

                if a.is_none() {
                    a = Some(f);
                    def_team = Some(dt);
                } else {
                    let au = a.unwrap();
                    match self.fighters[au.0][au.1]
                        .speed_roll
                        .cmp(&self.fighters[f.0][f.1].speed_roll)
                    {
                        std::cmp::Ordering::Less => {}
                        std::cmp::Ordering::Equal => {
                            if self.rng.gen() {
                                a = Some(f);
                                def_team = Some(dt);
                            }
                        }
                        std::cmp::Ordering::Greater => {
                            a = Some(f);
                            def_team = Some(dt);
                        }
                    };
                }
            }

            let teams = self.fighters.split_at_mut(1);
            let (a_team, d_team) = if def_team.unwrap() == 1 {
                (teams.0, teams.1)
            } else {
                (teams.1, teams.0)
            };

            let d = loop {
                let d = d_team[0].choose_mut(&mut self.rng).unwrap();
                if d.current_health > 0 {
                    break d;
                }
            };

            (&mut a_team[0][a.unwrap().1], d)
        };

        logger(&|| format!("{} is attacking {}!", attacker.name(), defender.name()));

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
                logger(&|| format!("{} goes down!", defender.name()));
                defender.knockdown_count += 1;
                for i in 1..=10 {
                    logger(&|| format!("{}!", i));
                    if i != 10 && D10.sample(&mut self.rng) >= 5 + defender.knockdown_count {
                        defender.current_health = 10 * D6.sample(&mut self.rng);
                        logger(&|| {
                            format!(
                                "{} gets back up! They now have {} health.",
                                defender.name(),
                                defender.current_health
                            )
                        });
                        break;
                    }
                }
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

        let attack_roll = attacker.speed_roll;
        do_speed_roll(attacker, &mut self.rng);
        attacker.speed_roll += attack_roll;

        self.fighters.iter_mut().for_each(|t| {
            t.iter_mut().for_each(|f| {
                f.speed_roll = f.speed_roll.saturating_sub(attack_roll);
            })
        });
    }
}

fn do_speed_roll(fi: &mut FightFighter, rng: &mut SmallRng) {
    fi.speed_roll = std::cmp::max(1, D14.sample(rng).saturating_sub(fi.stat(Speed)));
}
