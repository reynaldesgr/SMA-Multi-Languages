use super::*;
use mersenne_twister_m::MT19937;
use rand::{Rng, SeedableRng};

#[derive(Clone)]
pub struct Agent {
    id: i32,
    state: State,
    position: Position,
    time_in_state: f64,
    dE: f64,
    dI: f64,
    dR: f64,
}

impl Agent
{
    pub fn empty() -> Self {
        Self {
            id: 0,
            state: State::S,
            position: Position::new(0, 0),
            time_in_state: 0.0,
            dE: 0.0,
            dI: 0.0,
            dR: 0.0,
        }
    }

    pub fn new(id: i32, state: State, position: Position, dE: f64, dI: f64, dR: f64) -> Self {
        Self {
            id,
            state,
            position,
            time_in_state: 0.0,
            dE,
            dI,
            dR,
        }
    } 

    pub fn infection(&mut self, probability_to_get_infected: f64) {

        let random_uniform_number: f64 = rand::random::<f64>(); 

        if random_uniform_number < probability_to_get_infected {
            self.set_state(State::E);
            self.time_in_state = 0.0;
        }
    }

    pub fn check_state(&mut self) {
        match self.state {
            State::E => {
                if self.time_in_state > self.dE {
                    self.set_state(State::I);
                    self.time_in_state = 0.0;
                }
            },
            State::I => {
                if self.time_in_state > self.dI {
                    self.set_state(State::R);
                    self.time_in_state = 0.0;
                }
            },
            State::R => {
                if self.time_in_state > self.dR {
                    self.set_state(State::S);
                    self.time_in_state = 0.0;
                }
            },
            _ => {}
        }
    }

    pub fn move_agent(&mut self) {
        let mut rng = RANDOM.lock().unwrap();
        let dist_x = Uniform::new(0, GRID_WIDTH as i32);
        let dist_y = Uniform::new(0, GRID_HEIGHT as i32);

        let new_pos = Position {
            x: (rng.genrand() as usize % GRID_WIDTH) as i32,
            y: (rng.genrand() as usize % GRID_HEIGHT) as i32
        };

        self.set_position(new_pos);
    }

    pub fn increase_time_in_state(&mut self) {
        self.time_in_state += 1f64;
    }

    pub fn display_agent(&self) {
        println!("Agent ID: {}", self.id);
        println!("State: {}", self.get_char_state());
        println!("Position (x, y): {}, {}", self.position.x, self.position.y);
        println!("Time in State: {}", self.time_in_state);
        println!("dE: {}, dI: {}, dR: {}", self.dE, self.dI, self.dR);
        println!();
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_time_in_state(&self) -> f64 {
        self.time_in_state
    }

    pub fn get_char_state(&self) -> char {
        match self.state {
            State::S => 'S',
            State::E => 'E',
            State::I => 'I',
            State::R => 'R',
        }
    }

    pub fn get_num_state(&self) -> State {
        self.state
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_dE(&self) -> f64 {
        self.dE
    }

    pub fn get_dI(&self) -> f64 {
        self.dI
    }

    pub fn get_dR(&self) -> f64 {
        self.dR
    }

    pub fn set_position(&mut self, new_pos: Position) {
        self.position = new_pos;
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
        self.time_in_state = 0.0;
    }
}
