from leetcode_prelude import *


# Problem 2418
class Solution:
    def sortPeople(self, names: List[str], heights: List[int]) -> List[str]:
        # returns name sorted in descending order by height

        pairs = sorted(zip(heights, names), reverse=True)

        return [name for _, name in pairs]
