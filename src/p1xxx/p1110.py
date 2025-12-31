from leetcode_prelude import *


# Problem 1110
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def delNodes(
        self,
        root: Optional[TreeNode],
        to_delete: List[int],
    ) -> List[TreeNode]:
        res = []
        to_delete: set[int] = set(to_delete)

        def dfs(node: TreeNode | None) -> bool:
            if node is None:
                return None

            if dfs(node.left):
                node.left = None
            if dfs(node.right):
                node.right = None

            if node.val in to_delete:
                if node.left is not None:
                    res.append(node.left)

                if node.right is not None:
                    res.append(node.right)

                return True

            return False

        deleted = dfs(root)
        if not deleted:
            res.append(root)

        return res
