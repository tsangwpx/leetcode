from leetcode_prelude import *


# Problem 1823
class Solution:
    def findTheWinner(self, n: int, k: int) -> int:
        alive = list(range(n))
        curr = 0

        while len(alive) >= 2:
            # print(n, curr, alive)
            curr = (curr + k - 1) % n
            del alive[curr]
            n -= 1
            curr = curr % n
            # print(n, curr, alive, "END")

        return alive[0] + 1
