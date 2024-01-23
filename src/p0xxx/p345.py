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


# Problem 345
class Solution:
    def reverseVowels(self, s: str) -> str:
        index = 0
        last = len(s) - 1

        while index < last:
            while index < last and s[index] not in "aeiouAEIOU":
                index += 1

            while index < last and s[last] not in "aeiouAEIOU":
                last -= 1

            if index == last:
                break

            s = s[0:index] + s[last] + s[index + 1 : last] + s[index] + s[last + 1 :]
            index += 1
            last -= 1

        return s
