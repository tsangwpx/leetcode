from leetcode_prelude import *


# Problem 1289
class Solution:
    def minFallingPathSum(self, grid: List[List[int]]) -> int:
        n = len(grid)

        # if n == 1:
        #     return grid[0][0]

        indices = list(range(n))

        # initial case need at least two different indices
        dp0 = [(0, 0), (0, 1)]
        dp1 = []

        for i in range(n):
            dp1.clear()

            row = grid[i]
            indices.sort(key=row.__getitem__)

            # Pick the three smallest numbers
            # because the minimum may be blocked by the previous row and the next row
            # we should have three candidates
            for idx in indices[0:3]:
                for prev, prev_idx in dp0:
                    if prev_idx != idx:
                        break
                else:
                    raise AssertionError()

                dp1.append((row[idx] + prev, idx))

            dp1.sort()
            dp0, dp1 = dp1, dp0
            # print(dp0)

        return min(dp0)[0]
