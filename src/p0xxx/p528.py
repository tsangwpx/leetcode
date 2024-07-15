from leetcode_prelude import *


# Problem 528
class Solution:
    def __init__(self, w: List[int]):
        from itertools import accumulate
        from random import Random

        self._rand = Random()
        self._aw = list(accumulate(w))
        self._total = self._aw[-1]

    def pickIndex(self) -> int:
        from bisect import bisect_right

        value = self._rand.randrange(self._total)
        index = bisect_right(self._aw, value)

        return index


# Your Solution object will be instantiated and called as such:
# obj = Solution(w)
# param_1 = obj.pickIndex()
