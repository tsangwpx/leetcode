from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 64
class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])
        dp = [[0] * n for _ in range(m)]

        # space may be improvied by storing last two anti-diagonal elements
        # but indexing become tricky

        dp[0][0] = grid[0][0]

        # initialize first column and row, which are boundary
        for i in range(1, m):
            dp[i][0] = dp[i - 1][0] + grid[i][0]

        for j in range(1, n):
            dp[0][j] = dp[0][j - 1] + grid[0][j]

        for i in range(1, m):
            for j in range(1, n):
                dp[i][j] = grid[i][j] + min(dp[i - 1][j], dp[i][j - 1])

        return dp[m - 1][n - 1]

    def minPathSum2(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])
        dp = [[0] * n for _ in range(m)]

        # space may be improvied by storing last two anti-diagonal elements
        # but indexing become tricky
        # indexing *is* now tricky though

        dp[0][0] = grid[0][0]

        # initialize first column and row, which are boundary
        for i in range(1, m):
            dp[i][0] = dp[i - 1][0] + grid[i][0]

        for j in range(1, n):
            dp[0][j] = dp[0][j - 1] + grid[0][j]

        row_start = col_start = 1

        while row_start < m and col_start < n:
            row = row_start
            col = col_start

            while row > 0 and col < n:
                dp[row][col] = grid[row][col] + min(dp[row - 1][col], dp[row][col - 1])
                row -= 1
                col += 1

            row_start += 1
            if row_start == m:
                row_start = m - 1
                col_start += 1

            if col_start == n:
                break

        return dp[m - 1][n - 1]
