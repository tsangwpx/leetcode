from leetcode_prelude import *


# Problem 1368
class Solution:
    def minCost(self, grid: List[List[int]]) -> int:

        DIRECTIONS = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
        ]

        m = len(grid)
        n = len(grid[0])
        target_row = m - 1
        target_col = n - 1

        if target_row == 0 and target_col == 0:
            # special case
            return 0

        seen = [[False] * n for _ in range(m)]
        seen[0][0] = True

        pending = [(0, 0)]

        cost = 0

        # 1. Repeatedly search for reachable cells
        # 2. Increase cost value and modify newly visited cell to reach unseen cell.
        # 3. until reach the goal cell.

        # bfs to reach all cells
        while pending:
            newly_visited = []

            # dfs to find reachable cells without increasing cost
            while pending:
                row, col = pending.pop()
                newly_visited.append((row, col))

                dr, dc = DIRECTIONS[grid[row][col] - 1]
                row2 = row + dr
                col2 = col + dc

                if row2 == target_row and col2 == target_col:
                    return cost

                if 0 <= row2 < m and 0 <= col2 < n and not seen[row2][col2]:
                    seen[row2][col2] = True
                    pending.append((row2, col2))

            cost += 1

            for row, col in newly_visited:
                for dr, dc in DIRECTIONS:
                    row2 = row + dr
                    col2 = col + dc

                    if row2 == target_row and col2 == target_col:
                        return cost

                    if 0 <= row2 < m and 0 <= col2 < n and not seen[row2][col2]:
                        seen[row2][col2] = True
                        pending.append((row2, col2))

        raise AssertionError("unreachable")
