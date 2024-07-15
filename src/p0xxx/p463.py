from leetcode_prelude import *


# Problem 463
class Solution:
    def islandPerimeter(self, grid: List[List[int]]) -> int:
        n = len(grid)
        m = len(grid[0])

        perimeter = 0

        WATER = 0
        LAND = 1

        for i, row in enumerate(grid):
            for j, cell in enumerate(row):
                if cell != LAND:
                    continue

                if i == 0 or grid[i - 1][j] == WATER:
                    perimeter += 1

                if i + 1 == n or grid[i + 1][j] == WATER:
                    perimeter += 1

                if j == 0 or grid[i][j - 1] == WATER:
                    perimeter += 1

                if j + 1 == m or grid[i][j + 1] == WATER:
                    perimeter += 1

        return perimeter
