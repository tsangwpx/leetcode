from leetcode_prelude import *


# Problem 1975
class Solution:
    def maxMatrixSum(self, matrix: List[List[int]]) -> int:
        neg_count = 0
        sum_ = 0
        abs_smallest = 10**5

        for row in matrix:
            for item in row:
                abs_item = abs(item)
                abs_smallest = min(abs_smallest, abs_item)
                sum_ += abs_item

                if item < 0:
                    neg_count += 1

        if neg_count % 2 == 1:
            sum_ -= abs_smallest
            sum_ -= abs_smallest

        return sum_
