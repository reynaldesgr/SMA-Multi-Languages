import java.util.Random;

public class Agent 
{
    private int id;
    private State state;
    private Position position;
    private double timeInState;
    private double dE;
    private double dI;
    private double dR;

    public Agent(int id, State state, Position position, double dE, double dI, double dR) {
        this.id = id;
        this.state = state;
        this.position = position;
        this.timeInState = 0.0;
        this.dE = dE;
        this.dI = dI;
        this.dR = dR;
    }

    public Agent() {
        this.id = 0;
        this.state = State.S;
        this.position = new Position(0, 0);
        this.timeInState = 0.0;
        this.dE = 0.0;
        this.dI = 0.0;
        this.dR = 0.0;
    }

    public void infection(double probabilityToGetInfected) {
        Random random = new Random();
        double randomUniformNumber = random.nextDouble();

        if (randomUniformNumber < probabilityToGetInfected) {
            this.state = State.E;
            this.timeInState = 0.0;
        }
    }

    public void checkState() {
        switch (state) {
            case E:
                if (timeInState > dE) {
                    setState(State.I);
                    timeInState = 0.0;
                }
                break;
            case I:
                if (timeInState > dI) {
                    setState(State.R);
                    timeInState = 0.0;
                }
                break;
            case R:
                if (timeInState > dR) {
                    setState(State.S);
                    timeInState = 0;
                }
                break;
            default:
                break;
        }
    }

    public void move() {
        Random random = new Random();
        int newX = random.nextInt(Grid.GRID_WIDTH);
        int newY = random.nextInt(Grid.GRID_HEIGHT);
        Position newPos = new Position(newX, newY);
        this.setPosition(newPos);
    }

    public void increaseTimeInState() {
        this.timeInState += 1;
    }

    public void displayAgent() {
        System.out.println("Agent ID: " + getId());
        System.out.println("State: " + getCharState());
        System.out.println("Position (x, y): " + getPosition().getX() + ", " + getPosition().getY());
        System.out.println("Time in State: " + getTimeInState());
        System.out.println("dE: " + getDE() + ", dI: " + getDI() + ", dR: " + getDR());
        System.out.println();
    }
    
    public int getId() {
        return id;
    }

    public double getTimeInState() {
        return timeInState;
    }

    public char getCharState() {
        switch (state) {
            case S:
                return 'S';
            case E:
                return 'E';
            case I:
                return 'I';
            case R:
                return 'R';
            default:
                return '?';
        }
    }

    public State getNumState() {
        return state;
    }

    public Position getPosition() {
        return position;
    }

    public double getDE() {
        return dE;
    }

    public double getDI() {
        return dI;
    }

    public double getDR() {
        return dR;
    }

    public void setPosition(Position newPos) {
        this.position = newPos;
    }

    public void setState(State newState) {
        this.state       = newState;
        this.timeInState = 0.0;
    }
}