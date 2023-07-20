struct Square {
    numbers: [i32; 10],
    limit: i32
}

impl Square {
    fn new(numbers: [i32; 10], limit: i32) -> Square {
        Square {numbers, limit}
    }

    fn rotate_square(&mut self) {
        self.numbers[10] = self.numbers[0];
        self.numbers.swap(0, 6);
        self.numbers.swap(6, 8);
        self.numbers.swap(8, 2);
        self.numbers.swap(2, 10);
        
        self.numbers[10] = self.numbers[1];
        self.numbers.swap(1, 3);
        self.numbers.swap(3, 7);
        self.numbers.swap(7, 5);
        self.numbers.swap(5, 10);
    }
}