import java.util.ArrayList;
import java.util.List;

public class Grid 
{
    public final static int GRID_WIDTH  = 300;
    public final static int GRID_HEIGHT = 300;
    public final        int NB_STATES   = 4;

    private final int width                    = GRID_WIDTH;
    private final int height                   = GRID_HEIGHT;
    private final List<List<List<Agent>>> zone = new ArrayList<>();

    private Population gridPopulation;
    private final int[] states = new int[NB_STATES];

    public Grid() {
        for (int i = 0; i < GRID_WIDTH; i++) {
            zone.add(new ArrayList<>(GRID_HEIGHT));
            for (int j = 0; j < GRID_HEIGHT; j++) {
                zone.get(i).add(new ArrayList<>());
            }
        }
    }

    public void initializeGrid(Population population) {
        int x, y;

        this.gridPopulation = population;

        List<Agent> agents = population.getAgents();

        for (Agent agent : agents) {
            x = agent.getPosition().getX();
            y = agent.getPosition().getY();
            zone.get(x).get(y).add(agent);
        }
    }

    public void movePopulation() {
        clearGrid();

        for (Agent agent : gridPopulation.getAgents()) {
            agent.move();
            zone.get(agent.getPosition().getX()).get(agent.getPosition().getY()).add(agent);
        }
    }

    public void clearGrid() {
        for (int x = 0; x < width; x++) {
            for (int y = 0; y < height; y++) {
                zone.get(x).get(y).clear();
            }
        }
    }

    public void clearGridStates() {
        for (int i = 0; i < NB_STATES; i++) {
            states[i] = 0;
        }
    }

    public void displayGrid() {
        for (int x = 0; x < width; x++) {
            for (int y = 0; y < height; y++) {
                List<Agent> listAgent = zone.get(x).get(y);
                for (Agent agent : listAgent) {
                    System.out.print(agent.getCharState());
                }
                if (listAgent.isEmpty()) {
                    System.out.print("0");
                }
            }
            System.out.println();
        }
    }

    public void simulate() {
        clearGridStates();
        movePopulation();

        for (Agent agent : gridPopulation.getAgents()) {
            if (agent.getNumState() == State.S) {
                agent.infection(probabilityToGetInfectedAt(agent.getPosition().getX(), agent.getPosition().getY()));
            } else {
                agent.checkState();
            }
            agent.increaseTimeInState();

            State s = agent.getNumState();
            switch (s) {
                case S:
                    states[0]++;
                    break;
                case E:
                    states[1]++;
                    break;
                case I:
                    states[2]++;
                    break;
                case R:
                    states[3]++;
                    break;
                default:
                    break;
            }
        }
    }

    public double probabilityToGetInfectedAt(int x, int y) {
        int infectiousNeighbors = 0;
        int neighborX, neighborY;

        double probability;

        for (int dx = -1; dx <= 1; dx++) {
            for (int dy = -1; dy <= 1; dy++) {
                neighborX = (x + dx + width) % width;
                neighborY = (y + dy + height) % height;

                if (neighborX >= 0 && neighborX < width && neighborY >= 0 && neighborY < height) {
                    for (Agent agent : zone.get(neighborX).get(neighborY)) {
                        if (agent != zone.get(x).get(y).get(0) && agent.getNumState() == State.I) {
                            infectiousNeighbors++;
                        }
                    }
                }
            }
        }

        probability = 1.0 - Math.exp(-0.5 * infectiousNeighbors);

        return probability;
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public int getState(int index) {
        if (index >= 0 && index < NB_STATES) {
            return states[index];
        } else {
            return -1;
        }
    }
}