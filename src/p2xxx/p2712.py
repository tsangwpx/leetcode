from leetcode_prelude import *


# Problem 2712
class Solution:
    def minimumCost(self, s: str) -> int:
        res = 0
        size = len(s)

        for i in range(1, size):
            if s[i] != s[i - 1]:
                res += min(i, size - i)

        return res
