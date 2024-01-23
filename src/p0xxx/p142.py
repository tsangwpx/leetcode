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


# Problem 142
class Solution:
    def detectCycle(self, head: ListNode) -> ListNode:
        hare = head
        tort = head

        while hare is not None:
            hare = hare.next
            if hare is None:
                break

            hare = hare.next
            tort = tort.next

            if hare == tort:
                break

        if hare is None:
            return None

        hare = head
        while hare != tort:
            hare = hare.next
            tort = tort.next

        return hare
