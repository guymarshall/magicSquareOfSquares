import java.util.ArrayList;
import java.util.Arrays;
import java.util.stream.IntStream;

public class Main {
    public static final int LIMIT = 1000;

    public static void main(String[] args) {
        final int[] SQUARE_NUMBERS = IntStream.range(0, LIMIT).map(i -> (i + 1) * (i + 1)).toArray();

        // final int mostFrequentTotal = Square.getMostFrequentTotal(SQUARE_NUMBERS);
        final int mostFrequentTotal = 1003001;

        System.out.printf("The most frequent total is %d%n", mostFrequentTotal);

        final int TOTAL_ITERATIONS = LIMIT * LIMIT * LIMIT;
        int currentIteration = 0;
        ArrayList<int[]> tripletsThatMakeTotal = new ArrayList<>();

        for (int first : SQUARE_NUMBERS) {
            for (int second : SQUARE_NUMBERS) {
                for (int third : SQUARE_NUMBERS) {
                    final int total = first + second + third;

                    if (total == mostFrequentTotal) {
                        tripletsThatMakeTotal.add(new int[] { first, second, third });
                    }

                    currentIteration++;
                    final double progress = ((double) currentIteration / (double) TOTAL_ITERATIONS) * 100.0;

                    if (currentIteration % (TOTAL_ITERATIONS / 1000) == 0) {
                        System.out.printf("Progress: %.1f%n", progress);
                    }
                }
            }
        }

        final int count = tripletsThatMakeTotal.size();
        int index = 0;
        for (int[] topRow : tripletsThatMakeTotal) {
            for (int[] middleRow : tripletsThatMakeTotal) {
                for (int[] bottomRow : tripletsThatMakeTotal) {
                    // don't need to check row sums as they are already correct

                    // columns
                    if (topRow[0] + middleRow[0] + bottomRow[0] != mostFrequentTotal) {
                        continue;
                    }
                    if (topRow[1] + middleRow[1] + bottomRow[1] != mostFrequentTotal) {
                        continue;
                    }
                    if (topRow[2] + middleRow[2] + bottomRow[2] != mostFrequentTotal) {
                        continue;
                    }

                    // diagonals
                    if (topRow[0] + middleRow[1] + bottomRow[2] != mostFrequentTotal) {
                        continue;
                    }
                    if (topRow[2] + middleRow[1] + bottomRow[0] != mostFrequentTotal) {
                        continue;
                    }

                    final int[] MERGED_ROWS = new int[] {
                            topRow[0],
                            topRow[1],
                            topRow[2],
                            middleRow[0],
                            middleRow[1],
                            middleRow[2],
                            bottomRow[0],
                            bottomRow[1],
                            bottomRow[2]
                    };

                    if (!Square.numbersAreUnique(MERGED_ROWS)) {
                        continue;
                    }

                    System.out.println(Arrays.toString(MERGED_ROWS));
                    System.exit(0);
                }
            }

            final double progress = ((double) index / (double) count) * 100.0;

            if (index % (count / 1000) == 0) {
                System.out.printf("Progress: %.1f%n", progress);
            }

            index++;
        }
    }
}