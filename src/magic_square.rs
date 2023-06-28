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
        self.numbers = [
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