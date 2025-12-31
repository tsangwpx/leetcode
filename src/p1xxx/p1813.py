from leetcode_prelude import *


# Problem 1813
class Solution:
    def areSentencesSimilar(self, sentence1: str, sentence2: str) -> bool:
        parts1 = sentence1.split()
        parts2 = sentence2.split()

        len1 = len(parts1)
        len2 = len(parts2)

        if len1 < len2:
            # Make sure parts1 is longer
            parts1, parts2 = parts2, parts1
            len1, len2 = len2, len1

        idx1 = idx2 = 0

        # Skip equal words from the front
        while idx1 < len1 and idx2 < len2 and parts1[idx1] == parts2[idx2]:
            idx1 += 1
            idx2 += 1

        if idx2 == len2:
            # Shorter sentence is the prefix of the longer one.
            return True

        # Skip equal words from the back
        while idx1 < len1 and idx2 < len2 and parts1[len1 - 1] == parts2[len2 - 1]:
            len1 -= 1
            len2 -= 1

        # Finally, both pointers of shorter sentence should meet reach each other
        return idx2 == len2
