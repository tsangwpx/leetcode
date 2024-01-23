from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 1436
class Solution:
    def destCity(self, paths: List[List[str]]) -> str:
        from collections import defaultdict

        cities = set([s for _, s in paths])
        for src, _ in paths:
            cities.discard(src)

        return cities.pop()
