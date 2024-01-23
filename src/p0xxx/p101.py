from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 101
class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        pending = [(root.left, root.right)]

        while pending:
            left, right = pending.pop()
            if (left is None) != (right is None):
                return False
            if left is None:
                continue
            if left.val != right.val:
                return False
            pending.append((left.left, right.right))
            pending.append((left.right, right.left))

        return True

    def isSymmetric2(self, root: Optional[TreeNode]) -> bool:
        def is_equal(left, right):
            if left is None:
                return right is None
            elif right is None:
                return False

            return (
                left.val == right.val
                and is_equal(left.left, right.right)
                and is_equal(left.right, right.left)
            )

        return is_equal(root.left, root.right)
