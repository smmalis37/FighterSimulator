use rand::distributions::Uniform;
use rand::prelude::*;
use static_init::dynamic;

use crate::fighter::*;
use crate::stats::Stat::*;
use crate::stats::*;

#[dynamic]
//hp regain and get up chance
static REGEN: Uniform<StatValue> = Uniform::new_inclusive(1, 60);
#[dynamic]
static D10: Uniform<StatValue> = Uniform::new_inclusive(1, 100);
#[dynamic]
static D50: Uniform<StatValue> = Uniform::new_inclusive(1, 50);
#[dynamic]
//speed
static D14: Uniform<StatValue> = Uniform::new_inclusive(1, 140);
#[dynamic]
// damage
static D20: Uniform<StatValue> = Uniform::new_inclusive(1, 200);
#[dynamic]
//hit chance
static D100: Uniform<StatValue> = Uniform::new_inclusive(1, 1000);

#[derive(Debug)]
struct FightFighter<'a> {
    fighter: &'a Fighter,
    current_health: StatValue,
    speed_roll: StatValue,
    knockdown_count: StatValue,
    current_attack: StatValue,
    current_defense: StatValue,
    current_speed: StatValue,
    current_accuracy: StatValue,
    current_dodge: StatValue,
}

impl<'a> FightFighter<'a> {
    fn new(fighter: &'a Fighter) -> Self {
        Self {
            fighter,
            current_health: fighter.stat(Health),
            speed_roll: 0,
            knockdown_count: 0,
            current_attack: fighter.stat(Attack),
            current_defense: fighter.stat(Defense),
            current_speed: fighter.stat(Speed),
            current_accuracy: fighter.stat(Accuracy),
            current_dodge: fighter.stat(Dodge),
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
                if self.fighters[d].iter().all(|f| f.current_health <= 0) {
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
                        .map(|f| if f.current_health > 0 {
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
                if self.fighters[f.0][f.1].current_health <= 0 {
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
                attacker.current_accuracy,
                defender.name(),
                defender.current_dodge,
            )
        });

        if hit_roll + attacker.current_accuracy >= defender.current_dodge {
            let crit_bonus = if hit_roll >= 999 - (attacker.raw_stat(Accuracy) * 20) {
                logger(&|| "It's a crit!".into());
                2
            } else {
                1
            };

            let damage_roll = D20.sample(&mut self.rng);
            let damage = std::cmp::max(
                10,
                ((damage_roll + attacker.current_attack) * crit_bonus)
                    .saturating_sub(defender.current_defense),
            );
            logger(&|| {
                format!(
                    "A roll of {} + {} against a defense of {} means {} damage.",
                    damage_roll * crit_bonus,
                    attacker.current_attack * crit_bonus,
                    defender.current_defense,
                    damage
                )
            });

            defender.current_health = defender.current_health.saturating_sub(damage);

            if defender.current_health <= 0 {
                defender.current_health = 0;
                logger(&|| format!("{} goes down!", defender.name()));
                for i in 1..=10 {
                    if D50.sample(&mut self.rng) + defender.stat(Conviction)
                        > 50 + defender.knockdown_count
                        && defender.knockdown_count != 1
                    {
                        defender.knockdown_count += 1;
                        defender.current_attack += defender.stat(Conviction) * 4;
                        defender.current_defense += defender.stat(Conviction) * 4;
                        defender.current_speed += defender.stat(Conviction) * 4;
                        defender.current_accuracy += defender.stat(Conviction) * 10;
                        defender.current_dodge += defender.stat(Conviction) * 10;
                        defender.current_health = 200 * defender.stat(Conviction);
                        logger(&|| {
                            format!(
                                "{} gets back up! They now have {} health.",
                                defender.name(),
                                defender.current_health
                            )
                        });
                        break;
                    } else {
                        logger(&|| format!("{}!", i));
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
    fi.speed_roll = std::cmp::max(1, D14.sample(rng).saturating_sub(fi.current_speed));
}
