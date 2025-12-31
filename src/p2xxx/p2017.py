from leetcode_prelude import *


# Problem 2017
class Solution:
    def gridGame(self, grid: List[List[int]]) -> int:
        n = len(grid[0])
        pfxsum = [[0] * (n + 1) for _ in range(2)]
        for i in range(n):
            pfxsum[0][i + 1] = pfxsum[0][i] + grid[0][i]
            pfxsum[1][i + 1] = pfxsum[1][i] + grid[1][i]

        best_score = 10**5 * n

        for i in range(n):
            top = pfxsum[0][n] - pfxsum[0][i + 1]
            bot = pfxsum[1][i] - pfxsum[1][0]

            score = max(top, bot)

            if score < best_score:
                best_score = score

        return best_score
