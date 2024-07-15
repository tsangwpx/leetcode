from leetcode_prelude import *


# Problem 3191
class Solution:
    def minOperations(self, nums: List[int]) -> int:
        ops = 0

        for i in range(0, len(nums) - 2):
            first = nums[i]
            if not first:
                ops += 1
                nums[i + 1] = not nums[i + 1]
                nums[i + 2] = not nums[i + 2]

        return ops if nums[-1] and nums[-2] else -1
