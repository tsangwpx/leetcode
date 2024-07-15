from leetcode_prelude import *


# Problem 3190
class Solution:
    def minimumOperations(self, nums: List[int]) -> int:
        ops = 0

        for number in nums:
            r = number % 3
            r = min(r, 3 - r)
            ops += r

        return ops
