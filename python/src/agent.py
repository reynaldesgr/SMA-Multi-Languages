import random
from state     import State
from position import Position
from grid     import Grid

class Agent:
    def __init__(self, id=0, state=State.S, position=Position(0, 0), dE=0, dI=0, dR=0):
        self.id = id
        self.state = state
        self.position = position
        self.timeInState = 0.0
        self.dE = dE
        self.dI = dI
        self.dR = dR

    def infection(self, probabilityToGetInfected):
        if random.random() < probabilityToGetInfected:
            self.state = State.E
            self.timeInState = 0.0

    def check_state(self):
        if self.state == State.E and self.timeInState > self.dE:
            self.state = State.I
            self.timeInState = 0.0
        elif self.state == State.I and self.timeInState > self.dI:
            self.state = State.R
            self.timeInState = 0.0
        elif self.state == State.R and self.timeInState > self.dR:
            self.state = State.S
            self.timeInState = 0.0

    def move(self):
        newX = random.randint(0, Grid.GRID_WIDTH - 1)
        newY = random.randint(0, Grid.GRID_HEIGHT - 1)
        self.position = Position(newX, newY)

    def increase_time_in_state(self):
        self.timeInState += 1

    def display_agent(self):
        state_char = self.get_char_state()
        print(f"Agent ID: {self.id}")
        print(f"State: {state_char}")
        print(f"Position (x, y): {self.position.x}, {self.position.y}")
        print(f"Time in State: {self.timeInState}")
        print(f"dE: {self.dE}, dI: {self.dI}, dR: {self.dR}")
        print()

    def get_id(self):
        return self.id

    def get_time_in_state(self):
        return self.timeInState

    def get_char_state(self):
        return self.state.name

    def get_num_state(self):
        return self.state

    def get_position(self):
        return self.position

    def get_de(self):
        return self.dE

    def get_di(self):
        return self.dI

    def get_dr(self):
        return self.dR

    def set_position(self, newPos):
        self.position = newPos

    def set_state(self, newState):
        self.state = newState
        self.timeInState = 0.0