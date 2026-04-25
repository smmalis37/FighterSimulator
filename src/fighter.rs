use crate::stats::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Fighter {
    name: String,
    attack: AttackDie,
    defence: DefenceDie,

    cut_man: bool,
    strength_training: bool,
    impenetrable_guard: bool,
    champions_resilience: bool,
}

impl Fighter {
    pub fn new(
        name: String,
        attack: AttackDie,
        defence: DefenceDie,
        cut_man: bool,
        strength_training: bool,
        impenetrable_guard: bool,
        champions_resilience: bool,
    ) -> Fighter {
        Fighter {
            name,
            attack,
            defence,
            cut_man,
            strength_training,
            impenetrable_guard,
            champions_resilience,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attack(&self) -> AttackDie {
        self.attack
    }

    pub fn defence(&self) -> DefenceDie {
        self.defence
    }

    pub fn cut_man(&self) -> bool {
        self.cut_man
    }

    pub fn strength_training(&self) -> bool {
        self.strength_training
    }

    pub fn impenetrable_guard(&self) -> bool {
        self.impenetrable_guard
    }

    pub fn champions_resilience(&self) -> bool {
        self.champions_resilience
    }
}
