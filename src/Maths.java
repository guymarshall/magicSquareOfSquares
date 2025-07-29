public class Maths {
    public static boolean hasDuplicates(int[] square) {
        for (int i = 0; i < 9; i++) {
            for (int j = i + 1; j < 9; j++) {
                if (square[i] == square[j]) {
                    return true;
                }
            }
        }

        return false;
    }

    public static boolean hasDuplicates(Square square) {
        int a0 = square.topLeft();
        int a1 = square.topMiddle();
        int a2 = square.topRight();
        int a3 = square.middleLeft();
        int a4 = square.middleMiddle();
        int a5 = square.middleRight();
        int a6 = square.bottomLeft();
        int a7 = square.bottomMiddle();
        int a8 = square.bottomRight();

        if (a0 == a1 || a0 == a2 || a0 == a3 || a0 == a4 || a0 == a5 || a0 == a6 || a0 == a7 || a0 == a8) {
            return true;
        }
        if (a1 == a2 || a1 == a3 || a1 == a4 || a1 == a5 || a1 == a6 || a1 == a7 || a1 == a8) {
            return true;
        }
        if (a2 == a3 || a2 == a4 || a2 == a5 || a2 == a6 || a2 == a7 || a2 == a8) {
            return true;
        }
        if (a3 == a4 || a3 == a5 || a3 == a6 || a3 == a7 || a3 == a8) {
            return true;
        }
        if (a4 == a5 || a4 == a6 || a4 == a7 || a4 == a8) {
            return true;
        }
        if (a5 == a6 || a5 == a7 || a5 == a8) {
            return true;
        }
        if (a6 == a7 || a6 == a8) {
            return true;
        }
        if (a7 == a8) {
            return true;
        }

        return false;
    }

    public static boolean isMagicSquare(int[] square) {
        int firstRowSum = square[0] + square[1] + square[2];
        int secondRowSum = square[3] + square[4] + square[5];
        int thirdRowSum = square[6] + square[7] + square[8];

        int firstColumnSum = square[0] + square[3] + square[6];
        int secondColumnSum = square[1] + square[4] + square[7];
        int thirdColumnSum = square[2] + square[5] + square[8];

        int nwSeSum = square[0] + square[4] + square[8];
        int neSwSum = square[2] + square[4] + square[6];

        return
                (secondRowSum) == firstRowSum &&
                (thirdRowSum) == firstRowSum &&
                (firstColumnSum) == firstRowSum &&
                (secondColumnSum) == firstRowSum &&
                (thirdColumnSum) == firstRowSum &&
                (nwSeSum) == firstRowSum &&
                (neSwSum) == firstRowSum;
    }

    public static boolean isMagicSquare(Square square) {
        int firstRowSum = square.topLeft() + square.topMiddle() + square.topRight();
        int secondRowSum = square.middleLeft() + square.middleMiddle() + square.middleRight();
        int thirdRowSum = square.bottomLeft() + square.bottomMiddle() + square.bottomRight();

        int firstColumnSum = square.topLeft() + square.middleLeft() + square.bottomLeft();
        int secondColumnSum = square.topMiddle() + square.middleMiddle() + square.bottomMiddle();
        int thirdColumnSum = square.topRight() + square.middleRight() + square.bottomRight();

        int nwSeSum = square.topLeft() + square.middleMiddle() + square.bottomRight();
        int neSwSum = square.topRight() + square.middleMiddle() + square.bottomLeft();

        return
                (secondRowSum) == firstRowSum &&
                (thirdRowSum) == firstRowSum &&
                (firstColumnSum) == firstRowSum &&
                (secondColumnSum) == firstRowSum &&
                (thirdColumnSum) == firstRowSum &&
                (nwSeSum) == firstRowSum &&
                (neSwSum) == firstRowSum;
    }
}
