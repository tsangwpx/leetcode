from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 230
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:

        # inorder search
        # may improve by stack, or generator
        def dfs(node, k):
            # k is the number of node remained

            if node is None:
                # keep k unchanged
                return None, k

            res, k = dfs(node.left, k)
            if res is not None:
                return res, k
            
            if k == 1:
                # this node is the target
                return node.val, 0
            
            # otherwise, ask the right side with k decreased by one
            return dfs(node.right, k - 1)

        return dfs(root, k)[0]