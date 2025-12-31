from leetcode_prelude import *


# Problem 2583
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def kthLargestLevelSum(self, root: Optional[TreeNode], k: int) -> int:
        from heapq import heappush, heappop

        pq = []
        row = [root]
        next_row = []

        while row:
            next_row.clear()
            total = 0

            for node in row:
                total += node.val

                if node.left:
                    next_row.append(node.left)

                if node.right:
                    next_row.append(node.right)

            if len(pq) > k:
                heappop(pq)

            heappush(pq, total)
            row, next_row = next_row, row

        if len(pq) > k:
            heappop(pq)

        if len(pq) == k:
            return heappop(pq)
        else:
            return -1
