#ifndef GENERATE_SQUARE_NUMBERS_H
#define GENERATE_SQUARE_NUMBERS_H
#include <numeric>
#include <vector>

inline std::vector<int32_t> generate_square_numbers(const int32_t limit) {
    std::vector<int32_t> square_numbers(limit + 1);
    std::iota(square_numbers.begin(), square_numbers.end(), 0);

    std::transform(square_numbers.begin(), square_numbers.end(), square_numbers.begin(),
                   [](const int32_t num) { return num * num; });

    return square_numbers;
}

#endif //GENERATE_SQUARE_NUMBERS_H
