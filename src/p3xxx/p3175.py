from leetcode_prelude import *


# Problem 3175
class Solution:
    def findWinningPlayer(self, skills: List[int], k: int) -> int:
        n = len(skills)
        winner = 0
        count = 0

        for idx in range(1, n):
            if skills[winner] > skills[idx]:
                count += 1
            else:
                winner = idx
                count = 1

            if count >= k:
                return winner

        return winner
