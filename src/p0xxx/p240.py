from bisect import bisect_left
from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        m, n = len(matrix), len(matrix[0])

        # From the bottom left corner
        j, i = m - 1, 0

        while j >= 0 and i < n:
            item = matrix[j][i]

            if item == target:
                return True
            elif item > target:
                j -= 1
            else:
                i += 1

        return False

    def searchMatrix2(self, matrix: List[List[int]], target: int) -> bool:
        for j in range(len(matrix)):
            row = matrix[j]
            if not row[0] <= target <= row[-1]:
                continue

            i = bisect_left(row, target)
            if i < len(row) and row[i] == target:
                return True

        return False


if __name__ == "__main__":
    print(
        Solution().scheduleCourse([[100, 200], [200, 1300], [1000, 1250], [2000, 3200]])
    )
    print(Solution().scheduleCourse([[1, 2]]))
    print(Solution().scheduleCourse([[3, 2], [4, 3]]))
