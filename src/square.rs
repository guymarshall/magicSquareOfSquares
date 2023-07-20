struct Square {
    numbers: [i32; 9],
    limit: i32
}

impl Square {
    fn new(numbers: [i32; 9], limit: i32) -> Square {
        Square {numbers, limit}
    }

    fn rotate_square(&mut self) {
        self.numbers[9] = self.numbers[0];
        self.numbers.swap(0, 6);
        self.numbers.swap(6, 8);
        self.numbers.swap(8, 2);
        self.numbers.swap(2, 9);
        
        self.numbers[9] = self.numbers[1];
        self.numbers.swap(1, 3);
        self.numbers.swap(3, 7);
        self.numbers.swap(7, 5);
        self.numbers.swap(5, 9);
    }

    fn increment(&mut self) {
        // for all numbers in reverse from index 8 to 0
        // if number == LIMIT, set to 0, then increment next number
        // if number < LIMIT, increment

        (8..0).into_iter().for_each(|number| {
            if self.numbers[number] == self.limit {
                self.numbers[number] = 0;

                if number == 0 {
                    return;
                }

                self.numbers[number - 1] += 1;
                // NOT FINISHED
            }
        });
    }
}