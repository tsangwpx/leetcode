from leetcode_prelude import *


# Problem 1482
class Solution:
    def minDays(self, bloomDay: List[int], m: int, k: int) -> int:
        """
        Binary Search. O( N ) where N = len(bloomDay).

        Since the int range is bounded,
        the time complexity of binary search log( BOUNDED ) is constant.

        """

        if len(bloomDay) < m * k:
            # Impossible
            return -1

        lo = min(bloomDay)
        hi = max(bloomDay)

        # O(1)
        while lo < hi:
            mi = (lo + hi) // 2

            bouquets = 0
            adj = 0

            # O(N)
            for day in bloomDay:
                if mi >= day:
                    adj += 1
                else:
                    bouquets += adj // k
                    adj = 0

            bouquets += adj // k

            if bouquets >= m:
                hi = mi
            else:
                lo = mi + 1

        return lo

    def minDays2(self, bloomDay: List[int], m: int, k: int) -> int:
        """
        Union find. O( N log N ) where N = len(bloomDay)
        """
        from collections import defaultdict

        indices = list(range(len(bloomDay)))
        indices.sort(key=lambda s: bloomDay[s])

        size = len(bloomDay)
        heads = [size] * len(bloomDay)
        counter = defaultdict(int)
        bouquets = 0

        for idx in indices:

            if idx >= 1 and heads[idx - 1] < size:
                root = heads[idx - 1]
                while heads[root] != root:
                    root = heads[root]
            else:
                root = idx

            heads[idx] = root

            # undo bouquets before bump flower counter
            bouquets -= counter[root] // k
            counter[root] += 1

            right = idx + 1
            if right < size and heads[right] < size:
                assert heads[right] == right
                right_flowers = counter.pop(right)
                bouquets -= right_flowers // k
                counter[root] += right_flowers
                heads[right] = root

            bouquets += counter[root] // k

            if bouquets >= m:
                return bloomDay[idx]

        return -1
