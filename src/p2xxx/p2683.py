from leetcode_prelude import *


# Problem 2683
class Solution:
    def doesValidArrayExist(self, derived: List[int]) -> bool:
        x = 0
        for number in derived:
            x ^= number

        return x == 0
