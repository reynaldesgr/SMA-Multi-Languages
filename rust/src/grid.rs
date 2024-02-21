use std::collections::LinkedList;
use std::convert::TryInto;
use super::*;

pub const NB_STATES: usize = 4;
pub const GRID_HEIGHT: usize = 300;
pub const GRID_WIDTH: usize = 300;

pub struct Grid
{
    width: usize,
    height: usize,
    states: [usize; NB_STATES],
    grid_population: Population,
    zone: Vec<Vec<LinkedList<Agent>>>
}

impl Grid 
{
    pub fn new() -> Self {
        let states = [0; NB_STATES]; 
        let mut grid_population = Population {
            agents: Vec::with_capacity(NUM_AGENTS),
            duration_time: 0, 
        };

        grid_population.initialize_population();

        let zone = (0..GRID_WIDTH).map(|_| {
            (0..GRID_HEIGHT).map(|_| LinkedList::new()).collect::<Vec<LinkedList<Agent>>>()
        }).collect::<Vec<Vec<LinkedList<Agent>>>>();

        Grid {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            states,
            grid_population,
            zone,
        }
    }

    pub fn initialize_grid(&mut self, population: &mut Population) {
        for agent in population.get_agents() {
            let x = agent.get_position().x as usize;
            let y = agent.get_position().y as usize;

            if x < GRID_WIDTH.try_into().unwrap() && y < GRID_HEIGHT.try_into().unwrap() {
                self.zone[x][y].push_back(agent.clone());
            }
        }
    }

    pub fn move_population(&mut self) {
        self.clear_grid();

        let agents_count = self.grid_population.agents.len();
        for i in 0..agents_count {
            if let Some(agent) = self.grid_population.get_mut_agent(i) {
                agent.move_agent(); 

                let x = agent.get_position().x as usize;
                let y = agent.get_position().y as usize;

                if x < self.width && y < self.height {
                    self.zone[x][y].push_back(agent.clone());
                }
            }
        }
    }

    pub fn clear_grid(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.zone[x][y].clear();
            }
        }
    }

    pub fn clear_grid_states(&mut self) {
        for i in 0..NB_STATES {
            self.states[i] = 0;
        }
    }

    pub fn display_grid(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let list_agent = &self.zone[x][y];
                for e in list_agent {
                    print!("{}", e.get_char_state());
                }
                if list_agent.is_empty() {
                    print!("0");
                }
            }
            println!();
        }
    }

    pub fn simulate(&mut self) {
        self.clear_grid_states();
        self.move_population();

        let mut to_infect = Vec::new();
        let mut to_check  = Vec::new();

        for (i, agent) in self.grid_population.get_agents().iter().enumerate() {
            let state = agent.get_num_state();
            if state == State::S {
                let x = agent.get_position().x as usize;
                let y = agent.get_position().y as usize;
                let probability = self.probability_to_get_infected_at(x, y);
                to_infect.push((i, probability));
            }
            else
            {
               to_check.push(i);
            }
        }

        for (index, probability) in to_infect {
            if let Some(agent) = self.grid_population.get_mut_agent(index) {
                agent.infection(probability);
            }
        }

        for index in to_check
        {
            if let Some(agent) = self.grid_population.get_mut_agent(index) {
                agent.check_state();
            }
        }

        let agents_count = self.grid_population.get_agents().len();
        for i in 0..agents_count {
            if let Some(agent) = self.grid_population.get_mut_agent(i) {
                agent.increase_time_in_state();
                match agent.get_num_state() {
                    State::S => self.states[0] += 1,
                    State::E => self.states[1] += 1,
                    State::I => self.states[2] += 1,
                    State::R => self.states[3] += 1,
                }
            }
        }
    }

    pub fn probability_to_get_infected_at(&self, x: usize, y: usize) -> f64 {
        let mut infectious_neighbors = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let neighbor_x = (x as i32 + dx + self.width as i32) as usize % self.width;
                let neighbor_y = (y as i32 + dy + self.height as i32) as usize % self.height;

                if let Some(agents) = self.zone.get(neighbor_x).and_then(|row| row.get(neighbor_y)) {
                    for agent in agents {
                        if agent as *const _ != self.zone[x][y].front().unwrap() as *const _ && agent.get_num_state() == State::I {
                            infectious_neighbors += 1;
                        }
                    }
                }
            }
        }

        let probability = 1.0 - (-0.5 * infectious_neighbors as f64).exp();
        probability
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_state(&self, index: usize) -> Option<i32> {
        if index < NB_STATES {
            Some(self.states[index].try_into().unwrap())
        } else {
            None
        }
    }
}