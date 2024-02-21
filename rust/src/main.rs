pub mod grid;
pub use grid::*;

pub mod population;
pub use population::*;

pub mod agent;
pub use agent::*;

pub mod state;
pub use state::*;

pub mod position;
pub use position::*;

pub use rand::*;
use rand::distributions::{Distribution, Uniform};

use std::fs::File;
use std::io::prelude::*;
use std::fmt;


const NUM_AGENTS: usize = 20000;
const NUM_INFECTED: usize = 20;
const NUM_DAYS: usize = 730;
const NUM_SIMULATIONS: usize = 100;

pub struct SimulationData {
    day: usize,
    susceptible_count: usize,
    exposed_count: usize,
    infected_count: usize,
    recovered_count: usize,
}

impl fmt::Display for SimulationData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.day,
            self.susceptible_count,
            self.exposed_count,
            self.infected_count,
            self.recovered_count
        )
    }
}

fn simulate_an_epidemic() {
    let mut populations: Vec<Population> = Vec::with_capacity(NUM_SIMULATIONS);
    let mut grids: Vec<Grid> = Vec::with_capacity(NUM_SIMULATIONS);
    let mut csv_files: Vec<File> = Vec::with_capacity(NUM_SIMULATIONS);

    for _ in 0..NUM_SIMULATIONS {
        let population = Population::new();
        populations.push(population);
        let grid = Grid::new(); 
        grids.push(grid);
    }
    
    for sim in 0..NUM_SIMULATIONS {
        populations[sim].initialize_population();
        grids[sim].initialize_grid(&mut populations[sim]);

        let filename = format!("simulation/simulation_data{}.csv", sim + 1);
        let mut file = File::create(&filename).expect("Error opening file");
        writeln!(&mut file, "day,susceptibleCount,exposedCount,infectedCount,recoveredCount")
            .expect("Error writing to file");
        csv_files.push(file);
    }

    println!("Populations and grids successfully initialized for {} simulations.", NUM_SIMULATIONS);

    for day in 0..NUM_DAYS {
        println!("Day {} simulating...", day + 1);
        for sim in 0..NUM_SIMULATIONS {
            grids[sim].simulate();

            let simulation_data = SimulationData {
                day: day,
                susceptible_count: grids[sim].get_state(0).unwrap_or(0) as usize,
                exposed_count: grids[sim].get_state(1).unwrap_or(0) as usize,
                infected_count: grids[sim].get_state(2).unwrap_or(0) as usize,
                recovered_count: grids[sim].get_state(3).unwrap_or(0) as usize, 
            };
            writeln!(csv_files[sim], "{}", simulation_data).expect("Error writing to file");
        }
    }
}

fn main() {
    simulate_an_epidemic();
}
