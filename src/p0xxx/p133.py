from leetcode_prelude import *


# Definition for a Node.
class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


# Problem 133
"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

from typing import Optional


class Solution:
    def cloneGraph(self, node: Optional["Node"]) -> Optional["Node"]:
        table = {}

        def clone(node):
            if node is None:
                return None

            new_node = table.get(node.val)

            if new_node is None:
                neighbors = []
                new_node = Node(val=node.val, neighbors=neighbors)
                table[node.val] = new_node
                for friend in node.neighbors:
                    neighbors.append(clone(friend))

            return new_node

        return clone(node)
