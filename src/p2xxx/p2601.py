from leetcode_prelude import *


# Problem 2601
class Solution:
    def _compute_primes() -> list[int]:
        limit = 1000

        primes = [2, 3]

        for n in range(5, limit + 1, 2):
            if any(n % s == 0 for s in primes):
                continue
            primes.append(n)

        return primes

    _PRIMES = _compute_primes()

    del _compute_primes

    def primeSubOperation(self, nums: List[int]) -> bool:
        from bisect import bisect_right

        primes = self._PRIMES

        for idx in range(len(nums) - 2, -1, -1):
            value = nums[idx]
            next_ = nums[idx + 1]
            delta = value - next_

            if delta < 0:
                continue

            pos = bisect_right(primes, delta)
            if pos >= len(primes):
                return False

            p = primes[pos]
            if p >= value:
                return False

            nums[idx] -= p

        return True
