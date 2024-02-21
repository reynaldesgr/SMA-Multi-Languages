require 'csv'
require './agent.rb'
require './population.rb'

NUM_AGENTS = 20_000
NUM_INFECTED = 20
NUM_DAYS = 730
NUM_SIMULATIONS = 100

class SimulationData
  attr_accessor :day, :susceptible_count, :exposed_count, :infected_count, :recovered_count

  def initialize(day, susceptible_count, exposed_count, infected_count, recovered_count)
    @day               = day
    @susceptible_count = susceptible_count
    @exposed_count     = exposed_count
    @infected_count    = infected_count
    @recovered_count   = recovered_count
  end

  def to_csv
    [@day, @susceptible_count, @exposed_count, @infected_count, @recovered_count].join(',')
  end
end

def simulate_an_epidemic
  populations = Array.new(NUM_SIMULATIONS) { Population.new } 
  grids = Array.new(NUM_SIMULATIONS) { Grid.new }
  csv_files = []

  NUM_SIMULATIONS.times do |sim|
    populations[sim].initialize_population
    grids[sim].initialize_grid(populations[sim])

    filename = "simulation/simulation_data#{sim + 1}.csv"
    CSV.open(filename, 'wb') do |csv|
      csv << ['day', 'susceptibleCount', 'exposedCount', 'infectedCount', 'recoveredCount']
      csv_files << csv
    end
  end

  puts "Populations and grids successfully initialized for #{NUM_SIMULATIONS} simulations."

  NUM_DAYS.times do |day|
    puts "Day #{day + 1} simulating..."
    NUM_SIMULATIONS.times do |sim|
      grids[sim].simulate 

      simulation_data = SimulationData.new(
        day,
        grids[sim].get_state(0), 
        grids[sim].get_state(1),
        grids[sim].get_state(2),
        grids[sim].get_state(3)
      )

      CSV.open("simulation/simulation_data#{sim + 1}.csv", 'a') do |csv|
        csv << simulation_data.to_csv.split(',')
      end
    end
  end
end

def main
  simulate_an_epidemic
end

main if __FILE__ == $PROGRAM_NAME