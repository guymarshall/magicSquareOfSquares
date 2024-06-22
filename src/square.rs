pub(crate) fn numbers_are_unique(numbers: [usize; 9]) -> bool {
    numbers[0] != numbers[1] &&
        numbers[0] != numbers[2] &&
        numbers[0] != numbers[3] &&
        numbers[0] != numbers[4] &&
        numbers[0] != numbers[5] &&
        numbers[0] != numbers[6] &&
        numbers[0] != numbers[7] &&
        numbers[0] != numbers[8] &&
        numbers[1] != numbers[2] &&
        numbers[1] != numbers[3] &&
        numbers[1] != numbers[4] &&
        numbers[1] != numbers[5] &&
        numbers[1] != numbers[6] &&
        numbers[1] != numbers[7] &&
        numbers[1] != numbers[8] &&
        numbers[2] != numbers[3] &&
        numbers[2] != numbers[4] &&
        numbers[2] != numbers[5] &&
        numbers[2] != numbers[6] &&
        numbers[2] != numbers[7] &&
        numbers[2] != numbers[8] &&
        numbers[3] != numbers[4] &&
        numbers[3] != numbers[5] &&
        numbers[3] != numbers[6] &&
        numbers[3] != numbers[7] &&
        numbers[3] != numbers[8] &&
        numbers[4] != numbers[5] &&
        numbers[4] != numbers[6] &&
        numbers[4] != numbers[7] &&
        numbers[4] != numbers[8] &&
        numbers[5] != numbers[6] &&
        numbers[5] != numbers[7] &&
        numbers[5] != numbers[8] &&
        numbers[6] != numbers[7] &&
        numbers[6] != numbers[8] &&
        numbers[7] != numbers[8]
}

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