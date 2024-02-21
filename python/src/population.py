import math
import random
from agent import Agent
from position import Position
from state import State
from grid import Grid

class Population:
    NUM_AGENTS      = 20000
    NUM_INFECTED    = 20

    def __init__(self):
        self.agents = []
        random.seed(5489)

    def neg_exp(self, in_mean):
        """Generates a random value with a negative exponential distribution."""
        u = random.random()
        return -in_mean * math.log(1.0 - u)

    def initialize_population(self):
        for i in range(Population.NUM_AGENTS):
            d_e = self.neg_exp(3.)
            d_i = self.neg_exp(7.)
            d_r = self.neg_exp(365.)

            state = State.S if i < Population.NUM_AGENTS - Population.NUM_INFECTED else State.I

            x_pos = random.randint(0, Grid.GRID_WIDTH - 1)
            y_pos = random.randint(0, Grid.GRID_HEIGHT - 1)

            idv = Agent(i, state, Position(x_pos, y_pos), d_e, d_i, d_r)
            self.add_agent(idv)

    def add_agent(self, agent):
        """Adds an agent to the population."""
        self.agents.append(agent)

    def display_population(self):
        """Displays information about the population."""
        print("Population information:")
        for agent in self.agents:
            agent.display_agent()

    def get_agents(self):
        """Returns the list of agents in the population."""
        return self.agents

    def get_agent_by_id(self, agent_id):
        """Retrieves an agent by its unique identifier (ID)."""
        for agent in self.agents:
            if agent.get_id() == agent_id:
                return agent
        return None