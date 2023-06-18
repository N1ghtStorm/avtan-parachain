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
use scale_info::prelude::vec::{Vec, vec};

pub struct Dungeon {
    pub dungeon_type: DungeonType,
}

pub enum DungeonType {
    HumanTerritory,
    ElfTerritory,
}

pub enum HumanDungeon {
    OldRuins,
}

pub enum ElfDungeon {
    SpiderDen,
}