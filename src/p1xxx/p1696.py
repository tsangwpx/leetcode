import collections
import heapq
from typing import List, Optional


class Solution:
    def maxResult(self, nums: List[int], k: int) -> int:
        lower = sum([s for s in nums if s < 0], start=0)
        dp = [lower] * len(nums)
        dp[0] = nums[0]

        for i, n in enumerate(nums):
            for j in range(i + 1, min(i + k + 1, len(nums))):
                dp[j] = max(dp[j], dp[i] + nums[j])

        return dp[-1]

    def maxResult(self, nums: List[int], k: int) -> int:
        dp = [0] * len(nums)

        for i, n in enumerate(nums):
            # default value is only used when i == 0
            dp[i] = max((dp[j] + n for j in range(max(0, i - k), i)), default=n)

        # print(dp)

        return dp[-1]

    def maxResult(self, nums: List[int], k: int) -> int:
        # print('nums', nums)
        dp = [0] * len(nums)
        win = collections.deque()

        for i, n in enumerate(nums):
            score = n + dp[win[0]] if win else n
            dp[i] = score

            # remove old index
            while win and i >= win[0] + k:
                win.popleft()

            # remove smaller values from back
            while win and score >= dp[win[-1]]:
                win.pop()

            win.append(i)

            # print(i, n, score, 'win', win, 'dp', dp)

        # print(dp)

        return dp[-1]

    def maxResult(self, nums: List[int], k: int) -> int:
        # print('nums', nums)
        win = collections.deque()  # (index, score)

        for i, n in enumerate(nums):
            score = n + win[0][1] if win else n

            # remove old index
            while win and i >= win[0][0] + k:
                win.popleft()

            # remove smaller values from back
            while win and score >= win[-1][1]:
                win.pop()

            win.append((i, score))

            # print(i, n, score, 'win', win, 'dp', dp)

        # print(dp)

        return win[-1][1]


def main():
    print(Solution().maxResult([1, -1, -2, 4, -7, 3], 2))
    print(Solution().maxResult([10, -5, -2, 4, 0, 3], 3))
    print(Solution().maxResult([1, -5, -20, 4, -1, 3, -6, -3], 2))
    print(Solution().maxResult([100, -1, -100, -1, 100], 2))
    print(Solution().maxResult([1, -5, -3, -7, 3], 3))
    # return


if __name__ == "__main__":
    main()
