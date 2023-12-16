import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        final int COMBINATION_SIZE = 9;

        Scanner scanner = new Scanner(System.in);
        int limit = scanner.nextInt();

        ArrayList<Integer> squareNumbers = GenerateSquareNumbers.run(limit);

        // generate 9! permutations
        // then for every iteration, plug in a different combination of indices
        // for the squares vector
        ArrayList<ArrayList<Integer>> unfilteredIndices = GenerateUnfilteredIndices.run();

        int[][] permutationsToIgnore = new int[][] {
            // rotation
            {6, 3, 0, 7, 4, 1, 8, 5, 2},
            {8, 7, 6, 5, 4, 3, 2, 1, 0},
            {2, 5, 8, 1, 4, 7, 0, 3, 6},
            // flip nw-se
            {0, 3, 6, 1, 4, 7, 2, 5, 8},
            // flip sw-ne
            {8, 5, 2, 7, 4, 1, 6, 3, 0},
            // flip n-s
            {2, 1, 0, 5, 4, 3, 8, 7, 6},
            // flip w-e
            {6, 7, 8, 3, 4, 5, 0, 1, 2},
        };

        // for every unfilteredIndex in unfilteredIndices, remove any permutations that match the permutationsToIgnore array
        ArrayList<ArrayList<Integer>> indices = FilterIndices.run(unfilteredIndices, permutationsToIgnore);

        ArrayList<ArrayList<Integer>> combinations = GenerateCombinations.run(squareNumbers, COMBINATION_SIZE);

        ProcessCombinations.run(combinations, indices);
    }
}