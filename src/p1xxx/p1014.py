from leetcode_prelude import *


# Problem 1014
class Solution:
    def maxScoreSightseeingPair(self, values: List[int]) -> int:
        """
        For i < j,
        score = values[i] + values[j] + i - j
              = (values[i] + i) + (values[j] - j)

        taking maximum
        max score = max (values[i] + i) + min (values[j] - j) given i < j

        As we traverse over the array, take the maximum of (values[i] + i)
        and use it in the next iteration for computing score

        time complexity: O(N)
        """
        partial_max = values[0] + 0
        maximum = partial_max + (values[1] - 1)

        for j in range(1, len(values)):
            number = values[j]

            maximum = max(maximum, partial_max + number - j)
            partial_max = max(partial_max, number + j)

        return maximum
