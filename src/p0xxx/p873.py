from leetcode_prelude import *


# Problem 873
class Solution:
    def lenLongestFibSubseq(self, arr: List[int]) -> int:
        from collections import defaultdict

        table = defaultdict(int)
        seen = set()

        longest = 0

        for idx in range(len(arr)):
            number = arr[idx]

            prev = 0

            for prev in range(idx):
                left = arr[prev]
                right = number - arr[prev]

                if left >= right:
                    # Since arr is strictly increasing, it is impossible to have equality.
                    # If the question is changed, modify "seen" to count occurrences.
                    break

                if right not in seen:
                    continue

                # here left < right and left + right = number
                # print(f"{number} = {left} + {right}")

                step = table.get((left, right))
                step = 3 if step is None else step + 1

                # (right, number) must be new so directly insert the value instead of max
                table[(right, number)] = step
                longest = max(longest, step)

            seen.add(number)

        return longest

    def lenLongestFibSubseq2(self, arr: List[int]) -> int:
        from collections import defaultdict

        table = defaultdict(lambda: defaultdict(int))

        longest = 1

        for idx in range(len(arr)):
            number = arr[idx]

            best = 1
            prev = 0

            while prev < idx:
                left = arr[prev]
                right = number - arr[prev]

                if left >= right:
                    break

                if right in table:
                    if left in table[right]:
                        length = table[right][left] + 1
                    else:
                        length = 3

                    table[number][right] = max(table[number][right], length)
                    best = max(best, length)

                    # print(f"{number} = {left} + {right}")

                prev += 1

            table.setdefault(number, defaultdict(int))
            # print(idx, number, best)
            longest = max(longest, best)

        if longest == 1:
            return 0
        else:
            return longest
