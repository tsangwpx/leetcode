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


# Problem 392
class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        find = t.find

        pos = 0
        for ch in s:
            pos = find(ch, pos) + 1
            if pos == 0:
                return False

        return True
