import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class MostFrequentTotal {

    public static void main(String[] args) {
        // int number = 10;
        int number = 267;
        List<Integer> totals = new ArrayList<>();
        int mostCommonTotal;
        // int mostCommonTotalCount = 0;
        int mostCommonTotalCount = 774;

        while (true) {
            for (int first = 1; first < number; first++) {
                for (int second = 1; second < number; second++) {
                    for (int third = 1; third < number; third++) {
                        if (first != second && first != third && second != third) {
                            int total = (first * first) + (second * second) + (third * third);
                            totals.add(total);
                        }
                    }
                }
            }

            Map<Integer, Integer> counter = new HashMap<>();
            for (int total : totals) {
                counter.put(total, counter.getOrDefault(total, 0) + 1);
            }

            int currentMostCommonTotal = -1;
            int frequency = -1;
            for (Map.Entry<Integer, Integer> entry : counter.entrySet()) {
                if (entry.getValue() > frequency) {
                    currentMostCommonTotal = entry.getKey();
                    frequency = entry.getValue();
                }
            }

            if (frequency > mostCommonTotalCount) {
                mostCommonTotalCount = frequency;
                mostCommonTotal = currentMostCommonTotal;
                System.out.printf("%d: %d occurs %d times%n", number, mostCommonTotal, mostCommonTotalCount);
            }

            number++;
            totals.clear();
        }
    }
}
