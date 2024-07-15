from leetcode_prelude import *


# Problem 1002
class Solution:
    def commonChars(self, words: List[str]) -> List[str]:
        from collections import Counter
        from string import ascii_lowercase

        dp = {s: 100 for s in ascii_lowercase}

        for word in words:
            counter = Counter(word)

            for ch in ascii_lowercase:
                dp[ch] = min(dp[ch], counter[ch])

        res = []
        for ch, count in dp.items():
            res += [ch] * count

        return res
