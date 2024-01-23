from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 287
class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        # first iteration here
        tort = nums[0]
        hare = nums[nums[0]]

        # until they meet
        while tort != hare:
            tort = nums[tort]
            hare = nums[nums[hare]]

        # place tort to the beginning
        tort = 0
        while tort != hare:
            tort = nums[tort]
            hare = nums[hare]

        # where they meet again is the duplicate number
        return tort
