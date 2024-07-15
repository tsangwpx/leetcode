from leetcode_prelude import *


# Problem 523
class Solution:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        acc = 0

        remainders = {}
        remainders[0] = -1

        for pos, number in enumerate(nums):
            acc += number
            acc %= k

            last = remainders.setdefault(acc, pos)
            if pos - last >= 2:
                return True

        return False
