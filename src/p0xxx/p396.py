from leetcode_prelude import *


# Problem 396
class Solution:
    def maxRotateFunction(self, nums: List[int]) -> int:
        n = len(nums)
        num_sum = sum(nums)
        f0 = sum(i * s for i, s in enumerate(nums))
        best = f0

        for i in range(n - 1, 0, -1):
            f0 = f0 + num_sum - n * nums[i]
            best = max(best, f0)

        return best
