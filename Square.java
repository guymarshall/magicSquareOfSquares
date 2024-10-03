import java.util.HashMap;
import java.util.Map;

public class Square {
    public static int getMostFrequentTotal(final int[] SQUARE_NUMBERS) {
        final int TOTAL_ITERATIONS = Main.LIMIT * Main.LIMIT * Main.LIMIT;
        int currentIteration = 0;
        HashMap<Integer, Integer> totalsAndCounts = new HashMap<Integer, Integer>();

        for (int first : SQUARE_NUMBERS) {
            for (int second : SQUARE_NUMBERS) {
                for (int third : SQUARE_NUMBERS) {
                    final int total = first + second + third;

                    totalsAndCounts.put(total, totalsAndCounts.getOrDefault(total, 0) + 1);

                    currentIteration++;
                    final double PROGRESS = ((double) currentIteration / (double) TOTAL_ITERATIONS) * 100.0;

                    if (currentIteration % (TOTAL_ITERATIONS / 1000) == 0) {
                        System.out.printf("Progress: %.1f%n", PROGRESS);
                    }
                }
            }
        }

        Map.Entry<Integer, Integer> maxEntry = totalsAndCounts.entrySet()
                .stream()
                .max(Map.Entry.comparingByValue())
                .get();

        final int TOTAL_WITH_HIGHEST_COUNT = maxEntry.getKey();

        return TOTAL_WITH_HIGHEST_COUNT;
    }

    public static boolean numbersAreUnique(final int[] NUMBERS) {
        return NUMBERS[0] != NUMBERS[1] && NUMBERS[0] != NUMBERS[2] && NUMBERS[0] != NUMBERS[3]
                && NUMBERS[0] != NUMBERS[4] && NUMBERS[0] != NUMBERS[5] && NUMBERS[0] != NUMBERS[6]
                && NUMBERS[0] != NUMBERS[7] && NUMBERS[0] != NUMBERS[8] && NUMBERS[1] != NUMBERS[2]
                && NUMBERS[1] != NUMBERS[3] && NUMBERS[1] != NUMBERS[4] && NUMBERS[1] != NUMBERS[5]
                && NUMBERS[1] != NUMBERS[6] && NUMBERS[1] != NUMBERS[7] && NUMBERS[1] != NUMBERS[8]
                && NUMBERS[2] != NUMBERS[3] && NUMBERS[2] != NUMBERS[4] && NUMBERS[2] != NUMBERS[5]
                && NUMBERS[2] != NUMBERS[6] && NUMBERS[2] != NUMBERS[7] && NUMBERS[2] != NUMBERS[8]
                && NUMBERS[3] != NUMBERS[4] && NUMBERS[3] != NUMBERS[5] && NUMBERS[3] != NUMBERS[6]
                && NUMBERS[3] != NUMBERS[7] && NUMBERS[3] != NUMBERS[8] && NUMBERS[4] != NUMBERS[5]
                && NUMBERS[4] != NUMBERS[6] && NUMBERS[4] != NUMBERS[7] && NUMBERS[4] != NUMBERS[8]
                && NUMBERS[5] != NUMBERS[6] && NUMBERS[5] != NUMBERS[7] && NUMBERS[5] != NUMBERS[8]
                && NUMBERS[6] != NUMBERS[7] && NUMBERS[6] != NUMBERS[8] && NUMBERS[7] != NUMBERS[8];
    }
}
