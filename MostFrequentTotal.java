import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;

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

            Map<Integer, Integer> counter = totals.stream()
                .collect(Collectors.toMap(
                    Function.identity(),
                    v -> 1,
                    Integer::sum
                ));

                Map.Entry<Integer, Integer> mostCommonEntry = counter.entrySet()
                .stream()
                .max(Map.Entry.comparingByValue())
                .orElseThrow(() -> new IllegalStateException("Counter is empty"));

            int currentMostCommonTotal = mostCommonEntry.getKey();
            int frequency = mostCommonEntry.getValue();

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
