from leetcode_prelude import *


# Problem 2337
class Solution:
    def canChange(self, start: str, target: str) -> bool:
        n = len(start)
        i = j = 0

        while True:
            # skip "_"
            while i < n and start[i] == "_":
                i += 1

            while j < n and target[j] == "_":
                j += 1

            if i == n or j == n:
                # return True if successfully scan the whole string
                return i == n and j == n

            # the relative positions of L and R must be held.
            if start[i] != target[j]:
                return False

            if start[i] == "L":
                if i < j:
                    # L cant move to right
                    return False
            else:
                # R at start[i]
                if i > j:
                    # R cant move to left
                    return False

            i += 1
            j += 1

        assert False, "unreachable"
