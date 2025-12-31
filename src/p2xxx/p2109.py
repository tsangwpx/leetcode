from leetcode_prelude import *


# Problem 2109
class Solution:
    def addSpaces(self, s: str, spaces: List[int]) -> str:
        parts = []

        stop = spaces[0]
        parts.append(s[:stop])

        for i in range(len(spaces) - 1):
            start = spaces[i]
            stop = spaces[i + 1]
            parts.append(s[start:stop])

        start = spaces[-1]
        parts.append(s[start:])

        return " ".join(parts)
