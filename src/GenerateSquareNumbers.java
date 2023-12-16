import java.util.ArrayList;

public class GenerateSquareNumbers {
    public static ArrayList<Integer> run(int limit) {
        ArrayList<Integer> squareNumbers = new ArrayList<>(limit);

        for (int i = 0; i < limit; i++) {
            squareNumbers.add(i * i);
        }

        return squareNumbers;
    }
}
