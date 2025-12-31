from leetcode_prelude import *


# Problem 1861
class Solution:
    def rotateTheBox(self, box: List[List[str]]) -> List[List[str]]:
        STONE: int = "#"
        OBSTACLE: int = "*"
        EMPTY: int = "."

        m = len(box)
        n = len(box[0])
        res = [[EMPTY] * m for _ in range(n)]

        for i in range(m):
            stones = 0

            for j in range(n):
                item = box[i][j]

                if item == STONE:
                    stones += 1
                elif item == OBSTACLE:
                    res[j][m - i - 1] = OBSTACLE

                if stones >= 1 and (j + 1 == n or box[i][j + 1] == OBSTACLE):
                    col = m - i - 1
                    # print(
                    #     (i, j),
                    #     f"fill {col=} {stones=} in ",
                    #     range(j - stones + 1, j + 1),
                    # )

                    for row in range(j - stones + 1, j + 1):
                        res[row][col] = STONE

                    stones = 0

        return res
