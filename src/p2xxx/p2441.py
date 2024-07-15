from leetcode_prelude import *


# Problem 2441
class Solution:
    def findMaxK(self, nums: List[int]) -> int:
        negative = set([s for s in nums if s < 0])
        maximum = -1

        for number in nums:
            if number > maximum and -number in negative:
                maximum = number

        return maximum
