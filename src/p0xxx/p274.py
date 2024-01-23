from leetcode_prelude import *


# Problem 274
class Solution:
    def hIndex(self, citations: List[int]) -> int:
        publications = len(citations)
        buckets = [0] * (len(citations) + 1)

        for cites in citations:
            buckets[min(cites, publications)] += 1

        count = 0
        for h in range(publications, -1, -1):
            count += buckets[h]

            if count >= h:
                return h

        return 0
