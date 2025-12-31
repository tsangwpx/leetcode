from leetcode_prelude import *


# Problem 1930
class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        from collections import Counter

        found: set[tuple[str, str]] = set()
        counter = Counter()
        first_occurrences: dict[str, int] = {}
        last_occurrences: dict[str, int] = {}

        for idx, ch in enumerate(s):
            counter[ch] += 1

            start = first_occurrences.setdefault(ch, idx)

            if start < idx:
                for ch2, mid in last_occurrences.items():
                    if start < mid:
                        found.add((ch, ch2))

            last_occurrences[ch] = idx

        return len(found)
