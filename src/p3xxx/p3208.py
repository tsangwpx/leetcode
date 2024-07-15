from leetcode_prelude import *


# Problem 3208
class Solution:
    def numberOfAlternatingGroups(self, colors: List[int], k: int) -> int:
        count = 0
        size = 0
        prev = 0

        for i in range(len(colors)):
            item = colors[i]

            if size == 0 or prev == item:
                if size >= k:
                    count += size - k + 1

                size = 1
            else:
                size += 1

            prev = item

        for i in range(k - 1):
            item = colors[i]

            if item == prev:
                if size >= k:
                    count += size - k + 1

                size = 1
            else:
                size += 1

            prev = item

        if size >= k:
            count += size - k + 1

        return count
