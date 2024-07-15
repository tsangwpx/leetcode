from leetcode_prelude import *


# Problem 387
class Solution:
    def firstUniqChar(self, s: str) -> int:
        from collections import Counter

        counter = Counter(s)

        for i, ch in enumerate(s):
            if counter[ch] == 1:
                return i

        return -1

    def firstUniqChar(self, s: str) -> int:
        from collections import defaultdict

        # Insertion order is preserved
        counter = defaultdict(int)
        for i in range(len(s)):
            ch = s[i]
            counter[ch] += 1

        for ch, count in counter.items():
            if count == 1:
                return s.index(ch)

        return -1
