from leetcode_prelude import *


# Problem 2684
class Solution:
    def maxMoves(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])

        # we may compress the space usage to O(N)
        # by scanning the grid from left to right one column at a time
        # so that the `pending` array is replaced by cell reachability in a column
        # and the `dp` matrix can be optimized by reusing two columns

        pending = [(0, i, 0) for i in range(m)]

        max_moves = 0
        dp = [[0] * n for _ in range(m)]

        while pending:
            count, i, j = pending.pop()

            level = grid[i][j]
            count += 1
            j += 1

            if j >= n:
                # moving out of last column
                continue

            found = False

            if i >= 1 and level < grid[i - 1][j] and dp[i - 1][j] < count:
                dp[i - 1][j] = count
                pending.append((count, i - 1, j))
                found = True

            if level < grid[i][j] and dp[i][j] < count:
                dp[i][j] = count
                pending.append((count, i, j))
                found = True

            if i + 1 < m and level < grid[i + 1][j] and dp[i + 1][j] < count:
                dp[i + 1][j] = count
                pending.append((count, i + 1, j))
                found = True

            if found:
                max_moves = max(max_moves, count)

        return max_moves
