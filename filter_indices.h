#ifndef FILTER_INDICES_H
#define FILTER_INDICES_H
#include <array>
#include <vector>

inline bool should_keep_permutation(const std::vector<int32_t>& indices, const std::array<std::array<int32_t, 9>, 7>& permutations_to_ignore) {
    return std::all_of(permutations_to_ignore.begin(), permutations_to_ignore.end(), [&](const std::array<int32_t, 9>& permutation_to_ignore) {
        return std::mismatch(indices.begin(), indices.end(), permutation_to_ignore.begin()).first == indices.end();
    });
}

inline std::vector<std::vector<int32_t>> filter_indices(const std::vector<std::vector<int32_t>>& unfiltered_indices, const std::array<std::array<int32_t, 9>, 7>& permutations_to_ignore) {
    std::vector<std::vector<int32_t>> indices;
    for (const auto& indices_set : unfiltered_indices) {
        if (should_keep_permutation(indices_set, permutations_to_ignore)) {
            indices.push_back(indices_set);
        }
    }
    return indices;
}

#endif //FILTER_INDICES_H
