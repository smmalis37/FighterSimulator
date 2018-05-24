pub type StatValue = u32;

#[derive(Debug, EnumMap, Copy, Clone)]
pub enum Stat {
    Attack,
    Speed,
    Endurance,
}

impl Stat {
    pub fn costs(&self) -> &'static [StatValue] {
        match *self {
            Stat::Attack => &[
                0, 5, 10, 15, 20, 25, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 450,
                160, 180,
            ],
            Stat::Speed => &[0, 10, 20, 35, 50, 70, 90, 110, 135, 160, 190],
            Stat::Endurance => &[0, 20, 40, 80, 100, 150],
        }
    }

    pub fn zero_allowed(&self) -> bool {
        match *self {
            Stat::Attack => false,
            Stat::Speed => false,
            Stat::Endurance => true,
        }
    }
}

pub const BASE_HEALTH: StatValue = 50;
pub const HEALTH_PER_POINT: StatValue = 3;
pub const TOTAL_POINTS: StatValue = 200;
