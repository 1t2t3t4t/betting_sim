use std::{
    collections::HashMap,
    fs::write,
    sync::{self, Arc},
    thread,
};

use common::BetContext;
use strategy::Strategy;

use crate::{common::Environment, strategy::martingale::MartinGaleStrat};

mod common;
mod simulation;
mod strategy;
mod utils;

fn boxed<S: 'static + Strategy>(strat: S) -> Arc<dyn Strategy> {
    Arc::new(strat)
}

fn main() {
    let envs = vec![
        Environment::default(),
        Environment::new(30000),
        Environment::new(50000),
        Environment::new(100000),
    ];
    let strategies: Vec<Arc<dyn Strategy>> = vec![boxed(MartinGaleStrat::new())];

    let (tx, rx) = sync::mpsc::sync_channel(envs.len());
    for env in envs.clone() {
        let clone_tx = tx.clone();
        let clone_strategies = strategies.clone();
        thread::spawn(move || {
            let sim_result =
                simulation::simulate(env, &clone_strategies, || BetContext::new(env.start_amount));
            clone_tx.send(sim_result).unwrap();
        });
    }

    for idx in 0..envs.len() {
        let result = rx.recv().unwrap();
        write_summary_report(format!("./out_{}", idx), &result);
    }

    println!("Simulation End");
}

fn write_summary_report(name: String, context_map: &HashMap<&str, BetContext>) {
    let mut report = Vec::<String>::new();
    for context in context_map {
        report.push(format!("====== {} ======", context.0));
        report.push(format!("{:#?}", context.1));
        report.push("===============\n\n".to_string());
    }
    let report_str = report.join("\n");
    write(name, report_str).unwrap();
}
