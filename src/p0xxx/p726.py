# Problem 726
import re

from leetcode_prelude import *


class Solution:
    _PATTERN = re.compile(
        r"(?P<name>[A-Z][a-z]*)|(?P<count>[0-9]+)|(?P<lpar>\()|(?P<rpar>\))"
    )

    def countOfAtoms(self, formula: str) -> str:
        from collections import defaultdict

        stack = [defaultdict(int)]

        matches = list(self._PATTERN.finditer(formula))
        eom = len(matches)  # end of matches

        idx = 0
        while idx < eom:
            name, lpar, rpar = matches[idx].group("name", "lpar", "rpar")
            idx += 1

            count = None
            if idx < eom and matches[idx].group("count") is not None:
                # Eat count if next match is a count
                # So this match must be either name or rpar.
                assert name is None != rpar is None, (name, rpar)

                count = int(matches[idx].group("count"))
                idx += 1

            if name is not None:
                count = count or 1
                stack[-1][name] += count
            elif lpar == "(":
                stack.append(defaultdict(int))
            elif rpar == ")":
                group = stack.pop()
                top = stack[-1]
                count = count or 1
                for key, value in group.items():
                    top[key] += value * count
            else:
                raise AssertionError("unreachable", matches[idx - 1])

        top = stack.pop()
        return "".join(f"{k}{v}" if v >= 2 else f"{k}" for k, v in sorted(top.items()))
