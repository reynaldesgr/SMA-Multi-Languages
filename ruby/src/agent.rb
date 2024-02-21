require './state.rb'
require './grid.rb'

class Agent
  attr_accessor :id, :state, :position, :time_in_state, :dE, :dI, :dR


  def initialize(id, state, position, dE, dI, dR)
    @id = id
    @state = state
    @position = position
    @time_in_state = 0.0
    @dE = dE
    @dI = dI
    @dR = dR
  end

  def self.empty
    new
  end

  def infection(probability_to_get_infected)
    if rand < probability_to_get_infected
      set_state(State::E)
      @time_in_state = 0.0
    end
  end

  def check_state
    case @state
    when State::E
      if @time_in_state > @dE
        set_state(State::I)
        @time_in_state = 0.0;
      end
    when State::I
      if @time_in_state > @dI
        set_state(State::R)
        @time_in_state = 0.0;
      end
    when State::R
      if @time_in_state > @dR
        set_state(State::S)
        @time_in_state = 0.0;
      end
    end
  end

  def move_agent
    @position.x = rand(0...Grid::GRID_WIDTH)
    @position.y = rand(0...Grid::GRID_HEIGHT)
  end

  def display_agent
    puts "Agent ID: #{@id}"
    puts "State: #{@state}"
    puts "Position (x, y): #{@position[:x]}, #{@position[:y]}"
    puts "Time in State: #{@time_in_state}"
    puts "dE: #{@dE}, dI: #{@dI}, dR: #{@dR}"
    puts ""
  end

  def increase_time_in_state
    @time_in_state += 1.0
  end
  
  def get_state
    @state
  end

  def get_position
    @position
  end 

  def get_time_in_state
    @time_in_state
  end 
  
  def get_dE 
    @dE
  end

  def get_dR
    @dR
  end

  def get_dI 
    @dI
  end
  
  private
  def set_state(new_state)
    @state = new_state
    @time_in_state = 0.0
  end
end