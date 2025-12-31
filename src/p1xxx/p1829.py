from leetcode_prelude import *


# Problem 1829
class Solution:
    def getMaximumXor(self, nums: List[int], maximumBit: int) -> List[int]:
        """
        Observations:
        1. the prefix XOR can be computed in reverse order
        2. k can be chosen to maximize the lowest `maximumBit` bits of the XOR result
        """

        res = []
        acc = 0

        # mask of the lowest bits
        mask = (1 << maximumBit) - 1

        for number in nums:
            acc ^= number
            k = (acc & mask) ^ mask
            res.append(k)

        res.reverse()

        return res
