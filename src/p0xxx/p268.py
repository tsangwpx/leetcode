from leetcode_prelude import *


# Problem 268
class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        n = len(nums)
        xor = n

        for i in range(n):
            xor = xor ^ i ^ nums[i]

        return xor
