//                                                   ~-.
//                                                   ,,,;            ~-.~-.~-
//                                               (.../           ~-.~-.~-.~-.~-.
//                                           < } O~`, ,        ~-.~-.~-.~-.~-.~-.
//                                               (/    T ,     ~-.~-.~-.~-.~-.~-.~-.
//                                                   ;    T     ~-.~-.~-.~-.~-.~-.~-.
//                                                 ;   {_.~-.~-.~-.~-.~-.~-.~
//                                               ;:  .-~`    ~-.~-.~-.~-.~-.
//                                               ;.: :'    ._   ~-.~-.~-.~-.~-
//                                               ;::`-.    '-._  ~-.~-.~-.~-
//                                               ;::. `-.    '-,~-.~-.~-.
//                                                   ';::::.`''-.-'
//                                                   ';::;;:,:'
//                                                       '||T
//                                                     __   _
//                                                       / |

use codec::{Decode, Encode};
use sp_core::RuntimeDebug;
use sp_runtime::scale_info::TypeInfo;

pub const INIT_GOLD: u128 = 1000;

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerData {
    heroes: Vec<Hero>,
    current_gold: u128,
}

impl PlayerData {
    pub fn new_elven() -> Self {
        Self {
            heroes: vec![
                Hero::new(HeroType::Elf(Elf::Archer)),
                Hero::new(HeroType::Elf(Elf::Druid)),
            ],
            current_gold: INIT_GOLD,
        }
    }

    pub fn new_human() -> Self {
        Self {
            heroes: vec![
                Hero::new(HeroType::Human(Human::Warrior)),
                Hero::new(HeroType::Human(Human::Mage)),
            ],
            current_gold: INIT_GOLD,
        }
    }

    pub fn new_with() -> Self {
        Self { heroes: Vec::new(), current_gold: 0 }
    }

    pub fn heroes(&self) -> &Vec<Hero> {
        &self.heroes
    }

    pub fn current_gold(&self) -> u128 {
        self.current_gold
    }

    pub fn add_hero(&mut self, hero: Hero) {
        self.heroes.push(hero)
    }
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Hero {
    pub level: u8,
    pub current_exp: u32,
    pub hero_type: HeroType,
}

impl Hero {
    pub fn new(hero_type: HeroType) -> Self {
        Self { level: 0, current_exp: 0, hero_type }
    }
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum HeroType {
    Human(Human),
    Elf(Elf),
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum Human {
    Warrior,
    Archer,
    Mage,
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum Elf {
    Archer,
    Druid,
}
