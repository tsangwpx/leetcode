from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Problem 139
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        from heapq import heappop, heappush

        size = len(s)
        pq = [0]
        used = [False] * (size + 1)
        used[0] = True

        while pq:
            start = -heappop(pq)

            for word in wordDict:
                dest = start + len(word)
                if dest > size or used[dest]:
                    continue

                if s[start:dest] == word:
                    # since the equality above, this never out of bounds
                    used[dest] = True

                    if dest == size:
                        # early termination
                        return True

                    heappush(pq, -dest)

        return used[size]

    def wordBreak2(self, s: str, wordDict: List[str]) -> bool:
        size = len(s)
        dp = [False] * (size + 1)
        dp[0] = True

        for i in range(size):
            if not dp[i]:
                # We cannot start here
                continue

            for word in wordDict:
                dest = i + len(word)
                if s[i:dest] == word:
                    # since the equality above, this never out of bounds
                    dp[dest] = True

            if dp[size]:
                # early termination
                break

        return dp[size]
