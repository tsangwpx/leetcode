from leetcode_prelude import *


# Problem 2381
class Solution:
    def shiftingLetters(self, s: str, shifts: List[List[int]]) -> str:
        alphabets = "abcdefghijklmnopqrstuvwxyz"
        assert len(set(alphabets)) == 26

        table = {}
        for idx, ch in enumerate(alphabets):
            table[ch] = alphabets[idx:] + alphabets[:idx]

        changes = [0] * (len(s) + 1)

        for start, end, direction in shifts:
            stop = end + 1
            if direction == 0:
                direction = -1

            changes[start] += direction
            changes[stop] += -direction

        offset = 0
        parts = []

        for idx, ch in enumerate(s):
            offset += changes[idx]
            parts.append(table[ch][offset % 26])

        return "".join(parts)
