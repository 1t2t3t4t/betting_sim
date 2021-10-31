use std::collections::HashMap;

use rand::{thread_rng, Rng};

use crate::{common::BetContext, common::Environment};

pub(crate) fn is_broke(context: &BetContext) -> bool {
    context.total_money == 0
}

pub(crate) fn reach_goal(context: &BetContext) -> bool {
    context.total_money >= context.start_money * 2
}

pub(crate) fn should_end(context: &HashMap<&str, BetContext>) -> bool {
    context.iter().all(|v| is_broke(v.1)) || context.iter().any(|v| reach_goal(v.1))
}

pub(crate) fn bet_result(env: &Environment, bet_amount: u64) -> Option<u64> {
    let win = thread_rng().gen_bool(env.win_rate);
    if win {
        Some((bet_amount as f64 * env.return_ratio) as u64)
    } else {
        None
    }
}
