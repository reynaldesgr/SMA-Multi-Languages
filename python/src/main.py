import csv
import os
from population import Population
from grid       import Grid
from state      import State

NUM_SIMULATIONS = 100
NUM_DAYS        = 730

def simulate_an_epidemic():
    populations = [Population() for _ in range(NUM_SIMULATIONS)]
    grids = [Grid() for _ in range(NUM_SIMULATIONS)]
    csv_writers  = []
    csv_files    = []

    os.makedirs("simulation", exist_ok=True)
    for sim in range(NUM_SIMULATIONS):
        populations[sim].initialize_population()
        grids[sim].initialize_grid(populations[sim])

        filename = f"simulation/simulation_data{sim + 1}.csv"
        csv_file = open(filename, 'w', newline='')
        csv_files.append(csv_file)
        csv_writer = csv.writer(csv_file)
        csv_writer.writerow(["day", "susceptibleCount", "exposedCount", "infectedCount", "recoveredCount"])
        csv_writers.append(csv_writer)

    print(f"Populations and grids successfully initialized for {NUM_SIMULATIONS} simulations.")
    print("Computing simulation ...")
        
    for day in range(NUM_DAYS):
        print(f"Day {day + 1} simulating...")
        try:
            for sim in range(NUM_SIMULATIONS):
                filename = f"simulation/simulation_data{sim + 1}.csv"
                grids[sim].simulate()

                num_s = grids[sim].get_state(0)
                num_e = grids[sim].get_state(1)
                num_i = grids[sim].get_state(2)
                num_r = grids[sim].get_state(3)

                csv_writers[sim].writerow([day, num_s, num_e, num_i, num_r])
        except IOError:
            print(f"Error opening file: {filename}")
    
    for csv_file in csv_files:
        csv_file.close()

if __name__ == "__main__":
    simulate_an_epidemic()