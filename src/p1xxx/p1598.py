from leetcode_prelude import *


# Problem 1598
class Solution:
    def minOperations(self, logs: List[str]) -> int:
        level = 0

        for txt in logs:
            if txt == "../":
                level = max(0, level - 1)
            elif txt == "./":
                continue
            else:
                level += 1

        return level
