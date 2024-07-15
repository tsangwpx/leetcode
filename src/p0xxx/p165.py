from leetcode_prelude import *


# Problem 165
class Solution:
    def compareVersion(self, version1: str, version2: str) -> int:
        from itertools import zip_longest

        parts1 = version1.split(".")
        parts2 = version2.split(".")

        for rev1, rev2 in zip_longest(parts1, parts2):
            ver1 = 0 if rev1 is None else int(rev1)
            ver2 = 0 if rev2 is None else int(rev2)

            if ver1 < ver2:
                return -1
            elif ver1 > ver2:
                return 1

        return 0
