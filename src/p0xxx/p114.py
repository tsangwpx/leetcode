from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """

        def visit(node: TreeNode) -> TreeNode:
            if node.left is None:
                if node.right is None:
                    return node
                return visit(node.right)

            left, right = node.left, node.right
            node.left, node.right = None, left

            last = visit(left)
            if right is not None:
                last.right = right
                last = visit(right)

            return last

        visit(root)

    def flatten2(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        if root is None:
            return

        def visit(node: TreeNode) -> TreeNode:
            if node.left is None:
                if node.right is None:
                    return node
                return visit(node.right)

            left, right = node.left, node.right
            node.left, node.right = None, left

            last = visit(left)
            if right is not None:
                last.right = right
                last = visit(right)

            return last

        visit(root)
