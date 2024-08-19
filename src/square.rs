#[inline(always)]
pub(crate) fn numbers_are_unique(numbers: [usize; 9]) -> bool {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] == numbers[j] {
                return false;
            }
        }
    }
    true
}

#[inline(always)]
pub(crate) fn sums_are_equal(numbers: [usize; 9]) -> bool {
    if (numbers[0] + numbers[1] + numbers[2]) != (numbers[3] + numbers[4] + numbers[5])
        || (numbers[3] + numbers[4] + numbers[5]) != (numbers[6] + numbers[7] + numbers[8])
    {
        return false;
    }

    if (numbers[6] + numbers[7] + numbers[8]) != (numbers[0] + numbers[3] + numbers[6])
        || (numbers[0] + numbers[3] + numbers[6]) != (numbers[1] + numbers[4] + numbers[7])
        || (numbers[1] + numbers[4] + numbers[7]) != (numbers[2] + numbers[5] + numbers[8])
    {
        return false;
    }

    (numbers[2] + numbers[5] + numbers[8]) == (numbers[0] + numbers[4] + numbers[8])
        && (numbers[0] + numbers[4] + numbers[8]) == (numbers[6] + numbers[4] + numbers[2])
}
