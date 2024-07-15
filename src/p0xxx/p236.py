from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        vals = (p.val, q.val)

        def visit(node: Optional["TreeNode"]) -> Optional["TreeNode"]:
            if node is None:
                return None

            if node.val in vals:
                return node

            left = visit(node.left)

            if left is None:
                right = visit(node.right)
                if right is None:
                    return None
            else:
                right = visit(node.right)
                if right is None:
                    return left

            return node

        return visit(root)
