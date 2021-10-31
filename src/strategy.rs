use crate::common::BetContext;

pub enum Bet {
    Hit(u64),
}

pub trait Strategy: Sync + Send {
    fn name(&self) -> &'static str;

    fn bet(&self, context: &BetContext) -> Bet;
}

pub struct DummyStrat;

impl Strategy for DummyStrat {
    fn name(&self) -> &'static str {
        "Dummy"
    }

    fn bet(&self, context: &BetContext) -> Bet {
        Bet::Hit((context.total_money as f64 * 0.1) as u64)
    }
}

pub mod martingale {
    use super::{Bet, Strategy};

    pub struct MartinGaleStrat {
        strating_bet: u64,
    }

    impl MartinGaleStrat {
        pub const fn new() -> Self {
            Self { strating_bet: 100 }
        }
    }

    impl Strategy for MartinGaleStrat {
        fn name(&self) -> &'static str {
            "MartinGale Strat"
        }

        fn bet(&self, context: &crate::common::BetContext) -> Bet {
            let hit = self.strating_bet * 2u64.pow(context.consec_bet_loses.len() as u32);
            Bet::Hit(hit)
        }
    }
}
