from leetcode_prelude import *


# Problem 826
class Solution:
    def maxProfitAssignment(
        self,
        difficulty: List[int],
        profit: List[int],
        worker: List[int],
    ) -> int:
        from bisect import bisect_right
        from collections import defaultdict

        table = defaultdict(int)

        for d, p in zip(difficulty, profit):
            table[d] = max(table[d], p)

        keys = sorted(table)
        minimum = table[keys[0]]
        maximum = minimum

        for d in keys:
            maximum = max(maximum, table[d])
            table[d] = maximum

        total = 0

        for w in worker:
            idx = bisect_right(keys, w)

            if idx == 0:
                # Due to bisect right,
                # no job with difficult equal or smaller to the worker skill.
                continue

            total += table[keys[idx - 1]]

        return total
