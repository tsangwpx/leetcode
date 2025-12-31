from leetcode_prelude import *


# Problem 1760
class Solution:
    def minimumSize(
        self,
        nums: List[int],
        maxOperations: int,
    ) -> int:
        penalty_max = max(nums)

        left = 1
        right = penalty_max  # inclusive

        while left < right:
            # see the branching comment below
            limit = (left + right) // 2
            op_count = 0

            for item in nums:
                op_count += -(-item // limit) - 1
                if op_count > maxOperations:
                    break

            if op_count > maxOperations:
                # too many operations
                # we have to increase the bin size
                # so the lower bound must be (limit + 1)
                # and because of this
                # we prefer the left middle point over the right one
                left = limit + 1
            else:
                right = limit

        return left
