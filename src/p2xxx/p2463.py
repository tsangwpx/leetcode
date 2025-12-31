from leetcode_prelude import *


# Problem 2463
class Solution:
    def minimumTotalDistance(
        self,
        robot: List[int],
        factory: List[List[int]],
    ) -> int:
        robot.sort()
        factory.sort()

        MAX = 10**9 * 1000

        dp = [[[-1] * 101 for _ in range(101)] for _ in range(101)]

        def solve(i: int, j: int, k: int) -> int:
            if dp[i][j][k] != -1:
                return dp[i][j][k]
            if i == len(robot):
                dp[i][j][k] = 0
            elif j == len(factory):
                dp[i][j][k] = MAX
            else:
                x = robot[i]
                y, limit = factory[j]
                dp[i][j][k] = min(
                    solve(i, j + 1, 0),
                    solve(i + 1, j, k + 1) + abs(x - y) if k < limit else MAX,
                )

            return dp[i][j][k]

        return solve(0, 0, 0)


# Problem 2684
class Solution:
    def maxMoves(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])

        # we may compress the space usage to O(N)
        # by scanning the grid from left to right one column at a time
        # so that the `pending` array is replaced by cell reachability in a column
        # and the `dp` matrix can be optimized by reusing two columns

        pending = [(0, i, 0) for i in range(m)]

        max_moves = 0
        dp = [[0] * n for _ in range(m)]

        while pending:
            count, i, j = pending.pop()

            level = grid[i][j]
            count += 1
            j += 1

            if j >= n:
                # moving out of last column
                continue

            found = False

            if i >= 1 and level < grid[i - 1][j] and dp[i - 1][j] < count:
                dp[i - 1][j] = count
                pending.append((count, i - 1, j))
                found = True

            if level < grid[i][j] and dp[i][j] < count:
                dp[i][j] = count
                pending.append((count, i, j))
                found = True

            if i + 1 < m and level < grid[i + 1][j] and dp[i + 1][j] < count:
                dp[i + 1][j] = count
                pending.append((count, i + 1, j))
                found = True

            if found:
                max_moves = max(max_moves, count)

        return max_moves


# Problem Q2
class Solution:
    def minBitwiseArray(self, nums: List[int]) -> List[int]:
        ans = []

        for number in nums:
            found = False
            for k in range(1, number):
                if (k | (k + 1)) == number:
                    found = True
                    ans.append(k)
                    break

            if not found:
                ans.append(-1)

        return ans


class Solution:
    def minBitwiseArray(self, nums: List[int]) -> List[int]:
        ans = []

        for number in nums:
            if number == 2:
                # special case
                ans.append(-1)
                continue

            shift = 0
            while ((number >> shift) & 1) == 1:
                shift += 1

            mask = (1 << shift) - 1
            prefix = number & ~mask

            target = prefix | (mask >> 1)
            ans.append(target)

        return ans
