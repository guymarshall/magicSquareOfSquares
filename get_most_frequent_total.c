#include <stdio.h>
#include <stdlib.h>

#define LIMIT 1000

void generate_permutation_totals(int *totals, const int *SQUARE_NUMBERS)
{
    int index = 0;

    for (int i = 0; i < LIMIT - 2; ++i)
    {
        for (int j = i + 1; j < LIMIT - 1; ++j)
        {
            for (int k = j + 1; k < LIMIT; ++k)
            {
                totals[index++] = SQUARE_NUMBERS[i] + SQUARE_NUMBERS[j] + SQUARE_NUMBERS[k];
            }
        }
    }
}

int main(void)
{
    int square_numbers[LIMIT];

    for (int i = 0; i < LIMIT; i++)
    {
        square_numbers[i] = (i + 1) * (i + 1);
    }

    const int PERMUTATION_SIZE = LIMIT * (LIMIT - 1) * (LIMIT - 2) / 6;
    int *totals = malloc(PERMUTATION_SIZE * sizeof(totals));
    if (totals == NULL)
    {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    generate_permutation_totals(totals, square_numbers);

    setbuf(stdout, NULL);

    int most_frequent_total = totals[0];
    int current_count = 0;
    int largest_count = 0;

    for (int total_index = 0; total_index < PERMUTATION_SIZE; total_index++)
    {
        int total = totals[total_index];

        for (int first_index = 0; first_index < LIMIT; first_index++)
        {
            int first = square_numbers[first_index];

            for (int second_index = 0; second_index < LIMIT; second_index++)
            {
                int second = square_numbers[second_index];

                if (second == first)
                {
                    continue;
                }

                for (int third_index = 0; third_index < LIMIT; third_index++)
                {
                    int third = square_numbers[third_index];

                    if (third == first)
                    {
                        continue;
                    }

                    if (first + second + third == total)
                    {
                        current_count++;
                    }

                    if (current_count > largest_count)
                    {
                        most_frequent_total = total;
                        largest_count = current_count;
                    }
                }
            }
        }

        current_count = 0;

        printf("\r%d / %d (most frequent: %d with count of %d)", total_index + 1, PERMUTATION_SIZE, most_frequent_total, largest_count);
    }

    printf("\nMost frequent total: %d\n", most_frequent_total);

    free(totals);
    return 0;
}
