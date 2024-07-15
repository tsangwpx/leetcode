from leetcode_prelude import *


# Problem 502
class Solution:
    def findMaximizedCapital(
        self,
        k: int,
        w: int,
        profits: List[int],
        capital: List[int],
    ) -> int:
        from heapq import heapify, heappop, heappush

        money = w

        ready = []
        pending = []

        for gain, minimum in zip(profits, capital):
            if gain == 0:
                continue
            elif money >= minimum:
                ready.append(-gain)
            else:
                pending.append((minimum, gain))

        heapify(ready)
        heapify(pending)

        for _ in range(k):
            while pending:
                minimum, gain = pending[0]
                if money >= minimum:
                    heappop(pending)
                    heappush(ready, -gain)
                else:
                    break

            if not ready:
                break

            money += -heappop(ready)

        return money
