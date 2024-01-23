import itertools
from typing import List, Optional


# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        res = []

        bfs = [] if root is None else [root]

        while bfs:
            res.append(bfs[-1].val)
            bfs2 = []
            for node in bfs:
                if node.left is not None:
                    bfs2.append(node.left)
                if node.right is not None:
                    bfs2.append(node.right)
            bfs = bfs2

        return res


def main():
    return


if __name__ == '__main__':
    main()
