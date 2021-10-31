use std::{
    fs::write,
    sync::{self},
    time::Instant,
};

use common::BetContext;
use strategy::Strategy;

use crate::{
    common::Environment,
    strategy::{martingale::MartinGaleStrat, TenPercentStrat},
};

mod common;
mod simulation;
mod strategy;
mod utils;

fn boxed<S: 'static + Strategy>(strat: S) -> Box<dyn Strategy> {
    Box::new(strat)
}

fn strategies_builder() -> Vec<Box<dyn Strategy>> {
    vec![boxed(MartinGaleStrat::new()), boxed(TenPercentStrat)]
}

fn main() {
    let start_time = Instant::now();
    let envs = vec![
        Environment::default(),
        Environment::new(30000),
        Environment::new(50000),
        Environment::new(100000),
        Environment::new(200000),
        Environment::new(1000000),
    ];

    let (tx, rx) = sync::mpsc::channel();
    for env in envs.clone() {
        let strategies = strategies_builder();
        for strategy in strategies {
            let clone_tx = tx.clone();
            std::thread::spawn(move || {
                let sim_result =
                    simulation::simulate(env, strategy.as_ref(), || BetContext::new(strategy.name(), env.start_amount));
                clone_tx.send(sim_result).unwrap();
            });
        }
    }

    for _ in 0..envs.len() {
        let result = rx.recv().unwrap();
        let file_name = format!("{}_{:#2}", result.strat_name, result.start_money);
        write_summary_report(format!("./out/{}/{}.txt", result.strat_name, file_name), &result);
    }

    println!(
        "Simulation End {} ms.",
        Instant::now().duration_since(start_time).as_millis()
    );
}

fn write_summary_report(name: String, context: &BetContext) {
    let p = std::path::Path::new(&name);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut report = Vec::<String>::new();
    report.push(format!("{:#?}", context));
    let report_str = report.join("\n");
    write(name, report_str).unwrap();
}
