#[inline(always)]
pub(crate) fn numbers_are_unique(first: &usize, second: &usize, third: &usize, fourth: &usize, fifth: &usize, sixth: &usize, seventh: &usize, eighth: &usize, ninth: &usize) -> bool {
    first != second && first != third && first != fourth && first != fifth && first != sixth && first != seventh && first != eighth && first != ninth && second != third && second != fourth && second != fifth && second != sixth && second != seventh && second != eighth && second != ninth && third != fourth && third != fifth && third != sixth && third != seventh && third != eighth && third != ninth && fourth != fifth && fourth != sixth && fourth != seventh && fourth != eighth && fourth != ninth && fifth != sixth && fifth != seventh && fifth != eighth && fifth != ninth && sixth != seventh && sixth != eighth && sixth != ninth && seventh != eighth && seventh != ninth && eighth != ninth
}

#[inline(always)]
pub(crate) fn sums_are_equal(first: &usize, second: &usize, third: &usize, fourth: &usize, fifth: &usize, sixth: &usize, seventh: &usize, eighth: &usize, ninth: &usize) -> bool {
    if (first + second + third) != (fourth + fifth + sixth) || (fourth + fifth + sixth) != (seventh + eighth + ninth) {
        return false;
    }

    if (seventh + eighth + ninth) != (first + fourth + seventh) || (first + fourth + seventh) != (second + fifth + eighth) || (second + fifth + eighth) != (third + sixth + ninth) {
        return false;
    }

    (third + sixth + ninth) == (first + fifth + ninth) && (first + fifth + ninth) == (seventh + fifth + third)
}
