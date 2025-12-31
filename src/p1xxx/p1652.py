from leetcode_prelude import *


# Problem 1652
class Solution:
    def decrypt(self, code: List[int], k: int) -> List[int]:
        """
        1. Sliding window of fixed size
        2. First, compute the window location
        3. Second, configure the elements which enter and leave the window.
        """
        res = [0] * len(code)
        if k == 0:
            return res

        size = len(code)

        if k > 0:
            # since 0 < k < n, the first value is not wrapping around the array
            value = sum(code[1 : 1 + k])
            head = k + 1  # k + 1 = n become 0 in the loop
            tail = 1
        else:
            value = sum(code[k:])
            head = 0
            tail = size + k

        for idx in range(len(code)):
            res[idx] = value

            if head >= size:
                head = 0

            if tail >= size:
                tail = 0

            value -= code[tail]
            value += code[head]

            head += 1
            tail += 1

        return res
