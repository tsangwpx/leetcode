from leetcode_prelude import *


# Problem 3152
class Solution:
    def isArraySpecial(self, nums: List[int], queries: List[List[int]]) -> List[bool]:
        # count parity changes
        changed = [0]
        count = 0

        # changed[i] = N, arr[0..i] has N parity changes
        # so changed[0] = 0, obviously.
        # changed[1] depend on arr[0..1]

        for i in range(len(nums) - 1):
            if (nums[i] ^ nums[i + 1]) & 1 == 1:
                count += 1

            changed.append(count)

        res = []
        for start, stop in queries:
            steps = stop - start
            p0 = changed[start]
            p1 = changed[stop]
            res.append(p1 == p0 + steps)

        return res
