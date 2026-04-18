use crate::fighter::*;
use crate::stats::*;
use enum_map::EnumMap;
use rand::distr::Bernoulli;
use rand::distr::Uniform;
use rand::prelude::*;
use static_init::dynamic;

#[dynamic]
static D6: Uniform<i16> = Uniform::new_inclusive(1, 6).unwrap();
#[dynamic]
static DIRTY_FIGHTING: Bernoulli = Bernoulli::new(0.05).unwrap();

struct FightFighter<'a> {
    fighter: &'a Fighter,
    current_health: i16,
    injuries: EnumMap<Target, i16>,
    warnings: u8,
    knockdowns: i16,
}

impl<'a> FightFighter<'a> {
    fn new(fighter: &'a Fighter) -> Self {
        Self {
            fighter,
            current_health: 60,
            injuries: EnumMap::default(),
            warnings: 0,
            knockdowns: 0,
        }
    }
}

impl<'a> std::ops::Deref for FightFighter<'a> {
    type Target = Fighter;

    fn deref(&self) -> &Self::Target {
        self.fighter
    }
}

pub struct Fight<'a> {
    f1: FightFighter<'a>,
    f2: FightFighter<'a>,
    rng: SmallRng,
}

impl<'a> Fight<'a> {
    pub fn new(f1: &'a Fighter, f2: &'a Fighter, seed: u64) -> Fight<'a> {
        Self {
            f1: FightFighter::new(f1),
            f2: FightFighter::new(f2),
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn run<L: FnMut(&dyn Fn() -> String)>(mut self, mut logger: L) -> &'a Fighter {
        for round in 0..5 {
            logger(&|| format!("Round {}", round + 1));
            for exchange in 0..6 {
                if let Some(fighter) = self.run_exchange(&mut logger, exchange) {
                    return fighter;
                }
            }
            logger(&|| format!("End of round {}", round + 1));
            if round < 4 {
                self.do_healing(&mut logger);
            }
        }

        match self.f1.current_health.cmp(&self.f2.current_health) {
            std::cmp::Ordering::Greater => self.f1.fighter,
            std::cmp::Ordering::Less => self.f2.fighter,
            std::cmp::Ordering::Equal => {
                logger(&|| "It's a draw! Deciding by coin flip...".to_string());
                if self.rng.random() {
                    self.f1.fighter
                } else {
                    self.f2.fighter
                }
            }
        }
    }

    fn run_exchange<L: FnMut(&dyn Fn() -> String)>(
        &mut self,
        logger: &mut L,
        exchange: u8,
    ) -> Option<&'a Fighter> {
        let (attacker, defender) = if self.rng.random() {
            (&mut self.f1, &mut self.f2)
        } else {
            (&mut self.f2, &mut self.f1)
        };
        logger(&|| {
            format!(
                "Exchange {}, {} is attacking!",
                exchange + 1,
                attacker.name()
            )
        });

        let mut exchange_damage = 0;
        let mut combo_count = None;
        let mut dirty_hit = false;
        loop {
            if combo_count.is_none() && self.rng.sample(*DIRTY_FIGHTING) {
                logger(&|| format!("{} is fighting dirty!", attacker.name()));

                let dead = Self::do_damage(logger, defender, 1);

                let dirty_roll = self.rng.sample(*D6);
                if dirty_roll >= 5 {
                    logger(&|| {
                        format!("But the ref saw them! {} gets a warning.", attacker.name())
                    });
                    attacker.warnings += 1;
                    if attacker.warnings >= 3 {
                        logger(&|| format!("{} is disqualified!", attacker.name()));
                        return Some(defender.fighter);
                    }
                    return None;
                } else {
                    logger(&|| "The ref missed it!".to_string());
                    dirty_hit = true;
                }

                if dead {
                    return Some(attacker.fighter);
                }
            } else {
                let mut damage = 0;
                let attack = attacker.attack().roll(&mut self.rng);
                match attack {
                    AttackResult::Damage(d) | AttackResult::Special(d) => damage += d,
                    AttackResult::Combo => {
                        *combo_count.get_or_insert(1) += 1;
                        continue;
                    }
                }
                let target = if self.rng.random() {
                    Target::Head
                } else {
                    Target::Body
                };
                if let Some(combo) = combo_count {
                    logger(&|| {
                        format!(
                            "A {} damage, {} hit combo to the {:?}!",
                            damage, combo, target
                        )
                    });
                } else {
                    logger(&|| format!("A {} damage punch to the {:?}!", damage, target));
                }
                let hit_count = combo_count.take().unwrap_or(1);

                let (defense, def) = if dirty_hit {
                    dirty_hit = false;
                    logger(&|| format!("{}'s guard is down from the dirty hit!", defender.name()));
                    (DefenceResult::Open, 3)
                } else {
                    let defense = defender.defence().roll(&mut self.rng);
                    let def = match defense {
                        DefenceResult::Open => {
                            logger(&|| format!("{} is wide open!", defender.name()));
                            3
                        }
                        DefenceResult::GuardUp(block, miss) => match target {
                            Target::Head => {
                                logger(&|| format!("{} blocks the attack!", defender.name()));
                                block
                            }
                            Target::Body => {
                                logger(&|| format!("{} mistakenly guards high!", defender.name()));
                                miss
                            }
                        },
                        DefenceResult::GuardDown(block, miss) => match target {
                            Target::Head => {
                                logger(&|| format!("{} mistakenly guards low!", defender.name()));
                                miss
                            }
                            Target::Body => {
                                logger(&|| format!("{} blocks the attack!", defender.name()));
                                block
                            }
                        },
                        DefenceResult::Dodge => {
                            logger(&|| format!("{} dodges the attack!", defender.name()));
                            return None;
                        }
                        DefenceResult::Counter => {
                            if self.rng.random() {
                                logger(&|| format!("{} counters the attack!", defender.name()));
                                if Self::do_damage(logger, attacker, 2) {
                                    return Some(defender.fighter);
                                }
                                return None;
                            } else {
                                logger(&|| {
                                    format!("{} tries to counter but whiffs!", defender.name())
                                });
                                2
                            }
                        }
                    };
                    (defense, def)
                };

                if damage + def <= 0 {
                    logger(&|| {
                        format!(
                            "A defense of {} means {} takes no damage.",
                            def,
                            defender.name()
                        )
                    });
                    return None;
                }

                if matches!(attack, AttackResult::Special(_))
                    && matches!(defense, DefenceResult::Open)
                {
                    logger(&|| {
                        format!(
                            "{} takes a direct hit from a special attack! They look hurt.",
                            defender.name()
                        )
                    });
                    defender.injuries[target] += hit_count;
                }

                let total_damage = (damage + def + defender.injuries[target]) * hit_count;
                if Self::do_damage(logger, defender, total_damage) {
                    return Some(attacker.fighter);
                }

                exchange_damage += total_damage;
                if exchange_damage >= 10 {
                    logger(&|| format!("{} goes down!", defender.name()));
                    defender.knockdowns += 1;
                    let down_roll = self.rng.sample(*D6) + self.rng.sample(*D6);
                    if down_roll >= 3 + (defender.knockdowns * 2) {
                        logger(&|| format!("{} gets back up!", defender.name()));
                        return None;
                    } else {
                        logger(&|| format!("{} is counted out!", defender.name()));
                        return Some(attacker.fighter);
                    }
                } else if exchange_damage >= 7 {
                    logger(&|| format!("{} looks dazed from the exchange!", defender.name()));
                    let clinch_roll = self.rng.sample(*D6);
                    if clinch_roll >= 5 {
                        logger(&|| format!("{} clinches to recover!", defender.name()));
                        return None;
                    }
                }
            }
        }
    }

    fn do_damage<L: FnMut(&dyn Fn() -> String)>(
        logger: &mut L,
        defender: &mut FightFighter,
        damage: i16,
    ) -> bool {
        defender.current_health -= damage;
        logger(&|| {
            format!(
                "{} takes {} damage! Current health: {}",
                defender.name(),
                damage,
                defender.current_health
            )
        });

        if defender.current_health <= 0 {
            logger(&|| "TKO!".to_string());
            return true;
        }
        false
    }

    fn do_healing<L: FnMut(&dyn Fn() -> String)>(&mut self, logger: &mut L) {
        for fighter in [&mut self.f1, &mut self.f2] {
            let heal_roll = self.rng.sample(*D6);
            if heal_roll == 6 && fighter.injuries[Target::Head] > 0 {
                fighter.injuries[Target::Head] -= 1;
                logger(&|| format!("{} heals a head injury!", fighter.name(),));
            } else if heal_roll >= 6 && fighter.injuries[Target::Body] > 0 {
                fighter.injuries[Target::Body] -= 1;
                logger(&|| format!("{} heals a body injury!", fighter.name(),));
            } else {
                fighter.current_health += heal_roll;
                logger(&|| format!("{} heals {} health!", fighter.name(), heal_roll,));
            }
            logger(&|| {
                format!(
                    "{}'s current health: {}, current injuries: {:?}",
                    fighter.name(),
                    fighter.current_health,
                    fighter.injuries
                )
            });
        }
    }
}
