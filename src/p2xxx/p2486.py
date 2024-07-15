from leetcode_prelude import *


# Problem 2486
class Solution:
    def appendCharacters(self, s: str, t: str) -> int:
        c = 0  # count
        p = 0  # position

        h: str  # character
        for h in t:
            p = s.find(h, p)
            if p < 0:
                break
            p += 1
            c += 1

        return len(t) - c
