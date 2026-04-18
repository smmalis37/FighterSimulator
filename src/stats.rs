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
    Dirty,
}

impl AttackDie {
    pub fn roll<R: rand::Rng>(&self, rng: &mut R) -> AttackResult {
        *match self {
            AttackDie::Boxer
            | AttackDie::Swarmer
            | AttackDie::Brawler
            | AttackDie::Hearty
            | AttackDie::Slugger
            | AttackDie::Jobber => &[
                AttackResult::Damage(1),
                AttackResult::Damage(1),
                AttackResult::Damage(2),
                AttackResult::Damage(2),
                AttackResult::Combo,
                AttackResult::Special(3),
            ],
            AttackDie::DirtyFighter => &[
                AttackResult::Damage(1),
                AttackResult::Damage(1),
                AttackResult::Damage(2),
                AttackResult::Dirty,
                AttackResult::Combo,
                AttackResult::Special(3),
            ],
            AttackDie::Reckless => &[
                AttackResult::Damage(1),
                AttackResult::Damage(2),
                AttackResult::Damage(2),
                AttackResult::Damage(3),
                AttackResult::Combo,
                AttackResult::Special(4),
            ],
        }
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
    Counter(i16),
}

impl DefenceDie {
    pub fn roll<R: rand::Rng>(&self, rng: &mut R) -> DefenceResult {
        *match self {
            DefenceDie::PeekABoo => &[
                DefenceResult::Open,
                DefenceResult::GuardUp(-1, 1),
                DefenceResult::GuardDown(-1, 1),
                DefenceResult::GuardUp(-1, 1),
                DefenceResult::GuardDown(-2, 2),
                DefenceResult::GuardUp(-2, 2),
                DefenceResult::Dodge,
                DefenceResult::Counter(1),
            ],
            DefenceDie::PhillyShell => &[
                DefenceResult::Open,
                DefenceResult::GuardDown(-1, 1),
                DefenceResult::GuardUp(-1, 2),
                DefenceResult::GuardDown(-1, 1),
                DefenceResult::GuardUp(-2, 2),
                DefenceResult::GuardDown(-2, 2),
                DefenceResult::Dodge,
                DefenceResult::Counter(2),
            ],
            DefenceDie::CrossArms => &[
                DefenceResult::Open,
                DefenceResult::GuardUp(-1, 2),
                DefenceResult::GuardDown(-2, 1),
                DefenceResult::GuardUp(-1, 2),
                DefenceResult::GuardDown(-2, 1),
                DefenceResult::GuardUp(-2, 2),
                DefenceResult::Dodge,
                DefenceResult::Counter(1),
            ],
            DefenceDie::LoosyGoosy => &[
                DefenceResult::Open,
                DefenceResult::GuardUp(-2, 2),
                DefenceResult::GuardDown(-2, 2),
                DefenceResult::GuardUp(-2, 2),
                DefenceResult::GuardDown(-3, 3),
                DefenceResult::GuardUp(-3, 3),
                DefenceResult::Dodge,
                DefenceResult::Counter(1),
            ],
            DefenceDie::PunchingBag => &[
                DefenceResult::Open,
                DefenceResult::GuardUp(-1, 2),
                DefenceResult::GuardDown(-1, 2),
                DefenceResult::GuardUp(-1, 2),
                DefenceResult::GuardDown(-2, 2),
                DefenceResult::GuardUp(-2, 2),
                DefenceResult::Dodge,
                DefenceResult::Counter(1),
            ],
        }
        .choose(rng)
        .unwrap()
    }
}

#[derive(Enum, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Target {
    Head,
    Body,
}
