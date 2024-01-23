from leetcode_prelude import *


# Problem 173
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class BSTIterator:
    def __init__(self, root: Optional[TreeNode]):
        self.values = values = []
        self.index = 0

        def visit(node):
            if node is None:
                return
            visit(node.left)
            values.append(node.val)
            visit(node.right)

        visit(root)

    def next(self) -> int:
        result = self.values[self.index]
        self.index += 1
        return result

    def hasNext(self) -> bool:
        return self.index < len(self.values)


# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator(root)
# param_1 = obj.next()
# param_2 = obj.hasNext()
