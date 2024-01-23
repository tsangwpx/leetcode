from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 152
class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        maximum = nums[0]
        product = 1

        for item in nums:
            product *= item
            maximum = max(maximum, product)

            if product == 0:
                product = 1

        product = 1
        for item in reversed(nums):
            product *= item
            maximum = max(maximum, product)
            if product == 0:
                product = 1

        return maximum
