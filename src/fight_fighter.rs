use crate::{Fighter, SignedStatValue, Stat, StatValue};
use enum_map::EnumMap;
use fastrand::Rng;

#[derive(Debug)]
pub(crate) struct FightFighter<'a> {
    pub fighter: &'a Fighter,
    mods: EnumMap<Stat, SignedStatValue>,
    speed_roll: StatValue,
    knockdown_count: StatValue,
}

impl<'a> FightFighter<'a> {
    pub(crate) fn new(fighter: &'a Fighter) -> Self {
        Self {
            fighter,
            mods: EnumMap::default(),
            speed_roll: 0,
            knockdown_count: 0,
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.fighter.name()
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.stat(Stat::Health) > 0
    }

    pub(crate) fn speed_roll(&self) -> StatValue {
        self.speed_roll
    }

    pub(crate) fn take_damage(&mut self, damage: StatValue) {
        self.mods[Stat::Health] = self.mods[Stat::Health].saturating_sub_unsigned(damage);
        if !self.is_alive() {
            self.knockdown_count += 1;
        }
    }

    pub(crate) fn knockdown_count(&self) -> StatValue {
        self.knockdown_count
    }

    pub(crate) fn stat(&self, stat: Stat) -> StatValue {
        stat.effective_value(self.fighter.raw_stat(stat))
            .saturating_add_signed(self.mods[stat])
    }

    pub(crate) fn crit_chance(&self) -> StatValue {
        1000 - (self.fighter.raw_stat(Stat::Accuracy) * 20)
    }

    pub(crate) fn do_speed_roll(&mut self, rng: &mut Rng) {
        self.speed_roll +=
            std::cmp::max(1, rng.u16(1..=140).saturating_sub(self.stat(Stat::Speed)));
    }

    pub(crate) fn end_turn(&mut self, attacker_speed_roll: StatValue) {
        self.speed_roll = self.speed_roll.saturating_sub(attacker_speed_roll);
    }

    pub(crate) fn get_back_up(&mut self) {
        self.mods[Stat::Attack] =
            self.mods[Stat::Attack].saturating_add_unsigned(self.stat(Stat::Conviction) * 4);
        self.mods[Stat::Defense] =
            self.mods[Stat::Defense].saturating_add_unsigned(self.stat(Stat::Conviction) * 4);
        self.mods[Stat::Speed] =
            self.mods[Stat::Speed].saturating_add_unsigned(self.stat(Stat::Conviction) * 4);
        self.mods[Stat::Accuracy] =
            self.mods[Stat::Accuracy].saturating_add_unsigned(self.stat(Stat::Conviction) * 10);
        self.mods[Stat::Dodge] =
            self.mods[Stat::Dodge].saturating_add_unsigned(self.stat(Stat::Conviction) * 10);

        self.mods[Stat::Health] = ((200 * self.stat(Stat::Conviction)) as SignedStatValue)
            .saturating_sub_unsigned(
                Stat::Health.effective_value(self.fighter.raw_stat(Stat::Health)),
            );
    }
}
