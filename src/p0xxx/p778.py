from leetcode_prelude import *


# Problem 778
class Solution:
    def swimInWater(self, grid: List[List[int]]) -> int:
        from heapq import heappop, heappush

        DIRECTIONS = (
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        )

        n = len(grid)

        time = grid[0][0]
        seen = [[False] * n for _ in range(n)]
        seen[0][0] = True

        pq = [(grid[0][0], 0, 0)]

        while pq:
            elevation, row, col = heappop(pq)
            time = max(time, elevation)

            if row == n - 1 and col == n - 1:
                return time

            for dr, dc in DIRECTIONS:
                row2 = row + dr
                col2 = col + dc
                if 0 <= row2 < n and 0 <= col2 < n and not seen[row2][col2]:
                    seen[row2][col2] = True
                    heappush(pq, (grid[row2][col2], row2, col2))

        raise AssertionError("unreachable")
