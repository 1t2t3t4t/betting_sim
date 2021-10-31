use std::collections::HashMap;

use strategy::{Bet, Strategy};

use crate::strategy::DummyStrat;

mod strategy;
mod utils;

struct Environment {
    win_rate: f64,
    return_ratio: f64,
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

fn perform_bet_strat(strat: &Box<dyn Strategy>, context: &mut BetContext, env: &Environment) {
    let bet = strat.bet(context);
    match bet {
        Bet::Hit(bet_amount) => {
            let bet_amount = if bet_amount > 0 { bet_amount } else { context.total_money };
            context.total_money -= bet_amount;
            let res = utils::bet_result(env, bet_amount);
            if let Some(win_amount) = res {
                context.total_money += win_amount;
                context.records.push(BetRecord::Win(win_amount));
                context.consec_bet_loses.clear();
            } else {
                context.consec_bet_loses.push(bet_amount);
                context.records.push(BetRecord::Lose(bet_amount));
            }
        }
        Bet::Down => (),
    }
}

fn main() {
    let env = Environment::default();
    let strategies: Vec<Box<dyn Strategy>> = vec![Box::new(DummyStrat)];
    let mut context_map = strategies
        .iter()
        .map(|v| (v.name(), BetContext::default()))
        .collect::<HashMap<&str, BetContext>>();

    while !utils::should_end(&context_map) {
        for strat in &strategies {
            let context = context_map.get_mut(strat.name()).unwrap();
            if utils::is_broke(context) {
                continue;
            }
            perform_bet_strat(strat, context, &env)
        }
    }

    println!("Summarize\n{:?}", context_map);
}
