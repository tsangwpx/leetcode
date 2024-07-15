from leetcode_prelude import *


# Problem 1481
class Solution:
    def findLeastNumOfUniqueInts(self, arr: List[int], k: int) -> int:
        from collections import Counter

        num_counter = Counter(arr)
        num_and_counts = sorted(num_counter.items(), key=lambda s: -s[1])

        while num_and_counts:
            _, count = num_and_counts[-1]

            if count > k:
                break

            num_and_counts.pop()
            k -= count

        return len(num_and_counts)
