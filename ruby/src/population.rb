require 'securerandom'
require './agent.rb'
require './position.rb'

class Population
  attr_accessor :agents, :duration_time

  def initialize
    @agents = []
    @duration_time = 0
    @rng = Random.new(5489)
  end

  def neg_exp(lambda)
    u = @rng.rand
    -lambda * Math.log(1.0 - u)
  end

  def initialize_population
    NUM_AGENTS.times do |i|
      d_e = neg_exp(3.0)
      d_i = neg_exp(7.0)
      d_r = neg_exp(365.0)

      state = i < NUM_AGENTS - NUM_INFECTED ? :S : :I

      x_pos = @rng.rand(0...Grid::GRID_WIDTH)
      y_pos = @rng.rand(0...Grid::GRID_HEIGHT)

      agent = Agent.new(i, state, Position.new(x_pos, y_pos), d_e, d_i, d_r)
      add_agent(agent)
    end
  end

  def add_agent(agent)
    @agents << agent
  end

  def display_population
    puts "Population :"
    @agents.each(&:display_agent)
  end

  def get_agents
    @agents
  end

  def get_mut_agent(index)
    @agents[index]
  end
end