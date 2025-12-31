from leetcode_prelude import *


# Problem 3151
class Solution:
    def isArraySpecial(self, nums: List[int]) -> bool:
        for a, b in zip(nums[:-1], nums[1:]):
            if (a & 1) == (b & 1):
                return False

        return True
