import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class Population {
    private final int NUM_AGENTS   = 20000; 
    private final int NUM_INFECTED = 20; 
    private final int GRID_WIDTH   = 300; 
    private final int GRID_HEIGHT  = 300; 

    private final List<Agent> agents = new ArrayList<>();

    public void initializePopulation() {
        MTRandom random = new MTRandom();
        random.setSeed(5489);
        
        for (int i = 0; i < NUM_AGENTS; i++) {
            double dE = negExp(3, random);
            double dI = negExp(7, random);
            double dR = negExp(365, random);

            State s;
            if (i < NUM_AGENTS - NUM_INFECTED) {
                s = State.S; // Agent S
            } else {
                s = State.I; // Agent I
            }

            Agent idv = new Agent(i, s, new Position(random.nextInt(GRID_WIDTH), random.nextInt(GRID_HEIGHT)), dE, dI, dR);
            agents.add(idv);
        }
    }

    private double negExp(double inMean, Random random) {
        double u = random.nextDouble();
        return -inMean * Math.log(1.0 - u);
    }

    public void addAgent(Agent idv) {
        agents.add(idv);
    }

    public void displayPopulation() {
        System.out.println("Population information:");
        for (Agent agent : agents) {
            agent.displayAgent();
        }
    }

    public List<Agent> getAgents() {
        return agents;
    }

    public Agent getAgentById(int id) {
        for (Agent agent : agents) {
            if (agent.getId() == id) {
                return agent;
            }
        }
        return null;
    }
}
