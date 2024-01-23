from leetcode_prelude import *


# Problem 103
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []

        reverse = False
        nodes = [root]
        childiren = []
        result = []

        while nodes:
            values = []
            result.append(values)

            for node in nodes:
                values.append(node.val)
                if node.left is not None:
                    childiren.append(node.left)
                if node.right is not None:
                    childiren.append(node.right)

            if reverse:
                values.reverse()

            reverse = not reverse
            childiren, nodes = nodes, childiren
            childiren.clear()

        return result
