from leetcode_prelude import *


# Problem 3159
class Solution:
    def occurrencesOfElement(
        self,
        nums: List[int],
        queries: List[int],
        x: int,
    ) -> List[int]:
        indices = [s for s, val in enumerate(nums) if val == x]

        res = []

        for idx in queries:
            idx -= 1
            if idx < len(indices):
                res.append(indices[idx])
            else:
                res.append(-1)

        return res
