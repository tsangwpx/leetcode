from leetcode_prelude import *


# Problem 948
class Solution:
    def bagOfTokensScore(self, tokens: List[int], power: int) -> int:
        tokens.sort()

        left = 0
        right = len(tokens) - 1

        max_score = 0
        score = 0

        while True:
            while left <= right and power >= tokens[left]:
                power -= tokens[left]
                left += 1
                score += 1

            max_score = max(score, max_score)

            while left <= right and power < tokens[left] and score >= 1:
                power += tokens[right]
                right -= 1
                score -= 1

            if left > right or power < tokens[left]:
                break

        return max_score

    def bagOfTokensScore2(self, tokens: List[int], power: int) -> int:
        from collections import deque

        tokens.sort()
        tokens = deque(tokens)

        max_score = 0
        score = 0

        while True:
            while tokens and power >= tokens[0]:
                power -= tokens.popleft()
                score += 1

            max_score = max(score, max_score)

            while tokens and power < tokens[0] and score >= 1:
                score -= 1
                power += tokens.pop()

            if not tokens or power < tokens[0]:
                break

        return max_score
