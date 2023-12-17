import java.util.ArrayList;
import java.util.Collections;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class GenerateUnfilteredIndices {
    private static boolean nextPermutation(ArrayList<Integer> indices) {
        int i = indices.size() - 2;
        while (i >= 0 && indices.get(i) >= indices.get(i + 1)) {
            i--;
        }
        if (i < 0) {
            return false;
        }

        int j = indices.size() - 1;
        while (indices.get(j) <= indices.get(i)) {
            j--;
        }

        Collections.swap(indices, i, j);
        Collections.reverse(indices.subList(i + 1, indices.size()));
        return true;
    }

    public static ArrayList<ArrayList<Integer>> run() {
        ArrayList<ArrayList<Integer>> unfilteredIndices = new ArrayList<>();

        ArrayList<Integer> indices = IntStream.range(0, 9)
                .parallel()
                .boxed()
                .collect(Collectors.toCollection(ArrayList::new));

        do {
            unfilteredIndices.add(new ArrayList<>(indices));
        } while (nextPermutation(indices));

        return unfilteredIndices;
    }
}
