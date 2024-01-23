from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# Definition for a Node.
class Node:
    def __init__(self, x: int, next: "Node" = None, random: "Node" = None):
        self.val = int(x)
        self.next = next
        self.random = random


# Problem 138
class Solution:
    def copyRandomList(self, head: "Optional[Node]") -> "Optional[Node]":
        dummy = Node(0)
        dummy.next = head

        table = {}

        node = dummy
        while node.next is not None:
            old_next = node.next
            new_next = Node(
                old_next.val,
                next=old_next.next,
                random=old_next.random,
            )
            table[id(old_next)] = new_next
            # print(
            #     "Table",
            #     old_next.val,
            #     old_next.random.val if old_next.random is not None else False,
            # )

            node.next = new_next
            node = node.next

        for _, new_node in table.items():
            if new_node.random is not None:
                new_random = table[id(new_node.random)]
                new_node.random = new_random

        return dummy.next
