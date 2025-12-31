from leetcode_prelude import *


# Problem 2491
class Solution:
    def dividePlayers(self, skill: List[int]) -> int:
        from collections import Counter

        chemistry = 0

        total = -1

        counter = Counter(skill)
        sorted_keys = sorted(counter.keys())
        key_size = len(sorted_keys)

        for idx in range(0, key_size // 2):
            key = sorted_keys[idx]
            key2 = sorted_keys[key_size - 1 - idx]
            count = counter[key]
            count2 = counter[key2]

            if count != count2:
                return -1

            if total < 0:
                total = key + key2
            elif total != (key + key2):
                return -1

            chemistry += key * key2 * count

        if key_size % 2 == 1:
            # the middle item must be paired with itself
            key = sorted_keys[key_size // 2]
            count = counter[key]

            if total >= 0 and key * 2 != total:
                # total is bad
                return -1
            if count % 2 != 0:
                # count must be even
                return -1

            chemistry += key * key * count // 2

        return chemistry
