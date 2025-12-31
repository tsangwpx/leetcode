from leetcode_prelude import *


# Problem 2924
class Solution:
    def findChampion(
        self,
        n: int,
        edges: List[List[int]],
    ) -> int:
        survivors = [True] * n

        for src, dst in edges:
            survivors[dst] = False

        if survivors.count(True) != 1:
            return -1

        return next(i for i, s in enumerate(survivors) if s)
