from leetcode_prelude import *


# Problem 3176
class Solution:
    def maximumLength(self, nums: List[int], k: int) -> int:
        """
        Time complexity: O(NK)
        Space complexity: O(NK)
        """
        from collections import defaultdict

        # record a number and its best length given diff
        best_length: list[dict[int, int]] = [defaultdict(int) for _ in range(k + 1)]

        # res[m] is the best length of at most m differences
        res: list[int] = [0] * (k + 1)

        for item in nums:
            for diff in range(k, -1, -1):
                same_length = best_length[diff][item] + 1
                diff_length = 0
                if diff >= 1:
                    # this step is tricky.
                    # the prev number may be the same
                    # but we could consider it different
                    diff_length = res[diff - 1] + 1

                length = max(same_length, diff_length)
                best_length[diff][item] = length
                res[diff] = max(res[diff], length)

        return res[k]

    def maximumLength2(self, nums: List[int], k: int) -> int:
        """
        Time complexity: O(N**2 K)
        Space complexity: O(NK)
        """
        from collections import defaultdict

        # record (number, diff) and its best length
        dp: dict[tuple[int, int], int] = defaultdict(int)

        for item in nums:
            dp2 = dp.copy()
            dp2[(item, 0)] = max(dp2[(item, 0)], 1)

            for (prev, diff), size in dp.items():
                if prev != item:
                    diff += 1

                    if diff > k:
                        continue

                dp2[(item, diff)] = max(dp2[(item, diff)], size + 1)

            # print(item, sorted(dp2.items()))
            dp = dp2

        return max(dp.values())
