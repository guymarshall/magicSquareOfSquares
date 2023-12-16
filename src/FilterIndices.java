import java.util.ArrayList;

public class FilterIndices {
    private static boolean shouldKeepPermutation(ArrayList<Integer> indices, int[][] permutationsToIgnore) {
        // TODO: implement
//        return std::all_of(permutationsToIgnore.begin(), permutationsToIgnore.end(), [&](const std::array<int32_t, 9>& permutation_to_ignore) {
//            return std::mismatch(indices.begin(), indices.end(), permutation_to_ignore.begin()).first == indices.end();
//        });
    }

    public static ArrayList<ArrayList<Integer>> run(ArrayList<ArrayList<Integer>> unfilteredIndices, int[][] permutationsToIgnore) {
        ArrayList<ArrayList<Integer>> indices = new ArrayList<>();

        for (ArrayList<Integer> indices_set : unfilteredIndices) {
            if (shouldKeepPermutation(indices_set, permutationsToIgnore)) {
                indices.add(indices_set);
            }
        }

        return indices;
    }
}
