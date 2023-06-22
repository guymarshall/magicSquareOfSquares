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
}