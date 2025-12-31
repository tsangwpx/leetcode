from leetcode_prelude import *


# Problem 1726
class Solution:
    def tupleSameProduct(self, nums: List[int]) -> int:
        from collections import defaultdict

        pairs: dict[int, int] = defaultdict(int)

        count = 0

        for i in range(0, len(nums)):
            for j in range(0, i):
                a = nums[i]
                b = nums[j]

                prod = a * b
                pc = pairs[prod]

                count += pc * 8
                pairs[prod] += 1

        return count
