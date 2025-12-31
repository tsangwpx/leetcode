from leetcode_prelude import *


# Problem 2641
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def replaceValueInTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        root2 = TreeNode(val=0)

        row = [(root, root2)]
        next_row = []

        while row:
            child_level_sum = 0

            for node, node2 in row:
                if node.left:
                    child_level_sum += node.left.val
                if node.right:
                    child_level_sum += node.right.val

            for node, node2 in row:
                child_sum = 0
                if node.left:
                    child_sum += node.left.val
                if node.right:
                    child_sum += node.right.val

                if node.left:
                    node2.left = TreeNode(val=child_level_sum - child_sum)
                    next_row.append((node.left, node2.left))
                if node.right:
                    node2.right = TreeNode(val=child_level_sum - child_sum)
                    next_row.append((node.right, node2.right))

            row, next_row = next_row, row
            next_row.clear()

        return root2
