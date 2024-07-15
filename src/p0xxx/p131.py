from leetcode_prelude import *


# Problem 131
class Solution:
    def partition(
        self,
        string: str,
    ) -> List[List[str]]:
        size = len(string)
        table = [[False] * size for _ in range(size)]

        # stop is inclusive
        for stop in range(size):
            ch2 = string[stop]

            for start in range(stop):
                ch = string[start]

                table[start][stop] = ch == ch2 and (
                    stop - start <= 2 or table[start + 1][stop - 1]
                )

            table[stop][stop] = True

        res = []
        work = []

        def recurse(start):
            if start == size:
                res.append(work.copy())
                return

            for stop in range(start, size):
                if table[start][stop]:
                    work.append(string[start : stop + 1])
                    recurse(stop + 1)
                    work.pop()

        recurse(0)

        return res
