import java.util.ArrayList;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class ProcessCombinations {
    private static boolean sumsAreEqual(ArrayList<Integer> numbers) {
        int top_row_sum = numbers.get(0) + numbers.get(1) + numbers.get(2);
        int middle_row_sum = numbers.get(3) + numbers.get(4) + numbers.get(5);
        int bottom_row_sum = numbers.get(6) + numbers.get(7) + numbers.get(8);

        if (top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum) {
            return false;
        }

        int left_column_sum = numbers.get(0) + numbers.get(3) + numbers.get(6);
        int middle_column_sum = numbers.get(1) + numbers.get(4) + numbers.get(7);
        int right_column_sum = numbers.get(2) + numbers.get(5) + numbers.get(8);

        if (bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum) {
            return false;
        }

        int nw_se_sum = numbers.get(0) + numbers.get(4) + numbers.get(8);
        int sw_ne_sum = numbers.get(6) + numbers.get(4) + numbers.get(2);

        return right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum;
    }

    public static void run(ArrayList<ArrayList<Integer>> combinations, ArrayList<ArrayList<Integer>> indices) {
        ExecutorService executor = Executors.newFixedThreadPool(Runtime.getRuntime().availableProcessors());

        for (ArrayList<Integer> combination : combinations) {
            for (ArrayList<Integer> index : indices) {
                ArrayList<Integer> numbers = new ArrayList<>();
                numbers.ensureCapacity(index.size());
                for (Integer idx : index) {
                    numbers.add(combination.get(idx));
                }

                if (sumsAreEqual(numbers)) {
                    executor.execute(() -> {
                        synchronized (System.out) {
                            for (Integer num : numbers) {
                                System.out.print(num + " ");
                            }
                            System.out.println();
                        }
                    });
                }
            }
        }

        executor.shutdown();
    }
}
