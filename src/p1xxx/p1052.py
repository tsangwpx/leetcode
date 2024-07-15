from leetcode_prelude import *


# Problem 1052
class Solution:
    def maxSatisfied(
        self,
        customers: List[int],
        grumpy: List[int],
        minutes: int,
    ) -> int:
        satisfied = 0
        for count, bad in zip(customers, grumpy):
            if not bad:
                satisfied += count

        best = satisfied

        left = 0
        for idx in range(len(customers)):
            # 0 1 2 3; minutes = 3
            # ^left
            # when idx = 3, forget customers[0]

            if grumpy[idx]:
                satisfied += customers[idx]

            if idx >= left + minutes:
                if grumpy[left]:
                    satisfied -= customers[left]

                left += 1

            best = max(best, satisfied)

        return best
