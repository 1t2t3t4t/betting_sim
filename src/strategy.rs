use crate::common::BetContext;

pub enum Bet {
    Hit(u64),
}

pub trait Strategy: Sync + Send {
    fn name(&self) -> &'static str;

    fn bet(&self, context: &BetContext) -> Bet;
}

pub struct TenPercentStrat;

impl Strategy for TenPercentStrat {
    fn name(&self) -> &'static str {
        "TenPercentStrat"
    }

    fn bet(&self, context: &BetContext) -> Bet {
        Bet::Hit((context.total_money as f64 * 0.1) as u64)
    }
}

pub mod martingale {
    use super::{Bet, Strategy};

    pub struct MartinGaleStrat {
        base_bet: u64,
    }

    impl MartinGaleStrat {
        pub const fn new() -> Self {
            Self { base_bet: 100 }
        }
    }

    impl Strategy for MartinGaleStrat {
        fn name(&self) -> &'static str {
            "MartinGaleStrat"
        }

        fn bet(&self, context: &crate::common::BetContext) -> Bet {
            let hit = self.base_bet * 2u64.pow(context.consec_bet_loses.len() as u32);
            Bet::Hit(hit)
        }
    }
}
