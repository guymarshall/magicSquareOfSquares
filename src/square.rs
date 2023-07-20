struct Square {
    numbers: [[i32; 3]; 3]
}

impl Square {
    fn new(numbers: [[i32; 3]; 3]) -> Square {
        Square {numbers}
    }

    fn rotate(&self) {
        (0..3).iter().for_each(|i| {
            (i..3).iter().for_each(|j| {
                let temp = self.numbers[i][j];
                self.numbers[i][j] = self.numbers[j][i];
                self.numbers[j][i] = temp;
            });
        });

        (0..3).iter().for_each(|i| {
            self.numbers[i].reverse();
        });
    }
}