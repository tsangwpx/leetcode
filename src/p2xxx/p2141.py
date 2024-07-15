from leetcode_prelude import *


# Problem 2141
class Solution:
    def maxRunTime(self, n: int, batteries: List[int]) -> int:
        batteries.sort()
        total = sum(batteries)

        while batteries[-1] > total // n:
            total -= batteries.pop()
            n -= 1

        return total // n
