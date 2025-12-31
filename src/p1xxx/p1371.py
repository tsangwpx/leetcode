from leetcode_prelude import *


# Problem 1371
class Solution:
    def findTheLongestSubstring(
        self,
        s: str,
    ) -> int:
        # from collections import defaultdict

        table = {
            "a": 1 << 0,
            "e": 1 << 1,
            "i": 1 << 2,
            "o": 1 << 3,
            "u": 1 << 4,
        }

        # mapping key to the first key index
        # inclusive
        memo: dict[int, int] = {}
        memo[0] = -1

        key = 0
        max_len = 0

        for idx, ch in enumerate(s):
            flag = table.get(ch, 0)
            key ^= flag

            if key not in memo:
                memo[key] = idx

            max_len = max(max_len, idx - memo[key])

        return max_len
