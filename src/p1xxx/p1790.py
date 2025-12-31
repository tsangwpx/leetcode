from leetcode_prelude import *


# Problem 1790
class Solution:
    def areAlmostEqual(self, s1: str, s2: str) -> bool:
        if len(s1) != len(s2):
            return False
        diff = 0
        da = []
        db = []
        for a, b in zip(s1, s2):
            if a == b:
                continue
            diff += 1
            if diff >= 3:
                return False

            da.append(a)
            db.append(b)

        if diff == 0:
            return True
        elif diff == 1:
            return False
        elif diff == 2:
            return sorted(da) == sorted(db)
        else:
            return False
