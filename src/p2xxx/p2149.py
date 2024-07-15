from leetcode_prelude import *


# Problem 2149
class Solution:
    def rearrangeArray(self, nums: List[int]) -> List[int]:
        res = [0] * len(nums)
        pi = 0
        ni = 1
        for number in nums:
            if number > 0:
                res[pi] = number
                pi += 2
            else:
                res[ni] = number
                ni += 2

        return res
