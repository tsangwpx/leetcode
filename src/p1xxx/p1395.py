from leetcode_prelude import *


# Problem 1395
class Solution:
    def numTeams(self, rating: List[int]) -> int:
        """

        Maintain two lists of dicts, which count the valid partial tuples
        One list is for increasing tuples; another decreasing tuples.

        list[0] is the count of uni-tuples, (a)
        list[1] is the count of bi-tuples, (a, b)
        list[2] is not needed because we dont need to save the result.

        In each iteration,
            1. list[1] can be updated with list[0] and item
            2. item is then added to list[0]

        See also others' implementations for alternatives.
        """
        from collections import defaultdict

        total = 0

        # (a, b, c) st. a < b* < c
        lt_ranks = [defaultdict(int) for _ in range(2)]

        # (a, b, c) st. a > b > c
        gt_ranks = [defaultdict(int) for _ in range(2)]

        for number in rating:
            for pivot, count in lt_ranks[1].items():
                if number < pivot:
                    total += count

            for pivot, count in lt_ranks[0].items():
                if number < pivot:
                    lt_ranks[1][number] += count

            lt_ranks[0][number] += 1

            for pivot, count in gt_ranks[1].items():
                if number > pivot:
                    total += count

            for pivot, count in gt_ranks[0].items():
                if number > pivot:
                    gt_ranks[1][number] += count

            gt_ranks[0][number] += 1

        return total
