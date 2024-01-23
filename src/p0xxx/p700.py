from leetcode_prelude import *


# Problem 700
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def searchBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        node = root
        while node is not None:
            if node.val == val:
                break
            elif node.val > val:
                node = node.left
            else:
                node = node.right

        return node
