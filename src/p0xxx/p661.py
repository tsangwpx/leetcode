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


# Problem 661
import numpy as np
from scipy.signal import convolve2d

dtype = np.int16
kernel = np.ones((3, 3), dtype=dtype)
convolve2d(kernel, kernel, "same")


class Solution:
    def imageSmoother(self, img: List[List[int]]) -> List[List[int]]:
        sum2d = convolve2d(np.array(img, dtype=dtype), kernel, "same")
        count2d = convolve2d(np.ones_like(sum2d), kernel, "same")
        sum2d //= count2d
        # print(count2d, count2d.dtype)
        # print(sum2d, sum2d.dtype)

        return sum2d.tolist()

    def imageSmoother2(self, img: List[List[int]]) -> List[List[int]]:
        m = len(img)
        n = len(img[0])

        res = [[0] * n for _ in range(m)]

        for i in range(m):
            for j in range(n):
                r0 = max(0, i - 1)
                r1 = min(m, i + 2)
                c0 = max(0, j - 1)
                c1 = min(n, j + 2)

                # print(f"({i}, {j}): R:{r0}:{r1}, C:{c0}:{c1}")

                total = 0
                count = 0

                for r in range(r0, r1):
                    for c in range(c0, c1):
                        total += img[r][c]
                        count += 1

                res[i][j] = int(total / count)

        return res
