from leetcode_prelude import *


# Problem 895
class FreqStack:
    def __init__(self):
        from collections import Counter

        self.counter = Counter()
        self.stacks: list[list[int]] = []

    def push(self, val: int) -> None:
        idx = self.counter[val]
        self.counter[val] += 1

        if idx == len(self.stacks):
            self.stacks.append([])

        self.stacks[idx].append(val)

    def pop(self) -> int:
        stack = self.stacks[-1]
        val = stack.pop()
        self.counter[val] -= 1
        if not stack:
            self.stacks.pop()

        return val


# Your FreqStack object will be instantiated and called as such:
# obj = FreqStack()
# obj.push(val)
# param_2 = obj.pop()
