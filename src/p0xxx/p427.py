from leetcode_prelude import *


# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight


# Problem 427
"""
# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight
"""


class Solution:
    def construct(self, grid: List[List[int]]) -> "Node":
        def build_node(row, col, size_max, size=None):
            """
            Build a region node whose top-left concer is (row, col)

            size_max specifies the maximum side of the region.
            size is the computed value for reuse if not None.
            """

            symbol = grid[row][col]

            if size is None:
                size = 1

                while size < size_max:
                    invalid = False

                    new_size = size * 2

                    # check the right block of the valid square
                    for i in range(row, row + size):
                        if invalid:
                            break

                        for j in range(col + size, col + new_size):
                            if grid[i][j] != symbol:
                                invalid = True
                                break

                    # check the two blocks below the valid squares
                    for i in range(row + size, row + new_size):
                        if invalid:
                            break

                        for j in range(col, col + new_size):
                            if grid[i][j] != symbol:
                                invalid = True
                                break

                    if invalid:
                        break

                    # everything is good.
                    # We double the side of the region with identical symbol.
                    size = new_size

            if size == size_max:
                return Node(val=symbol, isLeaf=True)
            else:
                # the symbol cannot occupy the whole region
                # save the size for reuse
                half_max = size_max // 2

                # For the top-left child node,
                # we pass the computed size to avoid duplicated computaton

                node = Node(isLeaf=False)
                node.topLeft = build_node(row, col, half_max, size)
                node.topRight = build_node(row, col + half_max, half_max)
                node.bottomLeft = build_node(row + half_max, col, half_max)
                node.bottomRight = build_node(row + half_max, col + half_max, half_max)
                return node

        return build_node(0, 0, len(grid))
