import operator
from itertools import islice, starmap
from typing import List


class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        res: List[List[int]] = [[0] * n for n in range(1, numRows + 1)]
        res[0][0] = 1

        for i in range(1, numRows):
            last = res[i - 1]
            row = res[i]
            row[0] = row[-1] = 1

            row[1:-1] = starmap(
                operator.add,
                zip(islice(last, 1, len(last)), islice(last, 0, len(last) - 1)),
            )

        return res
