#[derive(Debug)]
#[derive(Clone)]
pub struct MagicSquare {
    pub(crate) numbers: [i32; 9]
}

impl MagicSquare {
    pub fn new(numbers: [i32; 9]) -> MagicSquare {
        MagicSquare {
            numbers: [
                numbers[0] * numbers[0],
                numbers[1] * numbers[1],
                numbers[2] * numbers[2],
                numbers[3] * numbers[3],
                numbers[4] * numbers[4],
                numbers[5] * numbers[5],
                numbers[6] * numbers[6],
                numbers[7] * numbers[7],
                numbers[8] * numbers[8]
            ]
        }
    }

    pub fn set(&mut self, numbers: [i32; 9]) {
        self.numbers[0] = numbers[0] * numbers[0];
        self.numbers[1] = numbers[1] * numbers[1];
        self.numbers[2] = numbers[2] * numbers[2];
        self.numbers[3] = numbers[3] * numbers[3];
        self.numbers[4] = numbers[4] * numbers[4];
        self.numbers[5] = numbers[5] * numbers[5];
        self.numbers[6] = numbers[6] * numbers[6];
        self.numbers[7] = numbers[7] * numbers[7];
        self.numbers[8] = numbers[8] * numbers[8];
    }

    pub fn sums_are_equal(&self) -> bool {
        let top_row_sum: i32 = self.numbers[0] + self.numbers[1] + self.numbers[2];
        let middle_row_sum: i32 = self.numbers[3] + self.numbers[4] + self.numbers[5];
        let bottom_row_sum: i32 = self.numbers[6] + self.numbers[7] + self.numbers[8];

        if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
            return false;
        }

        let left_column_sum: i32 = self.numbers[0] + self.numbers[3] + self.numbers[6];
        let middle_column_sum: i32 = self.numbers[1] + self.numbers[4] + self.numbers[7];
        let right_column_sum: i32 = self.numbers[2] + self.numbers[5] + self.numbers[8];

        if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
            return false;
        }

        let nw_se_sum: i32 = self.numbers[0] + self.numbers[4] + self.numbers[8];
        let sw_ne_sum: i32 = self.numbers[6] + self.numbers[4] + self.numbers[2];

        right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
    }
}