from leetcode_prelude import *


# Problem 1863
class Solution:
    def subsetXORSum(self, nums: List[int]) -> int:
        """
        This is bit tricky.

        Basically, it inspect the representation of the final answer
        and exploit the fact that each bit is contributed equally if
        the bit is found in the power set of nums.

        See other's explanation.

        """
        from functools import reduce
        from operator import or_

        bits = reduce(or_, nums, 0)

        return bits << (len(nums) - 1)

    def subsetXORSum(self, nums: List[int]) -> int:
        """
        Since nums length is small, brute force is used.
        """
        from functools import reduce
        from itertools import combinations
        from operator import xor

        sum_ = 0

        for r in range(0, len(nums) + 1):
            for subset in combinations(nums, r):
                sum_ += reduce(xor, subset, 0)

        return sum_
