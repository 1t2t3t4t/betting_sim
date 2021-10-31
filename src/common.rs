pub struct Environment {
    pub win_rate: f64,
    pub return_ratio: f64,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            win_rate: 0.4,
            return_ratio: 2.0,
        }
    }
}

#[derive(Debug)]
pub enum BetRecord {
    Win(u64),
    Lose(u64),
}

#[derive(Debug)]
pub struct BetContext {
    pub total_money: u64,
    pub consec_bet_loses: Vec<u64>,
    pub records: Vec<BetRecord>,
}

impl Default for BetContext {
    fn default() -> Self {
        Self {
            total_money: 10000,
            consec_bet_loses: Vec::with_capacity(10),
            records: Vec::new(),
        }
    }
}
