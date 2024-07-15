from leetcode_prelude import *


# Problem 3068
class Solution:
    def maximumValueSum(
        self,
        nums: List[int],
        k: int,
        edges: List[List[int]],
    ) -> int:
        """
        First, edges are unused in the solution



        """
        total = 0
        positive_sum = 0
        positive_count = 0
        min_delta = 2**32

        for value in nums:
            total += value
            delta = (value ^ k) - value

            if delta > 0:
                positive_sum += delta
                positive_count += 1

            min_delta = min(min_delta, abs(delta))

        # print(total, positive_sum, positive_count, min_delta)
        if positive_count & 1:
            positive_sum -= min_delta

        return total + positive_sum
