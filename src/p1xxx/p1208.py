from leetcode_prelude import *


# Problem 1208
class Solution:
    def equalSubstring(self, s: str, t: str, maxCost: int) -> int:
        table = {chr(ord("a") + i): i for i in range(26)}

        max_length = 0
        left = 0

        for idx, (a, b) in enumerate(zip(s, t)):
            if a == b:
                max_length = max(max_length, idx + 1 - left)
                continue

            cost = abs(table[a] - table[b])
            while cost > maxCost and left < idx:
                refound = abs(table[s[left]] - table[t[left]])
                maxCost += refound
                left += 1

            if cost <= maxCost:
                maxCost -= cost
                max_length = max(max_length, idx + 1 - left)
            else:
                left = idx + 1

        return max_length
