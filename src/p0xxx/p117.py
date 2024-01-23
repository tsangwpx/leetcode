from leetcode_prelude import *


class Node:
    def __init__(
        self,
        val: int = 0,
        left: "Node" = None,
        right: "Node" = None,
        next: "Node" = None,
    ):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


# Problem 117
"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""


class Solution:
    def connect(self, root: "Node") -> "Node":
        parent_head = Node(next=root)
        head = tail = Node()

        while parent_head.next is not None:
            parent_node = parent_head.next
            while parent_node is not None:
                if parent_node.left is not None:
                    tail.next = parent_node.left
                    tail = tail.next
                if parent_node.right is not None:
                    tail.next = parent_node.right
                    tail = tail.next

                parent_node = parent_node.next

            parent_head.next = head.next
            head.next = None
            tail = head

        return root
