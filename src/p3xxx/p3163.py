from leetcode_prelude import *


# Problem 3163
class Solution:
    def compressedString(self, word: str) -> str:
        last = ""
        count = 1
        parts = []

        for ch in word:
            if ch == last and count < 9:
                count += 1
            elif last:
                parts.append(f"{count}{last}")
                count = 1

            last = ch

        if last:
            parts.append(f"{count}{last}")

        return "".join(parts)
