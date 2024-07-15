from leetcode_prelude import *


# Problem 1717
class Solution:
    def maximumGain(self, s: str, x: int, y: int) -> int:
        a = "a"
        b = "b"

        # make sure x > y
        if x < y:
            x, y = y, x
            a, b = b, a

        res = 0

        # first, remove all possible ab (more points)

        stack = []
        for ch in s:
            if stack and stack[-1] == a and ch == b:
                stack.pop()
                res += x
            else:
                stack.append(ch)

        # second, remove all ba (less points)

        stack2 = []
        for ch in stack:
            if stack2 and stack2[-1] == b and ch == a:
                stack2.pop()
                res += y
            else:
                stack2.append(ch)

        return res
