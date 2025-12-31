from leetcode_prelude import *


# Problem 921
class Solution:
    def minAddToMakeValid(self, s: str) -> int:
        """
        Keep tracking the balance of open and closing parentheses.
        """

        insertions = 0
        balance = 0

        for ch in s:
            if ch == "(":
                # increase the count
                balance += 1
            elif balance <= 0:
                # insert open parenthesis to cancel out this closing parenthesis
                insertions += 1
            else:
                # decrease the count
                balance -= 1

        # add closing parenthesis to the end
        insertions += balance

        return insertions
