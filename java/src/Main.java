import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

public class Main {

    private static final int NUM_SIMULATIONS = 100;
    private static final int NUM_DAYS = 730;

    public static void main(String[] args) {
        simulateAnEpidemic();
    }

    private static void simulateAnEpidemic() {
        Population[] populations = new Population[NUM_SIMULATIONS];
        Grid[] grids = new Grid[NUM_SIMULATIONS];
        FileWriter[] csvFiles = new FileWriter[NUM_SIMULATIONS];

        try {
            File directory = new File("src/simulation");
            if (!directory.exists()) {
                directory.mkdirs();
            }

            for (int sim = 0; sim < NUM_SIMULATIONS; sim++) {
                populations[sim] = new Population();
                populations[sim].initializePopulation();
                grids[sim] = new Grid();
                grids[sim].initializeGrid(populations[sim]);

                String filename = "src/simulation/simulation_data" + (sim + 1) + ".csv";
                csvFiles[sim] = new FileWriter(filename);
                csvFiles[sim].write("day,susceptibleCount,exposedCount,infectedCount,recoveredCount\n");
            }

            System.out.println("Populations and grids successfully initialized for " + NUM_SIMULATIONS + " simulations.");

            System.out.println("Computing simulation ...");

            for (int day = 0; day < NUM_DAYS; day++) {
                System.out.println("Day " + (day + 1) + " simulating...");

                for (int sim = 0; sim < NUM_SIMULATIONS; sim++) {
                    grids[sim].simulate();

                    int numS = grids[sim].getState(0);
                    int numE = grids[sim].getState(1);
                    int numI = grids[sim].getState(2);
                    int numR = grids[sim].getState(3);

                    csvFiles[sim].write(day + "," + numS + "," + numE + "," + numI + "," + numR + "\n");
                }
            }
        } catch (IOException e) {
            e.printStackTrace();
        } finally {
            for (FileWriter fileWriter : csvFiles) {
                if (fileWriter != null) {
                    try {
                        fileWriter.close();
                    } catch (IOException e) {
                        e.printStackTrace();
                    }
                }
            }
        }
    }
}