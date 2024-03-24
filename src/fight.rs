use fastrand::Rng;

use crate::fight_fighter::FightFighter;
use crate::fighter::*;
use crate::stats::*;

#[derive(Debug)]
pub struct Fight<'a, const TEAM_SIZE: usize> {
    fighters: [[FightFighter<'a>; TEAM_SIZE]; 2],
    rng: Rng,
}

impl<'a, const TEAM_SIZE: usize> Fight<'a, TEAM_SIZE> {
    pub fn new(
        t1: [&'a Fighter; TEAM_SIZE],
        t2: [&'a Fighter; TEAM_SIZE],
        seed: u64,
    ) -> Fight<'a, TEAM_SIZE> {
        let mut this = Self {
            fighters: [t1.map(FightFighter::new), t2.map(FightFighter::new)],
            rng: Rng::with_seed(seed),
        };

        for team in this.fighters.iter_mut() {
            for fighter in team.iter_mut() {
                fighter.do_speed_roll(&mut this.rng);
            }
        }

        this
    }

    pub fn run<L: FnMut(&dyn Fn() -> String)>(mut self, mut logger: L) -> &'a Fighter {
        loop {
            self.run_tick(&mut logger);
            for (a, d) in [(0, 1), (1, 0)] {
                if self.fighters[d].iter().all(|f| f.stat(Stat::Health) == 0) {
                    logger(&|| {
                        format!(
                            "The fight is over! Remaining healths: {}",
                            self.fighters[a]
                                .each_ref()
                                .map(|f| format!("{} - {}", f.name(), f.stat(Stat::Health)))
                                .join(", ")
                        )
                    });

                    // Hack so sim can know which team won
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
                    .map(|team| team
                        .each_ref()
                        .map(|fighter| if fighter.is_alive() {
                            format!("{} - {}", fighter.name(), fighter.speed_roll())
                        } else {
                            String::new()
                        })
                        .join(", "))
                    .join(", ")
            )
        });

        let (attacker, defender) = {
            let mut attacker_data = None;
            for (team_index, fighter_index, speed_roll) in self
                .fighters
                .iter()
                .enumerate()
                .flat_map(|(team_index, team)| {
                    team.iter()
                        .enumerate()
                        .map(move |(fighter_index, fighter)| {
                            (team_index, fighter_index, fighter.speed_roll())
                        })
                })
            {
                match attacker_data {
                    None => attacker_data = Some((team_index, fighter_index, speed_roll)),
                    Some(prev_data) => match prev_data.2.cmp(&speed_roll) {
                        std::cmp::Ordering::Less => {}
                        std::cmp::Ordering::Equal => {
                            if self.rng.bool() {
                                attacker_data = Some((team_index, fighter_index, speed_roll))
                            }
                        }
                        std::cmp::Ordering::Greater => {
                            attacker_data = Some((team_index, fighter_index, speed_roll))
                        }
                    },
                }
            }

            let (attacking_team_index, attacker_index, _) = attacker_data.unwrap();

            let teams = self.fighters.split_at_mut(1);
            let (attacking_team, defending_team) = if attacking_team_index == 0 {
                (&mut teams.0[0], &mut teams.1[0])
            } else {
                (&mut teams.1[0], &mut teams.0[0])
            };

            let defender = loop {
                let d = self.rng.choice(defending_team.iter_mut()).unwrap();
                if d.is_alive() {
                    break d;
                }
            };

            (&mut attacking_team[attacker_index], defender)
        };

        logger(&|| format!("{} is attacking {}!", attacker.name(), defender.name()));

        let hit_roll = self.rng.u16(1..=1000);
        logger(&|| {
            format!(
                "A roll of {} + {} against a dodge of {}.",
                hit_roll,
                attacker.stat(Stat::Accuracy),
                defender.stat(Stat::Dodge),
            )
        });

        if hit_roll + attacker.stat(Stat::Accuracy) >= defender.stat(Stat::Dodge) {
            let crit_bonus = if hit_roll > attacker.crit_chance() {
                logger(&|| "It's a crit!".into());
                2
            } else {
                1
            };

            let damage_roll = self.rng.u16(1..=200);
            let damage = std::cmp::max(
                1,
                ((damage_roll + attacker.stat(Stat::Attack)) * crit_bonus)
                    .saturating_sub(defender.stat(Stat::Defense)),
            );
            logger(&|| {
                format!(
                    "A roll of {} + {} against a defense of {} means {} damage.",
                    damage_roll * crit_bonus,
                    attacker.stat(Stat::Attack) * crit_bonus,
                    defender.stat(Stat::Defense),
                    damage
                )
            });

            defender.take_damage(damage);

            if !defender.is_alive() {
                logger(&|| format!("{} goes down!", defender.name()));
                for i in 1..=10 {
                    if self.rng.u16(1..=50) + defender.stat(Stat::Conviction)
                        > 50 + defender.knockdown_count()
                        && defender.knockdown_count() < 1
                    {
                        defender.get_back_up();
                        logger(&|| {
                            format!(
                                "{} gets back up! They now have {} health.",
                                defender.name(),
                                defender.stat(Stat::Health)
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
                        defender.stat(Stat::Health)
                    )
                });
            }
        } else {
            logger(&|| "Miss!".into());
        }

        let attacker_speed_roll = attacker.speed_roll();
        attacker.do_speed_roll(&mut self.rng);

        self.fighters
            .iter_mut()
            .flatten()
            .for_each(|f| f.end_turn(attacker_speed_roll));
    }
}
