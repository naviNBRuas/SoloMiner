use num_cpus;
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum MiningMode {
    Performance,
    Conservative,
}

pub fn get_recommended_threads(mode: MiningMode) -> usize {
    let num_cpus = num_cpus::get();
    match mode {
        MiningMode::Performance => {
            // Use all available logical cores for performance mode
            num_cpus
        }
        MiningMode::Conservative => {
            // Use half of the available logical cores for conservative mode, at least 1
            (num_cpus / 2).max(1)
        }
    }
}
