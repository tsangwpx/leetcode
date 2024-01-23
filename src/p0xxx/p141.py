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


# Problem 141
class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        hare = head
        tort = head

        while True:
            if hare is None:
                break

            hare = hare.next
            if hare is None:
                break

            hare = hare.next
            tort = tort.next

            if hare == tort:
                break

        # print(hare, tort)
        return hare is not None and hare == tort
