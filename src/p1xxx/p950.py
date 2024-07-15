from leetcode_prelude import *


# Problem 950
class Solution:
    def deckRevealedIncreasing(self, deck: List[int]) -> List[int]:
        """Reveal card in increasing order"""

        # [] 1,2,3,4,5,6,7
        # [1] 3,4,5,6,7,2
        # [1,3] 5,6,7,2,4
        # [1,3,5] 7,2,4,6
        # [1,3,5,7] 4,6,2
        # [1,3,5,7,4] 2,6
        # [1,3,5,7,4,2] 6
        # [1,3,5,7,4,2,6]

        from collections import deque

        deck.sort(reverse=True)

        dq = deque()

        for card in deck:
            dq.rotate(1)  # appendleft(pop())
            dq.appendleft(card)

        deck = list(dq)
        return deck
