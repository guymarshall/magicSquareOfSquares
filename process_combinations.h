#ifndef PROCESS_COMBINATIONS_H
#define PROCESS_COMBINATIONS_H
#include <iostream>
#include <vector>

#include "sums_are_equal.h"

inline void process_combinations(const std::vector<std::vector<int32_t>>& combinations, const std::vector<std::vector<int32_t>>& indices) {
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

#endif //PROCESS_COMBINATIONS_H
