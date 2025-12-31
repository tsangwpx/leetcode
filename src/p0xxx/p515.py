# Problem 515
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def largestValues(self, root: Optional[TreeNode]) -> List[int]:
        res = []

        def dfs(node: TreeNode | None, row: int) -> None:
            if node is None:
                return

            if len(res) == row:
                res.append(node.val)
            else:
                res[row] = max(res[row], node.val)

            dfs(node.left, row + 1)
            dfs(node.right, row + 1)

        dfs(root, 0)

        return res
