from leetcode_prelude import *


# Problem 1043
class Solution:
    def maxSumAfterPartitioning(self, arr: List[int], k: int) -> int:
        # maintain the max sum of the last k positions
        dp = [0] * k

        # k 0 1 2 3 4 5 6 7

        for i in range(len(arr)):
            # indexing
            # we are going to write at (i % k)
            # the last write index is ((i - 1) % k)

            # the max value in the partition ending here
            part_max = 0
            # the maximum sum so far
            sum_max = 0

            # when partition size is 1, value = part_max + ((i - 1) % k)
            # when partition size is 2, value = part_max * 2 + ((i - 2) % k)
            # ...
            # when partition size is k, value = part_max * k + ((i - k + 1) % k)

            for j in range(0, min(k, i + 1)):
                part_size = j + 1
                part_max = max(part_max, arr[i - j])
                sum_max = max(sum_max, dp[(i - part_size) % k] + part_max * part_size)

            dp[i % k] = sum_max

        return dp[(len(arr) - 1) % k]
