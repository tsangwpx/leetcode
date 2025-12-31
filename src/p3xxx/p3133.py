from leetcode_prelude import *


# Problem 3133
class Solution:
    def minEnd(self, n: int, x: int) -> int:
        """
        Observations:
        1. the one bits in x must appear in array items as well
        2. we change the remaining zero bits to create a strictly increasing array
        3. when n = 1, the array is [x].
           we need to place the binary presentation of (n - 1) in these zero bits
        """
        bits = n - 1
        res = x
        shift = 0

        while bits != 0:
            mask = 1 << shift

            if res & mask == 0:
                if bits & 1 == 1:
                    res |= mask

                bits >>= 1

            shift += 1

        return res
