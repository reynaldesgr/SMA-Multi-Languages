import math
from state import State

class Grid:
    GRID_WIDTH  = 300
    GRID_HEIGHT = 300

    NB_STATES   = 4
 
    def __init__(self):
        self.zone   = [[[] for _ in range(Grid.GRID_HEIGHT)] for _ in range(Grid.GRID_WIDTH)]
        self.width  = Grid.GRID_WIDTH
        self.height = Grid.GRID_HEIGHT
        self.states = [0] * Grid.NB_STATES

    def initialize_grid(self, population):
        self.grid_population = population
        agents = population.get_agents()
        for agent in agents:
            x, y = agent.get_position().get_x(), agent.get_position().get_y()
            self.zone[x][y].append(agent)

    def move_population(self):
        self.clear_grid()
        for agent in self.grid_population.get_agents():
            agent.move()
            x, y = agent.get_position().get_x(), agent.get_position().get_y()
            self.zone[x][y].append(agent)

    def clear_grid(self):
        for x in range(self.width):
            for y in range(self.height):
                self.zone[x][y].clear()

    def clear_grid_states(self):
        self.states = [0] * Grid.NB_STATES

    def display_grid(self):
        for x in range(self.width):
            for y in range(self.height):
                if self.zone[x][y]:
                    for agent in self.zone[x][y]:
                        print(agent.get_char_state(), end="")
                else:
                    print("0", end="")
            print()

    def simulate(self):
        self.clear_grid_states()
        self.move_population()
        for agent in self.grid_population.get_agents():
            if agent.get_num_state() == State.S:
                agent.infection(self.probability_to_get_infected_at(agent.get_position().get_x(), agent.get_position().get_y()))
            else:
                agent.check_state()
            agent.increase_time_in_state()
            state = agent.get_num_state()
            self.states[state.value - 1] += 1

    def probability_to_get_infected_at(self, x, y):
        infectious_neighbors = 0
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                neighbor_x = (x + dx + self.width) % self.width
                neighbor_y = (y + dy + self.height) % self.height
                for agent in self.zone[neighbor_x][neighbor_y]:
                    if agent is not self.zone[x][y][0] and agent.get_num_state() == State.I:
                        infectious_neighbors += 1
        probability = 1.0 - math.exp(-0.5 * infectious_neighbors)
        return probability

    def get_width(self):
        return self.width

    def get_height(self):
        return self.height

    def get_state(self, index):
        if 0 <= index < len(self.states):
            return self.states[index]
        return -1