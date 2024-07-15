from leetcode_prelude import *


# Problem 554
class Solution:
    def leastBricks(self, wall: List[List[int]]) -> int:
        from collections import Counter

        height = len(wall)
        width = sum(wall[0])

        edges = Counter()

        for row in wall:
            offset = 0
            for brick in row:
                offset += brick
                edges[offset] += 1

        edges[width] = 0

        return height - max(edges.values())
