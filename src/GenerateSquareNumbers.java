import java.util.stream.IntStream;

public class GenerateSquareNumbers {
    public static int[] run(int size) {
        return IntStream.range(0, size + 1)
                .map(number -> number * number)
                .toArray();
    }
}
