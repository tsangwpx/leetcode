from leetcode_prelude import *


# Problem 447
class Solution:
    def numberOfBoomerangs(self, points: List[List[int]]) -> int:
        from collections import defaultdict

        size = len(points)
        table: list[dict[int, int]] = [defaultdict(int) for _ in range(size)]
        count = 0

        for i in range(size):
            xi, yi = points[i]
            ti = table[i]

            for j in range(i + 1, size):
                xj, yj = points[j]
                tj = table[j]

                dist = (xi - xj) ** 2 + (yi - yj) ** 2

                count += ti[dist]
                ti[dist] += 1

                count += tj[dist]
                tj[dist] += 1

        return count * 2
