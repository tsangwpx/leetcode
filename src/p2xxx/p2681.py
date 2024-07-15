from leetcode_prelude import *


# Problem 2681
class Solution:
    def sumOfPower(self, nums: List[int]) -> int:
        nums.sort()

        MOD = 10**9 + 7

        res = 0
        h = 0

        for number in nums:
            res = (res + number**3 + number**2 * h) % MOD
            h = (h * 2 + number) % MOD

        return res
