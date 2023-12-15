#include <array>
#include <iostream>
#include <numeric>
#include <vector>

bool sums_are_equal(const std::vector<int32_t>& numbers) {
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

std::vector<int32_t> generate_square_numbers(const int32_t limit) {
    std::vector<int32_t> square_numbers(limit + 1);
    std::iota(square_numbers.begin(), square_numbers.end(), 0);

    std::transform(square_numbers.begin(), square_numbers.end(), square_numbers.begin(),
                   [](const int32_t num) { return num * num; });

    return square_numbers;
}

std::vector<std::vector<int32_t>> generate_unfiltered_indices() {
    std::vector<std::vector<int32_t>> unfiltered_indices;

    std::vector<int32_t> indices(9);
    std::iota(indices.begin(), indices.end(), 0);

    do {
        unfiltered_indices.push_back(indices);
    } while (std::next_permutation(indices.begin(), indices.end()));

    return unfiltered_indices;
}

bool should_keep_permutation(const std::vector<int32_t>& indices, const std::array<std::array<int32_t, 9>, 7>& permutations_to_ignore) {
    return std::all_of(permutations_to_ignore.begin(), permutations_to_ignore.end(), [&](const std::array<int32_t, 9>& permutation_to_ignore) {
        return std::mismatch(indices.begin(), indices.end(), permutation_to_ignore.begin()).first == indices.end();
    });
}

std::vector<std::vector<int32_t>> filter_indices(const std::vector<std::vector<int32_t>>& unfiltered_indices, const std::array<std::array<int32_t, 9>, 7>& permutations_to_ignore) {
    std::vector<std::vector<int32_t>> indices;
    for (const auto& indices_set : unfiltered_indices) {
        if (should_keep_permutation(indices_set, permutations_to_ignore)) {
            indices.push_back(indices_set);
        }
    }
    return indices;
}

std::vector<std::vector<int32_t>> generate_combinations(const std::vector<int32_t>& square_numbers, const int32_t combination_size) {
    std::vector<bool> bitmask(square_numbers.size(), false);
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

void process_combinations(const std::vector<std::vector<int32_t>>& combinations, const std::vector<std::vector<int32_t>>& indices) {
#pragma omp parallel for
    for (const auto & combination : combinations) {
        for (const auto& index : indices) {
            std::vector<int32_t> numbers;
            numbers.reserve(index.size());
            for (const int32_t idx : index) {
                numbers.push_back(combination[idx]);
            }

            if (sums_are_equal(numbers)) {
                // Critical section to ensure synchronized output
                #pragma omp critical
                {
                    for (const int32_t num : numbers) {
                        std::cout << num << " ";
                    }
                    std::cout << std::endl;
                }
            }
        }
    }
}

int main() {
    constexpr int32_t limit = 20;

    const std::vector<int32_t> square_numbers = generate_square_numbers(limit);

     // generate 9! permutations
     // then for every iteration, plug in a different combination of indices
     // for the squares vector

    const std::vector<std::vector<int32_t>> unfiltered_indices = generate_unfiltered_indices();

    const std::array<std::array<int32_t, 9>, 7> permutations_to_ignore = {{
        // rotation
        {6, 3, 0, 7, 4, 1, 8, 5, 2},
        {8, 7, 6, 5, 4, 3, 2, 1, 0},
        {2, 5, 8, 1, 4, 7, 0, 3, 6},
        // flip nw-se
        {0, 3, 6, 1, 4, 7, 2, 5, 8},
        // flip sw-ne
        {8, 5, 2, 7, 4, 1, 6, 3, 0},
        // flip n-s
        {2, 1, 0, 5, 4, 3, 8, 7, 6},
        // flip w-e
        {6, 7, 8, 3, 4, 5, 0, 1, 2},
    }};

    // for every unfiltered_index in unfiltered_indices, remove any permutations that match the permutations_to_ignore array
    const std::vector<std::vector<int32_t>> indices = filter_indices(unfiltered_indices, permutations_to_ignore);

    const std::vector<std::vector<int32_t>> combinations = generate_combinations(square_numbers, 9);

    process_combinations(combinations, indices);

    return 0;
}
