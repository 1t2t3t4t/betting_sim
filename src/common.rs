#[derive(Clone, Copy)]
pub struct Environment {
    pub win_rate: f64,
    pub return_ratio: f64,
    pub start_amount: u64,
}

impl Environment {
    pub fn new(start_amount: u64) -> Self {
        Self {
            start_amount,
            ..Default::default()
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            win_rate: 0.45,
            return_ratio: 2.0,
            start_amount: 10000,
        }
    }
}

#[derive(Debug)]
pub enum BetRecord {
    Win(u64, u64),
    Lose(u64, u64),
}

#[derive(Debug)]
pub struct BetContext {
    pub strat_name: String,
    pub start_money: u64,
    pub total_money: u64,
    pub max_total_money: u64,
    pub consec_bet_loses: Vec<u64>,
    pub records: Vec<BetRecord>,
}

impl BetContext {
    pub fn new(strat_name: &str, start_money: u64) -> Self {
        Self {
            strat_name: strat_name.to_string(),
            start_money,
            total_money: start_money,
            max_total_money: start_money,
            ..Default::default()
        }
    }
}

impl Default for BetContext {
    fn default() -> Self {
        Self {
            strat_name: String::new(),
            start_money: 50000,
            total_money: 50000,
            max_total_money: 50000,
            consec_bet_loses: Vec::with_capacity(10),
            records: Vec::new(),
        }
    }
}
