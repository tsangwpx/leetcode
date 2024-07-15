from leetcode_prelude import *


# Problem 409
class Solution:
    def longestPalindrome(self, s: str) -> int:
        table: dict[str, bool] = dict.fromkeys(s, False)

        for ch in s:
            table[ch] = not table[ch]

        single_count = sum(table.values())

        if single_count >= 1:
            return len(s) - single_count + 1
        else:
            return len(s)

    def longestPalindrome2(self, s: str) -> int:
        from collections import Counter

        has_single = False
        length = 0

        for _, count in Counter(s).items():
            # length += count // 2 * 2
            length += count & (-2)
            has_single = has_single or count % 2 == 1

        if has_single:
            length += 1

        return length
