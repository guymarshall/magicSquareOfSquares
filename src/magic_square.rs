#[derive(Debug)]
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

    fn nw_se_sum(&self) -> i32 {
        self.number1 + self.number5 + self.number9
    }

    fn sw_ne_sum(&self) -> i32 {
        self.number7 + self.number5 + self.number3
    }

    pub fn sums_are_equal(&self) -> bool {
        let top_row_sum = self.number1 + self.number2 + self.number3;
        let middle_row_sum = self.number4 + self.number5 + self.number6;
        let bottom_row_sum = self.number7 + self.number8 + self.number9;

        if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
            return false;
        }

        let left_column_sum = self.number1 + self.number4 + self.number7;
        let middle_column_sum = self.number2 + self.number5 + self.number8;
        let right_column_sum = self.number3 + self.number6 + self.number9;

        if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
            return false;
        }

        let nw_se_sum = self.number1 + self.number5 + self.number9;
        let sw_ne_sum = self.number7 + self.number5 + self.number3;

        right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
    }
}