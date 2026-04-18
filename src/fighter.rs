use crate::stats::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Fighter {
    name: String,
    attack: AttackDie,
    defence: DefenceDie,
}

impl Fighter {
    pub fn new(name: String, attack: AttackDie, defence: DefenceDie) -> Fighter {
        Fighter {
            name,
            attack,
            defence,
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
}
