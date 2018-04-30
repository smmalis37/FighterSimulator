use arrayvec::ArrayVec;

use fight::Round;
use fighter::*;
use stats::*;

#[derive(Debug)]
pub struct FullReport<'a> {
    pub new_round: Option<Round>,
    pub attacks: [Option<FullAttackReport<'a>>; 2],
    pub remaining_healths: [Option<StatValue>; 2],
    pub winner: Option<&'a Fighter>,
}

#[derive(Debug)]
pub struct FullAttackReport<'a> {
    pub attacker: &'a Fighter,
    pub defender: &'a Fighter,
    pub first_rolls: AttackRolls,
    pub second_rolls: AttackRolls,
    pub damage: StatValue,
}

type AttackRolls = ArrayVec<[StatValue; 20]>;

pub(crate) struct WinnerOnlyReport<'a> {
    pub winner: Option<&'a Fighter>,
}

pub(crate) struct MinAttackReport<'a> {
    attacker: &'a Fighter,
    damage: StatValue,
}

pub(crate) trait Report<'a> {
    type AttackReport: AttackReport<'a>;

    fn new() -> Self;
    fn get_winner(&self) -> Option<&'a Fighter>;

    fn set_winner(&mut self, winner: &'a Fighter);
    fn set_new_round(&mut self, new_round: Round);
    fn set_attack(
        &mut self,
        attack_index: usize,
        attack: Self::AttackReport,
        current_health: StatValue,
    );
}

pub(crate) trait AttackReport<'a> {
    fn new(
        attacker: &'a Fighter,
        defender: &'a Fighter,
        first_rolls: AttackRolls,
        second_rolls: AttackRolls,
        damage: StatValue,
    ) -> Self;

    fn get_damage(&self) -> StatValue;
    fn get_attacker(&self) -> &'a Fighter;
}

impl<'a> Report<'a> for FullReport<'a> {
    type AttackReport = FullAttackReport<'a>;

    fn new() -> Self {
        FullReport {
            new_round: None,
            attacks: [None, None],
            remaining_healths: [None, None],
            winner: None,
        }
    }

    fn get_winner(&self) -> Option<&'a Fighter> {
        self.winner
    }

    fn set_winner(&mut self, winner: &'a Fighter) {
        self.winner = Some(winner);
    }

    fn set_new_round(&mut self, new_round: Round) {
        self.new_round = Some(new_round);
    }

    fn set_attack(
        &mut self,
        attack_index: usize,
        attack: Self::AttackReport,
        current_health: StatValue,
    ) {
        self.attacks[attack_index] = Some(attack);
        self.remaining_healths[attack_index] = Some(current_health);
    }
}

impl<'a> Report<'a> for WinnerOnlyReport<'a> {
    type AttackReport = MinAttackReport<'a>;

    fn new() -> Self {
        WinnerOnlyReport { winner: None }
    }

    fn get_winner(&self) -> Option<&'a Fighter> {
        self.winner
    }

    fn set_winner(&mut self, winner: &'a Fighter) {
        self.winner = Some(winner);
    }

    fn set_new_round(&mut self, _new_round: Round) {}
    fn set_attack(
        &mut self,
        _attack_index: usize,
        _attack: Self::AttackReport,
        _current_health: StatValue,
    ) {
    }
}

impl<'a> AttackReport<'a> for FullAttackReport<'a> {
    fn new(
        attacker: &'a Fighter,
        defender: &'a Fighter,
        first_rolls: AttackRolls,
        second_rolls: AttackRolls,
        damage: StatValue,
    ) -> Self {
        FullAttackReport {
            attacker,
            defender,
            first_rolls,
            second_rolls,
            damage,
        }
    }

    fn get_damage(&self) -> StatValue {
        self.damage
    }

    fn get_attacker(&self) -> &'a Fighter {
        self.attacker
    }
}

impl<'a> AttackReport<'a> for MinAttackReport<'a> {
    fn new(
        attacker: &'a Fighter,
        _defender: &'a Fighter,
        _first_rolls: AttackRolls,
        _second_rolls: AttackRolls,
        damage: StatValue,
    ) -> Self {
        MinAttackReport { attacker, damage }
    }

    fn get_damage(&self) -> StatValue {
        self.damage
    }

    fn get_attacker(&self) -> &'a Fighter {
        self.attacker
    }
}
