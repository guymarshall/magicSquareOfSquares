from collections import Counter

def main():
    number = 263
    totals = []
    most_common_total = 69086
    most_common_total_count = 744

    while True:
        for first in range(1, number):
            for second in range(1, number):
                for third in range(1, number):
                    if first != second and first != third and second != third:
                        total = (first ** 2) + (second ** 2) + (third ** 2)

                        # if total not in totals:
                        totals.append(total)

        counter = Counter(totals)

        current_most_common_total, frequency = counter.most_common(1)[0]

        if frequency > most_common_total_count:
            most_common_total_count = frequency
            most_common_total = current_most_common_total
            print(f"{number}: {most_common_total} occurs {most_common_total_count} times")

        number += 1
        totals.clear()

if __name__ == "__main__":
    main()
