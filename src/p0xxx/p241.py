from leetcode_prelude import *


# Problem 241
class Solution:
    def diffWaysToCompute(self, expression: str) -> List[int]:
        from collections import deque

        idx = 0
        expect_operator = False

        operands = []
        operators = []

        while idx < len(expression):
            if expect_operator:
                assert expression[idx] in "+-*"
                operators.append(expression[idx])
                idx += 1
            else:
                start = idx
                idx += 1
                while idx < len(expression) and expression[idx] in "0123456789":
                    idx += 1
                operands.append(int(expression[start:idx]))

            expect_operator = not expect_operator

        if len(operands) == 1:
            return operands

        # print(operands)
        # print(operators)
        memo: dict[tuple[int, int], list[int]] = {}

        def compute(start: int, stop: int) -> list[int]:
            key = (start, stop)
            if key in memo:
                return memo[key]

            size = stop - start
            if size == 1:
                memo[key] = [operands[start]]
            elif size >= 2:
                res = []
                for mid in range(start + 1, stop):
                    op = operators[mid - 1]
                    left_results = compute(start, mid)
                    right_results = compute(mid, stop)

                    for left in left_results:
                        for right in right_results:
                            if op == "+":
                                val = left + right
                            elif op == "-":
                                val = left - right
                            elif op == "*":
                                val = left * right
                            else:
                                raise AssertionError(op)
                            res.append(val)

                memo[key] = res
            else:
                raise AssertionError(size)

            return memo[key]

        return compute(0, len(operands))
