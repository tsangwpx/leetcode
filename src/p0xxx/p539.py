from leetcode_prelude import *


# Problem 539
class Solution:
    def findMinDifference(self, timePoints: List[str]) -> int:
        """brute force sort time and next period time and compute difference"""
        day = 24 * 60  # one day in minutes

        seq = []

        for hhmm in timePoints:
            hh = int(hhmm[0:2])
            mm = int(hhmm[3:5])

            t0 = hh * 60 + mm
            t1 = t0 + day

            seq.append(t0)
            seq.append(t1)

        seq.sort()

        minimum = day
        for i in range(1, len(seq)):
            minimum = min(minimum, seq[i] - seq[i - 1])

        return minimum
