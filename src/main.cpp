#include <array>
#include <vector>

#include "generate_square_numbers.h"
#include "generate_unfiltered_indices.h"
#include "filter_indices.h"
#include "generate_combinations.h"
#include "process_combinations.h"

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
