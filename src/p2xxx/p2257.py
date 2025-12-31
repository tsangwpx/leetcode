from leetcode_prelude import *


# Problem 2257
class Solution:
    def countUnguarded(
        self,
        m: int,
        n: int,
        guards: List[List[int]],
        walls: List[List[int]],
    ) -> int:
        """
        Naive approach
        """

        EMPTY: int = 0
        GUARD: int = 1
        WALL: int = 2
        GUARDED: int = 3

        grid = [[EMPTY] * n for _ in range(m)]

        for row, col in walls:
            grid[row][col] = WALL

        for row, col in guards:
            grid[row][col] = GUARD

        for row, col in guards:
            for i in range(row + 1, m):
                if grid[i][col] in (GUARD, WALL):
                    break
                grid[i][col] = GUARDED

            for i in range(row - 1, -1, -1):
                if grid[i][col] in (GUARD, WALL):
                    break
                grid[i][col] = GUARDED

            for j in range(col + 1, n):
                if grid[row][j] in (GUARD, WALL):
                    break
                grid[row][j] = GUARDED

            for j in range(col - 1, -1, -1):
                if grid[row][j] in (GUARD, WALL):
                    break
                grid[row][j] = GUARDED

        return sum(1 if item == EMPTY else 0 for line in grid for item in line)
