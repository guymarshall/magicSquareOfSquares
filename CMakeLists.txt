cmake_minimum_required(VERSION 3.27)
project(magicSquare)

set(CMAKE_CXX_STANDARD 23)

add_executable(magicSquare main.cpp
        sums_are_equal.h
        generate_square_numbers.h
        generate_unfiltered_indices.h
        filter_indices.h
        generate_combinations.h
        process_combinations.h)
