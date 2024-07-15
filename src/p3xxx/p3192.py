from leetcode_prelude import *


# Problem 3192
class Solution:
    def minOperations(self, nums: List[int]) -> int:
        ops = 0
        for i in range(len(nums)):
            item = nums[i]

            if ops % 2 == 1:
                item = not item

            if not item:
                ops += 1

        return ops
