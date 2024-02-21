from enum import Enum

class State(Enum):
    
    S = 1  # Susceptible
    E = 2  # Exposed
    I = 3  # Infectious
    R = 4  # Recovered