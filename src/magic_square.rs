#[derive(Debug)]
pub struct MagicSquare {
    number1: u32,
    number2: u32,
    number3: u32,
    number4: u32,
    number5: u32,
    number6: u32,
    number7: u32,
    number8: u32,
    number9: u32
}

impl MagicSquare {
    pub fn new(number1: u32, number2: u32, number3: u32, number4: u32, number5: u32, number6: u32, number7: u32, number8: u32, number9: u32) -> MagicSquare {
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

    pub fn sums_are_equal(&self) -> bool {
        let top_row_sum: u32 = self.number1 + self.number2 + self.number3;
        let middle_row_sum: u32 = self.number4 + self.number5 + self.number6;
        let bottom_row_sum: u32 = self.number7 + self.number8 + self.number9;

        if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
            return false;
        }

        let left_column_sum: u32 = self.number1 + self.number4 + self.number7;
        let middle_column_sum: u32 = self.number2 + self.number5 + self.number8;
        let right_column_sum: u32 = self.number3 + self.number6 + self.number9;

        if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
            return false;
        }

        let nw_se_sum: u32 = self.number1 + self.number5 + self.number9;
        let sw_ne_sum: u32 = self.number7 + self.number5 + self.number3;

        right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
    }
}