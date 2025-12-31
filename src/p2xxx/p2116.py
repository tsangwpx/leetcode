from leetcode_prelude import *


# Problem 2116
class Solution:
    def canBeValid(self, s: str, locked: str) -> bool:
        if len(s) % 2 == 1:
            return False

        def check(input: str, locked: str, open: str, close: str) -> bool:
            balance = 0
            freedom = 0

            for ch, immutable in zip(input, locked):
                immutable = immutable == "1"

                if immutable:
                    if ch == open:
                        balance += 1
                    else:
                        balance -= 1
                else:
                    freedom += 1

                if freedom + balance < 0:
                    return False

            return balance <= freedom

        return check(s, locked, "(", ")") and check(s[::-1], locked[::-1], ")", "(")
