import java.util.ArrayList;
import java.util.Arrays;

public class FilterIndices {
    private static boolean isSamePermutation(ArrayList<Integer> indices, int[] permutationToIgnore) {
        // Convert lists to arrays for use with Arrays.mismatch
        Integer[] indicesArray = indices.toArray(new Integer[0]);
        Integer[] ignoreArray = permutationToIgnore.toArray(new Integer[0]);

        // Check if the permutations match using Arrays.mismatch
        return Arrays.mismatch(indicesArray, ignoreArray) == -1;
    }

    public static boolean shouldKeepPermutation(ArrayList<Integer> indices, int[][] permutationsToIgnore) {
        for (int[] permutationToIgnore : permutationsToIgnore) {
            if (permutationToIgnore.length != indices.size()) {
                continue; // Ignore permutations of different sizes
            }

            if (isSamePermutation(indices, permutationToIgnore)) {
                return false; // Permutation matches, should be ignored
            }
        }
        return true; // No matching permutation found, should keep it
    }

    public static ArrayList<ArrayList<Integer>> run(ArrayList<ArrayList<Integer>> unfilteredIndices, int[][] permutationsToIgnore) {
        ArrayList<ArrayList<Integer>> indices = new ArrayList<>();

        for (ArrayList<Integer> indicesSet : unfilteredIndices) {
            if (shouldKeepPermutation(indicesSet, permutationsToIgnore)) {
                indices.add(indicesSet);
            }
        }

        return indices;
    }
}
