from leetcode_prelude import *


# Problem 983
class Solution:
    def mincostTickets(self, days: List[int], costs: List[int]) -> int:
        """
        DP
        """
        from functools import cache

        day_pass, week_pass, month_pass = costs
        size = len(days)

        @cache
        def dfs(start: int) -> int:
            """
            minimum cost on the `start` day
            """
            if start == size:
                return 0

            day = days[start]
            idx = start
            while idx < size and days[idx] - day < 1:
                idx += 1

            day_cost = day_pass + dfs(idx)

            while idx < size and days[idx] - day < 7:
                idx += 1

            week_cost = week_pass + dfs(idx)

            while idx < size and days[idx] - day < 30:
                idx += 1

            month_cost = month_pass + dfs(idx)

            return min(day_cost, week_cost, month_cost)

        return dfs(0)
