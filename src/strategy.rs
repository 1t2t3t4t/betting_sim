use crate::BetContext;

pub enum Bet {
    Hit(u64),
    Down,
}

pub trait Strategy {
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
