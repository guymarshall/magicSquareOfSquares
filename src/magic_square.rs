pub struct MagicSquare {
    number1: i32,
    number2: i32,
    number3: i32,
    number4: i32,
    number5: i32,
    number6: i32,
    number7: i32,
    number8: i32,
    number9: i32
}

impl MagicSquare {
    pub fn new(number1: i32, number2: i32, number3: i32, number4: i32, number5: i32, number6: i32, number7: i32, number8: i32, number9: i32) -> MagicSquare {
        MagicSquare {
            number1: number1 * number1,
            number2: number2 * number2,
            number3: number3 * number3,
            number4: number4 * number4,
            number5: number5 * number5,
            number6: number6 * number6,
            number7: number7 * number7,
            number8: number8 * number8,
            number9: number9 * number9
        }
    }

    fn top_row_sum(&self) -> i32 {
        self.number1 + self.number2 + self.number3
    }

    fn middle_row_sum(&self) -> i32 {
        self.number4 + self.number5 + self.number6
    }

    fn bottom_row_sum(&self) -> i32 {
        self.number7 + self.number8 + self.number9
    }

    fn left_column_sum(&self) -> i32 {
        self.number1 + self.number4 + self.number7
    }

    fn middle_column_sum(&self) -> i32 {
        self.number2 + self.number5 + self.number8
    }

    fn right_column_sum(&self) -> i32 {
        self.number3 + self.number6 + self.number9
    }
}