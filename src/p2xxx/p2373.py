from leetcode_prelude import *


# Problem 2373
class Solution:
    def largestLocal(self, grid: List[List[int]]) -> List[List[int]]:
        n = len(grid)
        res = [[0] * (n - 2) for _ in range(n - 2)]

        for i in range(n - 2):
            r0 = grid[i]
            r1 = grid[i + 1]
            r2 = grid[i + 2]

            for j in range(n - 2):
                res[i][j] = max(
                    r0[j],
                    r0[j + 1],
                    r0[j + 2],
                    r1[j],
                    r1[j + 1],
                    r1[j + 2],
                    r2[j],
                    r2[j + 1],
                    r2[j + 2],
                )

        return res
