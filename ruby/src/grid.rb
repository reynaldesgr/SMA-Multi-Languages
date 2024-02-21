require './population.rb'
require './state.rb'

class Grid
    NB_STATES   = 4
    GRID_HEIGHT = 300
    GRID_WIDTH  = 300
  
    attr_accessor :width, :height, :states, :grid_population, :zone
  
    def initialize
      @width  = GRID_WIDTH
      @height = GRID_HEIGHT
      @states = [0] * NB_STATES
      @grid_population = Population.new
      @grid_population.initialize_population
  
      @zone = Array.new(GRID_WIDTH) {
        Array.new(GRID_HEIGHT) { [] }
      }
    end
  
    def initialize_grid(population)
      population.agents.each do |agent|
        x = agent.position.x
        y = agent.position.y
  
        if x < GRID_WIDTH && y < GRID_HEIGHT
          @zone[x][y] << agent.clone
        end
      end
    end
  
    def move_population
      clear_grid
  
      @grid_population.agents.each do |agent|
        agent.move_agent
  
        x = agent.position.x
        y = agent.position.y
  
        if x < @width && y < @height
          @zone[x][y] << agent.clone
        end
      end
    end
  
    def clear_grid
      @zone.each { |column| column.each(&:clear) }
    end
  
    def clear_grid_states
      @states = [0] * NB_STATES
    end
  
    def display_grid
      @zone.each do |column|
        column.each do |cell|
          cell.each { |agent| print agent.get_char_state }
          print "0" if cell.empty?
        end
        puts
      end
    end
  
    def simulate
      clear_grid_states
      move_population
  
      to_infect = []
      to_check = []
  
      @grid_population.agents.each_with_index do |agent, index|
        if agent.state == State::S
          x = agent.position.x
          y = agent.position.y
          probability = probability_to_get_infected_at(x, y)
          to_infect << [index, probability]
        else
          to_check << index
        end
      end
  
      to_infect.each do |index, probability|
        agent = @grid_population.agents[index]
        agent.infection(probability)
      end
  
      to_check.each do |index|
        agent = @grid_population.agents[index]
        agent.check_state
      end
  
      @grid_population.agents.each do |agent|
        agent.increase_time_in_state
        case agent.state
        when State::S then @states[0] += 1
        when State::E then @states[1] += 1
        when State::I then @states[2] += 1
        when State::R then @states[3] += 1
        end
      end
    end
  
    def probability_to_get_infected_at(x, y)
      infectious_neighbors = 0
  
      (-1..1).each do |dx|
        (-1..1).each do |dy|
          neighbor_x = (x + dx + @width) % @width
          neighbor_y = (y + dy + @height) % @height
  
          @zone[neighbor_x][neighbor_y].each do |agent|
            infectious_neighbors += 1 if agent.state == :I
          end
        end
      end
  
      1.0 - Math.exp(-0.5 * infectious_neighbors)
    end

    def get_state(index)
      if index >= 0 && index < NB_STATES
        states[index]
      else
        -1
      end
    end

    def get_width
      @width
    end

    def get_height
      @height
    end

  end  