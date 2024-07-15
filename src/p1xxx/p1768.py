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


# Problem 1768
class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        from itertools import chain, zip_longest

        s = ""

        for i in range(min(len(word1), len(word2))):
            s += word1[i]
            s += word2[i]

        if len(word1) >= len(word2):
            s += word1[len(word2) :]
        else:
            s += word2[len(word1) :]

        return s

    def mergeAlternately2(self, word1: str, word2: str) -> str:
        from itertools import chain, zip_longest

        return "".join(chain.from_iterable(zip_longest(word1, word2, fillvalue="")))
