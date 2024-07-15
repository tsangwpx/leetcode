from leetcode_prelude import *


# Problem 3021
class Solution:
    def flowerGame(self, n: int, m: int) -> int:
        # count many odd and even numbers in [1, n]
        # x + y must be odd for Alice to win
        # x + y must be either (odd, even) or (even, odd) pairs
        n_odd = (n + 1) // 2
        m_odd = (m + 1) // 2
        n_even = n // 2
        m_even = m // 2

        return n_odd * m_even + n_even * m_odd
