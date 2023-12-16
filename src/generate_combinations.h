#ifndef GENERATE_COMBINATIONS_H
#define GENERATE_COMBINATIONS_H
#include <vector>

inline std::vector<std::vector<int32_t>> generate_combinations(const std::vector<int32_t>& square_numbers, const int32_t combination_size) {
    std::vector bitmask(square_numbers.size(), false);
    std::fill_n(bitmask.begin(), combination_size, true);

    std::vector<std::vector<int32_t>> combinations;

    do {
        std::vector<int32_t> temp;
        for (size_t i = 0; i < square_numbers.size(); ++i) {
            if (bitmask[i]) {
                temp.push_back(square_numbers[i]);
            }
        }
        combinations.push_back(temp);
    } while (std::next_permutation(bitmask.begin(), bitmask.end()));

    return combinations;
}

#endif //GENERATE_COMBINATIONS_H
