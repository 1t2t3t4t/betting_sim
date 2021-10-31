use std::collections::HashMap;

use strategy::{Bet, Strategy};

use crate::strategy::DummyStrat;

mod strategy;
mod utils;

mod common;

fn perform_bet_strat(
    strat: &Box<dyn Strategy>,
    context: &mut common::BetContext,
    env: &common::Environment,
) {
    let bet = strat.bet(context);
    match bet {
        Bet::Hit(bet_amount) => {
            let bet_amount = if bet_amount > 0 {
                bet_amount
            } else {
                context.total_money
            };
            context.total_money -= bet_amount;
            let res = utils::bet_result(env, bet_amount);
            if let Some(win_amount) = res {
                context.total_money += win_amount;
                context.records.push(common::BetRecord::Win(win_amount));
                context.consec_bet_loses.clear();
            } else {
                context.consec_bet_loses.push(bet_amount);
                context.records.push(common::BetRecord::Lose(bet_amount));
            }
        }
        Bet::Down => (),
    }
}

fn main() {
    let env = common::Environment::default();
    let strategies: Vec<Box<dyn Strategy>> = vec![Box::new(DummyStrat)];
    let mut context_map = strategies
        .iter()
        .map(|v| (v.name(), common::BetContext::default()))
        .collect::<HashMap<&str, common::BetContext>>();

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
