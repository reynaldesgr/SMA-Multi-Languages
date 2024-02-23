use std::vec::Vec;
use mersenne_twister_m::MT19937;
use rand::{Rng, SeedableRng};

use super::*;

const NUM_AGENTS: usize = 20000;
const NUM_INFECTED: usize = 20;
const NUM_DAYS: usize = 730;
const NUM_SIMULATIONS: usize = 100;

fn neg_exp(lambda: f64) -> f64 {
    let u: f64 = rand::thread_rng().gen(); // Generates a random number in [0, 1)
    -lambda * (1.0 - u).ln()
}

pub struct Population {
    pub agents: Vec<Agent>,
    pub duration_time: i32,
}

impl Population {
    pub fn new() -> Self {
        Self {
            agents: Vec::with_capacity(NUM_AGENTS),
            duration_time: 0, 
        }
    }
    

    pub fn initialize_population(&mut self) {
        let mut rng = RANDOM.lock().unwrap();

        for i in 0..NUM_AGENTS {
            let d_e = neg_exp(3.);
            let d_i = neg_exp(7.);
            let d_r = neg_exp(365.);

            let s = if i < NUM_AGENTS - NUM_INFECTED {
                State::S
            } else {
                State::I
            };

            let x_pos = rng.genrand() as usize % GRID_WIDTH;
            let y_pos = rng.genrand() as usize % GRID_HEIGHT;


            let idv = Agent::new(i as i32, s, Position::new(x_pos.try_into().unwrap(), y_pos.try_into().unwrap()), d_e, d_i, d_r);
            self.add_agent(idv);
        }
    }

    pub fn add_agent(&mut self, idv: Agent) {
        self.agents.push(idv);
    }

    pub fn display_population(&self) {
        println!("Population information:");
        for agent in &self.agents {
            agent.display_agent();
        }
    }

    pub fn get_agents(&self) -> &[Agent] {
        &self.agents
    }

    pub fn get_mut_agent(&mut self, index: usize) -> Option<&mut Agent> {
        self.agents.get_mut(index)
    }

}