from leetcode_prelude import *


# Problem 3169
class Solution:
    def countDays(self, days: int, meetings: List[List[int]]) -> int:
        count = 0

        meetings.sort()

        day = 1

        for start, stop in meetings:
            if start > day:
                count += start - day

            day = max(day, stop + 1)

        if days >= day:
            count += days - day + 1

        return count
