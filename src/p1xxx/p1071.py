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


# Problem 1071
class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        if set(str1) != set(str2):
            return ""

        import math

        gcd = math.gcd(len(str1), len(str2))

        for k in range(gcd, 0, -1):
            if gcd % k != 0:
                continue

            factor = str1[0:k]

            if all(factor == str2[i : i + k] for i in range(0, len(str2), k)) and all(
                factor == str1[i : i + k] for i in range(0, len(str1), k)
            ):
                return True

        return ""
