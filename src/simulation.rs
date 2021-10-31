use std::collections::HashMap;

use crate::strategy::Bet;

use crate::common::{BetContext, BetRecord, Environment};

use crate::strategy::Strategy;
use crate::utils;

pub(crate) type SimulationResult = BetContext;

pub(crate) fn simulate(
    env: Environment,
    strategy: &dyn Strategy,
    context_builder: impl Fn() -> BetContext,
) -> SimulationResult {
    let mut context = context_builder();

    while !utils::should_end(&context) {
        perform_bet_strat(strategy, &mut context, &env)
    }
    context
}

fn perform_bet_strat(strat: &dyn Strategy, context: &mut BetContext, env: &Environment) {
    let bet = strat.bet(context);
    match bet {
        Bet::Hit(bet_amount) => {
            let bet_amount = if bet_amount > 0 && bet_amount <= context.total_money {
                bet_amount
            } else {
                context.total_money
            };
            let before_bet_tot = context.total_money;
            context.total_money -= bet_amount;

            let res = utils::bet_result(env, bet_amount);
            if let Some(win_amount) = res {
                context.total_money += win_amount;
                context
                    .records
                    .push(BetRecord::Win(win_amount - bet_amount, before_bet_tot));
                context.consec_bet_loses.clear();
                if context.total_money > context.max_total_money {
                    context.max_total_money = context.total_money;
                }
            } else {
                context.consec_bet_loses.push(bet_amount);
                context
                    .records
                    .push(BetRecord::Lose(bet_amount, before_bet_tot));
            }
        }
    }
}