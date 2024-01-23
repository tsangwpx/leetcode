from leetcode_prelude import *


# Problem 645
class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        seen = set()
        res = []

        for number in nums:
            if number in seen:
                res.append(number)
            seen.add(number)

        for number in range(1, len(nums) + 1):
            if number not in seen:
                res.append(number)

        return res
