from leetcode_prelude import *


# Problem 862
class Solution:
    def shortestSubarray(self, nums: List[int], k: int) -> int:
        """

        Steal from lee

        1. Use prefix sums to reduce O(N**2) to O(N**2), problem of start and stop indices
            P[i + 1] = A[0] + ... + A[i]
        2. Given a starting index `i`, let `j > i` be the minimum index such that
            P[j + 1] - P[i] >= k
            we need not check any index q > j because A[i] + ... + A[j] is the minimum sub-array.
        3. When iterating the array,
            we can safely skip/remove prefix P[i] if it satisfy the condition in previous steps.
            Time complexity is still O(N**2).
        4. To reach O(N), observe that the potential candidates in the prefix sums must be
            increasing. Given a stopping index j, we want to P[i] to be smaller and smaller
            (as i decrease) so that P[j + 1] - P[i] is bigger and bigger.
        5. Finally, the data structure suit the situation is a deque
            and we maintain the monotonic property.
        """
        from collections import deque

        mono = deque()
        mono.append((0, 0))

        acc = 0
        res = len(nums) + 1

        for idx, number in enumerate(nums):
            acc += number

            # print(acc, mono)
            while mono and acc - mono[0][1] >= k:
                left, _ = mono.popleft()
                res = min(res, idx + 1 - left)

            while mono and mono[-1][1] >= acc:
                mono.pop()

            mono.append((idx + 1, acc))

        if res > len(nums):
            return -1

        return res
