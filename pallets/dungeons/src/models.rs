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

pub struct PlayerData {
    pub heroes: Vec<Hero>,
}

pub struct Hero {
    pub level: u8,
    pub current_exp: u32,
}

pub enum HeroRace {
    Human(Human),
    Elf(Elf),
}

pub enum Human {
    Warrior,
    Archer,
    Mage,
}

pub enum Elf {
    Archer,
    Druid,
}
