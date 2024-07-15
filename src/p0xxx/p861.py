from leetcode_prelude import *


# Problem 861
class Solution:
    def matrixScore(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])
        mask = 1 << n - 1

        arr = [0] * m
        col_bit_count = [0] * n

        for i, row in enumerate(grid):
            number = 0
            inv = row[0] == 0

            for j, bit in enumerate(row):
                if inv:
                    bit = 1 - bit

                number <<= 1
                number |= bit

                if bit:
                    col_bit_count[j] += 1

            arr[i] = number

        # print(arr, col_bit_count)
        for j in range(1, n):
            if col_bit_count[j] * 2 >= m:
                continue
            bit = 1 << (n - 1 - j)
            for i in range(m):
                arr[i] ^= bit
        # print(arr)

        return sum(arr)
