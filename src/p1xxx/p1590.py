from leetcode_prelude import *


# Problem 1590
class Solution:
    def minSubarray(self, nums: List[int], p: int) -> int:
        """
        The problem:

            Remove the smallest sub-array such that the remaining sum is divisible by p

        Let R = sum[0:n] mod p, which is the remainder of the array sum.
        The problem want to find the smallest interval [a,b) such that
            sum[a:b] mod p = R
        because (sum[0:a] + sum[b:n]) mod p = 0 is desired.

        Now, the problem become how to find sum[a:b] mod p = R.
        Let X is the remainder of the running sum over [0:b],
            sum[0:b] mod p = X
        and we want to know if there is interval [a,b] such that
            R = sum[a:b] mod p
              = (sum[0:b] - sum[0:a]) mod p
              = (X - Y) mod p
            where Y = sum[0:a] mod p

        Rearranging the equation give us:
            Y = (X - R) mod p

        When Y exists, it tells the location of `a`.
        Note that sum[0:0] mod p = 0 is given as initial condition.

        This suggests that the algorithm need to memorize
        the closest position of a remainder of running sum.

        Be careful with boundary cases and off-by-one errors.
        """
        from collections import defaultdict

        target = sum(nums) % p
        if target == 0:
            return 0

        # the last indices give remainders of running sum
        # the index is inclusive
        # assign index -1 to 0 because sum[0:0] = 0 and it works in the computation.
        last_occurrences = defaultdict()
        last_occurrences[0] = -1

        acc = 0

        # removing the whole array is invalid
        width = len(nums)

        for idx, item in enumerate(nums):
            acc += item
            rem = acc % p
            com = (acc - target) % p
            if com in last_occurrences:
                # the last index is inclusive, and we dont want that number.
                # so starting the sum from (last + 1).
                # If the 0 index is not wanted, then the last should be -1
                width = min(width, idx + 1 - (last_occurrences[com] + 1))
            last_occurrences[rem] = idx

        return -1 if width == len(nums) else width
