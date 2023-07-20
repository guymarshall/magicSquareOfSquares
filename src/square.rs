struct Square {
    numbers: [i32; 9]
}

impl Square {
    fn new(numbers: [i32; 9]) -> Square {
        Square {numbers}
    }

    fn rotate_square(&self) -> [i32; 9] {
        let mut rotated_array: [i32; 9] = [0; 9];

        rotated_array[0] = self.numbers[2];
        rotated_array[1] = self.numbers[5];
        rotated_array[2] = self.numbers[8];
        rotated_array[3] = self.numbers[1];
        // rotated_array[4] stays the same (middle)
        rotated_array[5] = self.numbers[7];
        rotated_array[6] = self.numbers[0];
        rotated_array[7] = self.numbers[4];
        rotated_array[8] = self.numbers[6];
    }
}