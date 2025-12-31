from leetcode_prelude import *


# Problem 3312
class Solution:

    @staticmethod
    def _compute_divisors() -> list[list[int]]:
        limit = 5 * 10**4 + 1

        table = [[] for _ in range(limit)]

        for divisor in range(1, limit):
            for multiple in range(divisor, limit, divisor):
                table[multiple].append(divisor)

        return table

    _DIVISORS: list[list[int]] = _compute_divisors()

    def gcdValues(self, nums: List[int], queries: List[int]) -> List[int]:
        from bisect import bisect_right

        limit = max(nums) + 1
        divisor_counts = [0] * limit
        for number in nums:
            for d in self._DIVISORS[number]:
                divisor_counts[d] += 1

        gcd_counts = [0] * limit
        for gcd in range(1, len(gcd_counts))[::-1]:
            count = divisor_counts[gcd]

            # There are `n` numbers with divisor `d`
            # so they can form `n * (n - 1) / 2` pairs
            count = count * (count - 1) // 2

            for multiple in range(gcd * 2, limit, gcd):
                # remove duplicated counts from larger GCD
                count -= gcd_counts[multiple]

            gcd_counts[gcd] = count

        total = 0
        table: list[tuple[int, int]] = []
        # print(divisor_counts)
        # print(gcd_counts)

        for gcd, count in enumerate(gcd_counts):
            if count == 0:
                continue

            total += count
            table.append((total, gcd))

        # print(total, table)
        # print(queries)

        res = [0] * len(queries)
        for idx, query in enumerate(queries):
            # bisect right here
            pos = bisect_right(table, (query, limit))
            _, gcd = table[pos]
            res[idx] = gcd

        return res
