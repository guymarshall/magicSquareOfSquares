#ifndef SUMS_ARE_EQUAL_H
#define SUMS_ARE_EQUAL_H
#include <vector>

inline bool sums_are_equal(const std::vector<int32_t>& numbers) {
    const int32_t top_row_sum = numbers[0] + numbers[1] + numbers[2];
    const int32_t middle_row_sum = numbers[3] + numbers[4] + numbers[5];
    const int32_t bottom_row_sum = numbers[6] + numbers[7] + numbers[8];

    if (top_row_sum != middle_row_sum or middle_row_sum != bottom_row_sum) {
        return false;
    }

    const int32_t left_column_sum = numbers[0] + numbers[3] + numbers[6];
    const int32_t middle_column_sum = numbers[1] + numbers[4] + numbers[7];
    const int32_t right_column_sum = numbers[2] + numbers[5] + numbers[8];

    if (bottom_row_sum != left_column_sum or left_column_sum != middle_column_sum or middle_column_sum != right_column_sum) {
        return false;
    }

    const int32_t nw_se_sum = numbers[0] + numbers[4] + numbers[8];
    const int32_t sw_ne_sum = numbers[6] + numbers[4] + numbers[2];

    return right_column_sum == nw_se_sum and nw_se_sum == sw_ne_sum;
}

#endif //SUMS_ARE_EQUAL_H
