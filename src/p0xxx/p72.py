from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 72
class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        m = len(word1)
        n = len(word2)

        dp = [[0] * (n + 1) for _ in range(m + 1)]

        dp[0][0] = 0  # both are zero strings

        for i in range(1, n + 1):
            # word1 -> word2 need i insertions
            dp[0][i] = i

        for i in range(1, m + 1):
            # word1 -> word2 need i deletions
            dp[i][0] = i

        # dp[i][j] is the number of actiosn taken to make word1 become word2
        # So at state (i, j), word1 and word2 are assumed to be the same!

        # Transitions from (i, j)
        # (i, j) -> (i + 1, j + 1) # same or replace the new character
        # (i, j) -> (i + 1, j)  # delete is needed in word1
        # (i, j) -> (i, j + 1)  # insert is needed in word1

        # From (i + 1, j + 1)'s views
        # (i, j) need same (+0) or replace (+1)
        # (i, j + 1) need insert (+1)
        # (i + 1, j) need delete (+1)
        # also

        for i in range(m):
            for j in range(n):
                if word1[i] == word2[j]:
                    # we dont need to consider (i, j + 1) or (i + 1, j) in minization here 
                    # because they are bounded below by dp[i][j]
                    dp[i + 1][j + 1] = dp[i][j]
                else:
                    dp[i + 1][j + 1] = min(dp[i][j], dp[i + 1][j], dp[i][j + 1]) + 1

        return dp[m][n]
