from leetcode_prelude import *


# Problem 1400
class Solution:
    def canConstruct(self, s: str, k: int) -> bool:
        from collections import Counter

        counter = Counter(s)
        odd = sum(1 for count in counter.values() if count % 2 == 1)
        return odd <= k <= len(s)
