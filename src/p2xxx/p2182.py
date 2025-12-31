from leetcode_prelude import *


# Problem 2182
class Solution:
    def repeatLimitedString(self, s: str, repeatLimit: int) -> str:
        from collections import Counter
        from heapq import heappop, heappush, heapify

        pq = []

        for ch, count in Counter(s).items():
            pq.append((-ord(ch), ch, count))

        heapify(pq)

        parts = []

        while pq:
            rank, ch, count = heappop(pq)
            used = min(repeatLimit, count)
            parts.append(ch * used)
            count -= used

            if count >= 1:
                if not pq:
                    break
                rank2, ch2, count2 = heappop(pq)
                parts.append(ch2)
                count2 -= 1
                if count2 >= 1:
                    heappush(pq, (rank2, ch2, count2))
                heappush(pq, (rank, ch, count))

        return "".join(parts)
