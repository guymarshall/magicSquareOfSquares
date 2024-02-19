public class Main
{
    public static boolean numbersAreUnique(int[] numbers)
    {
        for (int i = 0; i < 8; i++)
        {
            for (int j = i + 1; j < 9; j++)
            {
                if (numbers[i] == numbers[j])
                {
                    return false;
                }
            }
        }
        return true;
    }

    public static boolean sumsAreEqual(int[] numbers)
    {
        if ((numbers[0] + numbers[1] + numbers[2]) != (numbers[3] + numbers[4] + numbers[5]) || (numbers[3] + numbers[4] + numbers[5]) != (numbers[6] + numbers[7] + numbers[8]))
        {
            return false;
        }

        if ((numbers[6] + numbers[7] + numbers[8]) != (numbers[0] + numbers[3] + numbers[6])
            || (numbers[0] + numbers[3] + numbers[6]) != (numbers[1] + numbers[4] + numbers[7])
            || (numbers[1] + numbers[4] + numbers[7]) != (numbers[2] + numbers[5] + numbers[8]))
        {
            return false;
        }

        return (numbers[2] + numbers[5] + numbers[8]) == (numbers[0] + numbers[4] + numbers[8])
            && (numbers[0] + numbers[4] + numbers[8]) == (numbers[6] + numbers[4] + numbers[2]);
    }

    public static int[] generateSquareNumbers(int count)
    {
        final int[] numbers = new int[count];

        for (int i = 0; i < count; i++)
        {
            numbers[i] = (i + 1) * (i + 1);
        }

        return numbers;
    }

    public static void main(String[] args)
    {
        final int LIMIT = 10;
        final int LIMIT_SQUARED = LIMIT * LIMIT;
        final int[] squareNumbers = generateSquareNumbers(LIMIT);

        for (int a = 0; a < LIMIT; a++)
        {
            for (int b = 0; b < LIMIT; b++)
            {
                for (int c = 0; c < LIMIT; c++)
                {
                    for (int d = 0; d < LIMIT; d++)
                    {
                        for (int e = 0; e < LIMIT; e++)
                        {
                            for (int f = 0; f < LIMIT; f++)
                            {
                                for (int g = 0; g < LIMIT; g++)
                                {
                                    for (int h = 0; h < LIMIT; h++)
                                    {
                                        for (int i = 0; i < LIMIT; i++)
                                        {
                                            if (numbersAreUnique(new int[]{squareNumbers[a], squareNumbers[b], squareNumbers[c], squareNumbers[d], squareNumbers[e], squareNumbers[f], squareNumbers[g], squareNumbers[h], squareNumbers[i]})
                                                && sumsAreEqual(new int[]{squareNumbers[a], squareNumbers[b], squareNumbers[c], squareNumbers[d], squareNumbers[e], squareNumbers[f], squareNumbers[g], squareNumbers[h], squareNumbers[i]}))
                                            {
                                                System.out.printf("%d, %d, %d, %d, %d, %d, %d, %d, %d%n", a, b, c, d, e, f, g, h, i);
                                                System.exit(0);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            System.out.printf("%d / %d%n", squareNumbers[a], LIMIT_SQUARED);
        }
    }
}