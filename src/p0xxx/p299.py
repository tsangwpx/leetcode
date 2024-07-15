from leetcode_prelude import *


# Problem 299
class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        from collections import Counter

        bulls = 0
        wanted = Counter()
        guessed = Counter()
        for a, b in zip(secret, guess):
            if a == b:
                bulls += 1
            else:
                wanted[a] += 1
                guessed[b] += 1

        cows = 0
        for ch, count in wanted.items():
            count2 = guessed[ch]
            cows += min(count, count2)

        return f"{bulls}A{cows}B"
