#ifndef GENERATE_UNFILTERED_INDICES_H
#define GENERATE_UNFILTERED_INDICES_H
#include <numeric>
#include <vector>

inline std::vector<std::vector<int32_t>> generate_unfiltered_indices() {
    std::vector<std::vector<int32_t>> unfiltered_indices;

    std::vector<int32_t> indices(9);
    std::iota(indices.begin(), indices.end(), 0);

    do {
        unfiltered_indices.push_back(indices);
    } while (std::next_permutation(indices.begin(), indices.end()));

    return unfiltered_indices;
}

#endif //GENERATE_UNFILTERED_INDICES_H
