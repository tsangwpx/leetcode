from leetcode_prelude import *


# Problem 2658
class Solution:
    def findMaxFish(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])

        pending = []

        max_collected = 0

        for i in range(m):
            for j in range(n):
                if grid[i][j] == 0:
                    continue

                collected = grid[i][j]
                grid[i][j] = 0
                pending.append((i, j))

                while pending:
                    x, y = pending.pop()
                    for p, q in ((x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)):
                        if not 0 <= p < m or not 0 <= q < n or grid[p][q] == 0:
                            continue
                        collected += grid[p][q]
                        grid[p][q] = 0
                        pending.append((p, q))

                max_collected = max(max_collected, collected)

        return max_collected
