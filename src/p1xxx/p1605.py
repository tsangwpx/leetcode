from leetcode_prelude import *


# Problem 1605
class Solution:
    def restoreMatrix(
        self,
        rowSum: List[int],
        colSum: List[int],
    ) -> List[List[int]]:
        """
        O(MN) init
        O(M+N) update
        """
        # m-by-n matrix
        m = len(rowSum)
        n = len(colSum)

        matrix = [[0] * n for _ in range(m)]

        i = j = 0

        # zip zap pattern
        while i < m and j < n:
            used = min(rowSum[i], colSum[j])
            rowSum[i] -= used
            colSum[j] -= used
            matrix[i][j] = used

            # one of the following branches is always executed

            if rowSum[i] == 0:
                i += 1

            if colSum[j] == 0:
                j += 1

        return matrix

    def restoreMatrix2(
        self,
        rowSum: List[int],
        colSum: List[int],
    ) -> List[List[int]]:
        """
        O(NM) init + update
        """
        # m-by-n matrix
        m = len(rowSum)
        n = len(colSum)

        matrix = [[0] * n for _ in range(m)]

        for i in range(m):
            rem = rowSum[i]

            for j in range(n):
                used = min(colSum[j], rem)

                matrix[i][j] = used
                colSum[j] -= used
                rem -= used

        return matrix
