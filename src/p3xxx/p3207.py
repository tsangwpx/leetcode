from leetcode_prelude import *


# Problem 3207
class Solution:
    def maximumPoints(self, enemyEnergies: List[int], currentEnergy: int) -> int:
        minimum = min(enemyEnergies)

        if currentEnergy < minimum:
            return 0

        # Gain 1 points
        points = 1
        currentEnergy -= minimum

        currentEnergy += sum(enemyEnergies) - minimum
        points += currentEnergy // minimum

        return points
