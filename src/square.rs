pub struct Square<'a> {
    pub first: &'a usize,
    pub second: &'a usize,
    pub third: &'a usize,
    pub fourth: &'a usize,
    pub fifth: &'a usize,
    pub sixth: &'a usize,
    pub seventh: &'a usize,
    pub eighth: &'a usize,
    pub ninth: &'a usize,
}

impl<'a> Square<'a> {
    pub(crate) fn numbers_are_unique(&self) -> bool {
        self.first != self.second &&
            self.first != self.third &&
            self.first != self.fourth &&
            self.first != self.fifth &&
            self.first != self.sixth &&
            self.first != self.seventh &&
            self.first != self.eighth &&
            self.first != self.ninth &&
            self.second != self.third &&
            self.second != self.fourth &&
            self.second != self.fifth &&
            self.second != self.sixth &&
            self.second != self.seventh &&
            self.second != self.eighth &&
            self.second != self.ninth &&
            self.third != self.fourth &&
            self.third != self.fifth &&
            self.third != self.sixth &&
            self.third != self.seventh &&
            self.third != self.eighth &&
            self.third != self.ninth &&
            self.fourth != self.fifth &&
            self.fourth != self.sixth &&
            self.fourth != self.seventh &&
            self.fourth != self.eighth &&
            self.fourth != self.ninth &&
            self.fifth != self.sixth &&
            self.fifth != self.seventh &&
            self.fifth != self.eighth &&
            self.fifth != self.ninth &&
            self.sixth != self.seventh &&
            self.sixth != self.eighth &&
            self.sixth != self.ninth &&
            self.seventh != self.eighth &&
            self.seventh != self.ninth &&
            self.eighth != self.ninth
    }

    pub(crate) fn sums_are_equal(&self) -> bool {
        if (self.first + self.second + self.third) != (self.fourth + self.fifth + self.sixth)
            || (self.fourth + self.fifth + self.sixth) != (self.seventh + self.eighth + self.ninth)
        {
            return false;
        }

        if (self.seventh + self.eighth + self.ninth) != (self.first + self.fourth + self.seventh)
            || (self.first + self.fourth + self.seventh) != (self.second + self.fifth + self.eighth)
            || (self.second + self.fifth + self.eighth) != (self.third + self.sixth + self.ninth)
        {
            return false;
        }

        (self.third + self.sixth + self.ninth) == (self.first + self.fifth + self.ninth)
            && (self.first + self.fifth + self.ninth) == (self.seventh + self.fifth + self.third)
    }
}