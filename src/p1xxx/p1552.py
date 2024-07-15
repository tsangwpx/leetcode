from leetcode_prelude import *


# Problem 1552
class Solution:
    def maxDistance(self, position: List[int], m: int) -> int:
        position.sort()

        lo = 1
        hi = (position[-1] - position[0]) // (m - 1)

        while lo < hi:
            force = (lo + hi + 1) // 2

            last = position[0]
            done = 1

            for idx in range(1, len(position)):
                dist = position[idx] - last
                if dist >= force:
                    last = position[idx]
                    done += 1

            # print(force, lo, hi, done)

            if done >= m:
                lo = force
            elif done < m:
                hi = force - 1

        return lo
