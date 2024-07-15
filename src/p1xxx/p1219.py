from leetcode_prelude import *


# Problem 1219
class Solution:
    def getMaximumGold(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])

        def visit(i: int, j: int) -> int:
            if i < 0 or i >= m or j < 0 or j >= n or grid[i][j] == 0:
                return 0

            cell = grid[i][j]
            grid[i][j] = 0

            gold = cell + max(
                visit(i, j - 1),
                visit(i, j + 1),
                visit(i - 1, j),
                visit(i + 1, j),
            )

            grid[i][j] = cell

            return gold

        max_gold = 0
        for i, row in enumerate(grid):
            for j, item in enumerate(row):
                if item:
                    max_gold = max(max_gold, visit(i, j))

        return max_gold
