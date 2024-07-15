from leetcode_prelude import *


# Problem 3160
class Solution:
    def queryResults(
        self,
        limit: int,
        queries: List[List[int]],
    ) -> List[int]:
        from collections import Counter

        ball_colors = {}
        color_counter = Counter()

        res = []

        for x, y in queries:
            if x in ball_colors:
                old_color = ball_colors[x]
                color_counter[old_color] -= 1

                if color_counter[old_color] == 0:
                    color_counter.pop(old_color)

            ball_colors[x] = y
            color_counter[y] += 1

            res.append(len(color_counter))

        return res
