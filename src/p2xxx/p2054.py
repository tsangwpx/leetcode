from leetcode_prelude import *


# Problem 2054
class Solution:
    def maxTwoEvents(self, events: List[List[int]]) -> int:
        timeline = []

        for start_time, end_time, score in events:
            timeline.append((start_time, 1, score))
            timeline.append((end_time + 1, 0, score))

        timeline.sort()

        res = 0
        best_finished = 0

        for _, is_start, score in timeline:
            if is_start:
                res = max(res, score + best_finished)
            else:
                best_finished = max(best_finished, score)

        return res
