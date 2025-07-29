public record Square(
        int topLeft, int topMiddle, int topRight,
        int middleLeft, int middleMiddle, int middleRight,
        int bottomLeft, int bottomMiddle, int bottomRight
) {
    public boolean hasDuplicates() {
        int a0 = this.topLeft;
        int a1 = this.topMiddle;
        int a2 = this.topRight;
        int a3 = this.middleLeft;
        int a4 = this.middleMiddle;
        int a5 = this.middleRight;
        int a6 = this.bottomLeft;
        int a7 = this.bottomMiddle;
        int a8 = this.bottomRight;

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

    public boolean isMagicSquare() {
        int firstRowSum = this.topLeft() + this.topMiddle() + this.topRight();
        int secondRowSum = this.middleLeft() + this.middleMiddle() + this.middleRight();
        int thirdRowSum = this.bottomLeft() + this.bottomMiddle() + this.bottomRight();

        int firstColumnSum = this.topLeft() + this.middleLeft() + this.bottomLeft();
        int secondColumnSum = this.topMiddle() + this.middleMiddle() + this.bottomMiddle();
        int thirdColumnSum = this.topRight() + this.middleRight() + this.bottomRight();

        int nwSeSum = this.topLeft() + this.middleMiddle() + this.bottomRight();
        int neSwSum = this.topRight() + this.middleMiddle() + this.bottomLeft();

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
