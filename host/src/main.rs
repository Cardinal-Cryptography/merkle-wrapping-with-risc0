use std::time::Duration;
use risc0_zkvm::ExecutorEnv;
use crate::data::get_long_merkle_proof;

mod data;
mod only_stark;
mod stark_then_groth;

fn get_executor_env() -> ExecutorEnv<'static> {
    ExecutorEnv::builder()
        .write(&get_long_merkle_proof())
        .unwrap()
        .build()
        .unwrap()
}

fn measure<F: FnOnce() -> T, T>(f: F) -> (Duration, T) {
    let start = std::time::Instant::now();
    let result = f();
    let duration = std::time::Instant::now().duration_since(start);
    (duration, result)
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        // .with_max_level(tracing::Level::INFO)
        .init();

    println!("==== Running the Stark pipeline =====");
    only_stark::stark_pipeline(get_executor_env());
    println!("==== Running the Stark+Groth pipeline =====");
    stark_then_groth::wrapping_pipeline(get_executor_env());
}
