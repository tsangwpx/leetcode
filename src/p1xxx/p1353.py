from leetcode_prelude import *


# Problem 1353
class Solution:
    def maxEvents(self, events: List[List[int]]) -> int:
        from heapq import heapify, heappop, heappush

        pending = [(t0, t1) for t0, t1 in events]
        heapify(pending)

        ready = []

        count = 0
        day = 1

        while pending or ready:
            if not ready:
                day, _ = pending[0]

            while pending and pending[0][0] <= day:
                t0, t1 = heappop(pending)
                heappush(ready, (t1, t0))

            # print(day, ready, count)

            while ready:
                t1, t0 = heappop(ready)

                if t0 <= day <= t1:
                    count += 1
                    day += 1
                    break

        return count
