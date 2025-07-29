public class Main {
    public static void main(String[] args) {
        /* TODO:
        magic constant is always 3x the centre number (x)
        top left is (x - a)
        bottom right is (x + a)
        top right is (x - b)
        bottom left is (x + b)
        */

        final int LIMIT = 10;

        int[] squareNumbers = new int[LIMIT];

        for (int i = 0; i < LIMIT; i++) {
            squareNumbers[i] = (i + 1) * (i + 1);
        }

        for (int first = 0; first < LIMIT; first++) {
            for (int second = 0; second < LIMIT; second++) {
                for (int third = 0; third < LIMIT; third++) {
                    for (int fourth = 0; fourth < LIMIT; fourth++) {
                        for (int fifth = 0; fifth < LIMIT; fifth++) {
                            for (int sixth = 0; sixth < LIMIT; sixth++) {
                                for (int seventh = 0; seventh < LIMIT; seventh++) {
                                    for (int eighth = 0; eighth < LIMIT; eighth++) {
                                        for (int ninth = 0; ninth < LIMIT; ninth++) {
                                            Square square = new Square(squareNumbers[first], squareNumbers[second], squareNumbers[third], squareNumbers[fourth], squareNumbers[fifth], squareNumbers[sixth], squareNumbers[seventh], squareNumbers[eighth], squareNumbers[ninth]);

                                            if (!Maths.hasDuplicates(square)) {
//                                            if (!square.hasDuplicates()) {
                                                if (Maths.isMagicSquare(square)) {
//                                                if (square.isMagicSquare()) {
                                                    System.out.println("MAGIC SQUARE FOUND");

                                                    System.out.printf("%d, %d, %d%n%d, %d, %d%n%d, %d, %d%n", square.topLeft(), square.topMiddle(), square.topRight(), square.middleLeft(), square.middleMiddle(), square.middleRight(), square.bottomLeft(), square.bottomMiddle(), square.bottomRight());

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
            }

            System.out.printf("%d/%d%n", first + 1, LIMIT);
        }

        System.out.println("No magic square found for limit " + LIMIT);
    }
}
