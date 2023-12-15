#include <array>
#include <iostream>
#include <numeric>
#include <vector>

bool sumsAreEqual(const std::vector<int32_t>& numbers) {
    const int32_t topRowSum = numbers[0] + numbers[1] + numbers[2];
    const int32_t middleRowSum = numbers[3] + numbers[4] + numbers[5];
    const int32_t bottomRowSum = numbers[6] + numbers[7] + numbers[8];

    if (topRowSum != middleRowSum or middleRowSum != bottomRowSum) {
        return false;
    }

    const int32_t leftColumnSum = numbers[0] + numbers[3] + numbers[6];
    const int32_t middleColumnSum = numbers[1] + numbers[4] + numbers[7];
    const int32_t rightColumnSum = numbers[2] + numbers[5] + numbers[8];

    if (bottomRowSum != leftColumnSum or leftColumnSum != middleColumnSum or middleColumnSum != rightColumnSum) {
        return false;
    }

    const int32_t nwSeSum = numbers[0] + numbers[4] + numbers[8];
    const int32_t swNeSum = numbers[6] + numbers[4] + numbers[2];

    return rightColumnSum == nwSeSum and nwSeSum == swNeSum;
}

std::vector<int32_t> generateSquareNumbers(const int32_t limit) {
    std::vector<int32_t> squareNumbers(limit + 1);
    std::iota(squareNumbers.begin(), squareNumbers.end(), 0);

    std::transform(squareNumbers.begin(), squareNumbers.end(), squareNumbers.begin(),
                   [](const int32_t num) { return num * num; });

    return squareNumbers;
}

std::vector<std::vector<int32_t>> generateUnfilteredIndices() {
    std::vector<std::vector<int32_t>> unfilteredIndices;

    std::vector<int32_t> indices(9);
    std::iota(indices.begin(), indices.end(), 0);

    do {
        unfilteredIndices.push_back(indices);
    } while (std::next_permutation(indices.begin(), indices.end()));

    return unfilteredIndices;
}

bool shouldKeepPermutation(const std::vector<int32_t>& indices, const std::array<std::array<int32_t, 9>, 7>& permutationsToIgnore) {
    return std::all_of(permutationsToIgnore.begin(), permutationsToIgnore.end(), [&](const std::array<int32_t, 9>& permutationToIgnore) {
        return std::mismatch(indices.begin(), indices.end(), permutationToIgnore.begin()).first == indices.end();
    });
}

std::vector<std::vector<int32_t>> filterIndices(const std::vector<std::vector<int32_t>>& unfilteredIndices, const std::array<std::array<int32_t, 9>, 7>& permutationsToIgnore) {
    std::vector<std::vector<int32_t>> indices;
    for (const auto& indices_set : unfilteredIndices) {
        if (shouldKeepPermutation(indices_set, permutationsToIgnore)) {
            indices.push_back(indices_set);
        }
    }
    return indices;
}

std::vector<std::vector<int32_t>> generateCombinations(const std::vector<int32_t>& squareNumbers, const int32_t combinationSize) {
    std::vector<bool> bitmask(squareNumbers.size(), false);
    std::fill_n(bitmask.begin(), combinationSize, true);

    std::vector<std::vector<int32_t>> combinations;

    do {
        std::vector<int32_t> temp;
        for (size_t i = 0; i < squareNumbers.size(); ++i) {
            if (bitmask[i]) {
                temp.push_back(squareNumbers[i]);
            }
        }
        combinations.push_back(temp);
    } while (std::next_permutation(bitmask.begin(), bitmask.end()));

    return combinations;
}

void processCombinations(const std::vector<std::vector<int32_t>>& combinations, const std::vector<std::vector<int32_t>>& indices) {
#pragma omp parallel for
    for (const auto & combination : combinations) {
        for (const auto& index : indices) {
            std::vector<int32_t> numbers;
            numbers.reserve(index.size());
            for (const int32_t idx : index) {
                numbers.push_back(combination[idx]);
            }

            if (sumsAreEqual(numbers)) {
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

    const std::vector<int32_t> squareNumbers = generateSquareNumbers(limit);

     // generate 9! permutations
     // then for every iteration, plug in a different combination of indices
     // for the squares vector

    const std::vector<std::vector<int32_t>> unfilteredIndices = generateUnfilteredIndices();

    const std::array<std::array<int32_t, 9>, 7> permutationsToIgnore = {{
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
    const std::vector<std::vector<int32_t>> indices = filterIndices(unfilteredIndices, permutationsToIgnore);

    const std::vector<std::vector<int32_t>> combinations = generateCombinations(squareNumbers, 9);

    processCombinations(combinations, indices);

    return 0;
}
