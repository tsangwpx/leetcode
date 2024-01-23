from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 108
class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        def dfs(start: int, stop: int) -> Optional[TreeNode]:
            print("dfs", start, stop)
            if start >= stop:
                return None

            mid = (start + stop) // 2
            return TreeNode(
                val=nums[mid],
                left=dfs(start, mid),
                right=dfs(mid + 1, stop),
            )

        return dfs(0, len(nums))
