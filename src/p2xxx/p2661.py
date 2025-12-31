from leetcode_prelude import *


# Problem 2661
class Solution:
    def firstCompleteIndex(self, arr: List[int], mat: List[List[int]]) -> int:
        m = len(mat)
        n = len(mat[0])

        num2row = [-1] * len(arr)
        num2col = [-1] * len(arr)

        for i, row in enumerate(mat):
            for j, number in enumerate(row):
                idx = number - 1
                num2row[idx] = i
                num2col[idx] = j

        row_counter = [0] * m
        col_counter = [0] * n

        for i, number in enumerate(arr):
            idx = number - 1

            row_idx = num2row[idx]
            col_idx = num2col[idx]

            row_counter[row_idx] += 1
            col_counter[col_idx] += 1
            if row_counter[row_idx] == n or col_counter[col_idx] == m:
                return i

        raise AssertionError("unreachable")
