from leetcode_prelude import *


# Problem 1190
class Solution:
    def reverseParentheses(self, s: str) -> str:
        """
        O(N**2) because the inner string may be reversed many times

        Also see O(N) wormhole solution by lee215
        """
        stack = [[]]

        for ch in s:
            if ch == "(":
                stack.append([])
            elif ch == ")":
                parts = stack.pop()
                parts.reverse()
                stack[-1].extend(parts)
            else:
                stack[-1].append(ch)

        return "".join(stack.pop())
