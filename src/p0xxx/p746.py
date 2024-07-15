from typing import List


class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        size = len(cost)

        cost.extend((0, 0))
        dp = [2**30] * len(cost)

        dp[0] = cost[0]
        dp[1] = cost[1]

        for i in range(size):
            dp[i + 1] = min(dp[i + 1], dp[i] + cost[i + 1])
            dp[i + 2] = min(dp[i + 2], dp[i] + cost[i + 2])

        return dp[size]


def main():
    print(Solution().latestTimeCatchTheBus([10, 20], [2, 17, 18, 19], 2))
    print(
        Solution().latestTimeCatchTheBus([20, 30, 10], [19, 13, 26, 4, 25, 11, 21], 2)
    )
    # return
    # print(Solution().isPossible([2, 4, 6, 7, 8]))
    # print(Solution().isPossible([1, 2, 3, 4, 5]))
    # print(Solution().isPossible([9, 3, 5]))
    # print(Solution().isPossible([1, 1, 1, 2]))
    # print(Solution().isPossible([8, 5]))


if __name__ == "__main__":
    main()
