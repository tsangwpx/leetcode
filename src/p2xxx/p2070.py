from leetcode_prelude import *


# Problem 2070
class Solution:
    def maximumBeauty(self, items: List[List[int]], queries: List[int]) -> List[int]:
        """
        Observations:
        1. the problem asks some kind of aggregations
        2. beauties with same price may exist. Pick the higher one.
        3. since the beauty is decreasing over price, bisect may be useful.
        """
        from collections import defaultdict
        from bisect import bisect_left, bisect_right

        table = defaultdict(int)
        table[0] = 0
        for price, beauty in items:
            table[price] = max(table[price], beauty)

        items = sorted(table.items())

        max_beauty = 0

        for i in range(len(items)):
            price, beauty = items[i]
            max_beauty = max(max_beauty, beauty)
            items[i] = (price, max_beauty)

        # print(items)

        res = []
        for cost in queries:
            idx = bisect_right(items, (cost, max_beauty + 1)) - 1
            if idx >= len(items):
                beauty = max_beauty
            else:
                beauty = items[idx][1]

            res.append(beauty)

        return res
