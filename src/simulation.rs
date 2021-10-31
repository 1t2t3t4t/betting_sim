use std::collections::HashMap;

use crate::strategy::Bet;

use crate::common::{BetContext, BetRecord, Environment};

use crate::strategy::Strategy;
use crate::utils;

use std::sync::Arc;

pub(crate) fn perform_bet_strat(
    strat: &Arc<dyn Strategy>,
    context: &mut BetContext,
    env: &Environment,
) {
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
            } else {
                context.consec_bet_loses.push(bet_amount);
                context
                    .records
                    .push(BetRecord::Lose(bet_amount, before_bet_tot));
            }
        }
    }
}

pub(crate) type SimulationResult = HashMap<&'static str, BetContext>;

pub(crate) fn simulate(
    env: Environment,
    strategies: &[Arc<dyn Strategy>],
    context_builder: impl Fn() -> BetContext,
) -> SimulationResult {
    let mut context_map = strategies
        .iter()
        .map(|v| (v.name(), context_builder()))
        .collect::<HashMap<&str, BetContext>>();

    while !utils::should_end(&context_map) {
        for strat in strategies {
            let context = context_map.get_mut(strat.name()).unwrap();
            if utils::is_broke(context) {
                continue;
            }
            perform_bet_strat(strat, context, &env)
        }
    }
    context_map
}
