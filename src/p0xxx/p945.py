from leetcode_prelude import *


# Problem 945
class Solution:
    def minIncrementForUnique(self, nums: List[int]) -> int:
        nums.sort()

        increment = 0

        for idx in range(1, len(nums)):
            delta = nums[idx] - nums[idx - 1]
            if delta >= 1:
                continue
            delta = -delta + 1
            nums[idx] += delta
            increment += delta

        return increment
