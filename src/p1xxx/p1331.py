from leetcode_prelude import *


# Problem 1331
class Solution:
    def arrayRankTransform(self, arr: List[int]) -> List[int]:
        table = {number: rank for rank, number in enumerate(sorted(set(arr)), 1)}
        return [table[s] for s in arr]
