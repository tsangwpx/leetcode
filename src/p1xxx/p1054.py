from leetcode_prelude import *


# Problem 1054
class Solution:
    def rearrangeBarcodes(
        self,
        barcodes: List[int],
    ) -> List[int]:
        """
        Write the most common and the second common to the output each time.

        Alternatively, we may write the even indices first by consuming
        the most frequent barcodes, and then write the odd indices later.
        This interleaving writes ensure neighbors are distinct.

        """
        from collections import Counter
        from heapq import heapify, heappop, heappush

        counter = Counter(barcodes)

        pq = [(-count, code) for code, count in counter.items()]
        heapify(pq)

        res = []
        while len(pq) >= 2:
            neg_count, code = heappop(pq)
            res.append(code)
            neg_count += 1

            neg_count2, code2 = heappop(pq)
            res.append(code2)
            neg_count2 += 1

            if neg_count < 0:
                heappush(pq, (neg_count, code))

            if neg_count2 < 0:
                heappush(pq, (neg_count2, code2))

        if pq:
            neg_count, code = heappop(pq)
            assert neg_count == -1
            res.append(code)

        return res
