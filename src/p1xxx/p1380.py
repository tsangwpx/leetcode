from leetcode_prelude import *


# Problem 1380
class Solution:
    def luckyNumbers(self, matrix: List[List[int]]) -> List[int]:
        m = len(matrix)
        n = len(matrix[0])
        row_minimums = [0] * m
        col_maximums = [0] * n

        for i in range(m):
            row_minimums[i] = min(matrix[i])

        for j in range(n):
            val = matrix[0][j]

            for i in range(1, m):
                val = max(matrix[i][j], val)

            col_maximums[j] = val

        seen = set(row_minimums)
        res = [s for s in col_maximums if s in seen]
        return res
