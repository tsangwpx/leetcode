from leetcode_prelude import *


# Problem 506
class Solution:
    def findRelativeRanks(self, score: List[int]) -> List[str]:
        indices = list(range(len(score)))
        indices.sort(key=score.__getitem__)
        indices.reverse()

        res = [None] * len(score)

        for rank, idx in enumerate(indices, start=1):
            res[idx] = "%d" % (rank,)

        if len(indices) >= 1:
            res[indices[0]] = "Gold Medal"

        if len(indices) >= 2:
            res[indices[1]] = "Silver Medal"

        if len(indices) >= 3:
            res[indices[2]] = "Bronze Medal"

        return res
