from leetcode_prelude import *


# Problem 2471
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def minimumOperations(self, root: Optional[TreeNode]) -> int:
        count = 0
        curr_nodes = [root]
        next_nodes = []

        while curr_nodes:
            values = []
            for node in curr_nodes:
                values.append(node.val)

                if node.left is not None:
                    next_nodes.append(node.left)

                if node.right is not None:
                    next_nodes.append(node.right)

            indices = list(range(len(values)))
            indices.sort(key=lambda s: values[s])
            # print(indices)

            for pos in range(len(values)):
                while (idx := indices[pos]) != pos:
                    count += 1

                    indices[pos] = indices[idx]
                    indices[idx] = idx

            curr_nodes.clear()
            curr_nodes, next_nodes = next_nodes, curr_nodes

        return count
