use enum_map::Enum;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

#[derive(Enum, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AttackDie {
    Boxer,
    DirtyFighter,
    Swarmer,
    Brawler,
    Reckless,
    Hearty,
    Slugger,
    Jobber,
}

#[derive(Copy, Clone)]
pub enum AttackResult {
    Damage(i16),
    Combo,
    Special(i16),
}

impl AttackDie {
    pub fn roll<R: rand::Rng>(&self, rng: &mut R) -> AttackResult {
        *[
            AttackResult::Damage(1),
            AttackResult::Damage(1),
            AttackResult::Damage(2),
            AttackResult::Damage(2),
            AttackResult::Combo,
            AttackResult::Special(3),
        ]
        .choose(rng)
        .unwrap()
    }
}

#[derive(Enum, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum DefenceDie {
    PeekABoo,
    PhillyShell,
    CrossArms,
    LoosyGoosy,
    PunchingBag,
}

#[derive(Copy, Clone)]
pub enum DefenceResult {
    Open,
    GuardUp(i16, i16),
    GuardDown(i16, i16),
    Dodge,
    Counter,
}

impl DefenceDie {
    pub fn roll<R: rand::Rng>(&self, rng: &mut R) -> DefenceResult {
        *[
            DefenceResult::Open,
            DefenceResult::GuardUp(-1, 1),
            DefenceResult::GuardDown(-1, 1),
            DefenceResult::GuardUp(-1, 1),
            DefenceResult::GuardDown(-2, 2),
            DefenceResult::GuardUp(-2, 2),
            DefenceResult::Dodge,
            DefenceResult::Counter,
        ]
        .choose(rng)
        .unwrap()
    }
}

#[derive(Enum, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Target {
    Head,
    Body,
}
