from leetcode_prelude import *


# Problem 2275
class Solution:
    def largestCombination(self, candidates: List[int]) -> int:
        from functools import reduce
        from operator import or_

        mask = reduce(or_, candidates, 0)

        bit = 1
        ones = []

        while bit <= 10**7:
            if (bit & mask) != 0:
                ones.append(bit)
            bit <<= 1

        counts = [0] * len(ones)

        for number in candidates:
            for i, bit in enumerate(ones):
                if (number & bit) != 0:
                    counts[i] += 1

        return max(counts)
