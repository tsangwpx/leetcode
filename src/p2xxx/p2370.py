from leetcode_prelude import *


# Problem 2370
class Solution:
    def longestIdealString(self, s: str, k: int) -> int:
        from collections import defaultdict

        base = ord("a")

        max_lengths = defaultdict(int)

        neighbours = {}
        for ch in range(26):
            start = max(ch - k, 0)
            stop = min(ch + k, 25) + 1

            neighbours[chr(ch + base)] = "".join(
                chr(s + base) for s in range(start, stop)
            )

        # print(neighbours)

        max_len = 1
        for ch in s:
            length = 1
            for friend in neighbours[ch]:
                length = max(length, max_lengths[friend] + 1)

            max_lengths[ch] = length
            max_len = max(max_len, length)

        return max_len
