from leetcode_prelude import *


# Problem 1963
class Solution:
    def minSwaps(
        self,
        s: str,
    ) -> int:
        # first, count the number of unbalanced open brackets
        count = 0

        for ch in s:
            if ch == "[":
                count += 1
            elif count >= 1:
                count -= 1

        # Now, there are `count` unbalanced open brackets
        # The pattern can be simplified as `count` closing brackets + `count` open brackets
        # Example:
        # "]["  -> "[]" 1 swap
        # "]][[" -> "[][]" 1 swaps
        # "]]][[[" -> "[][][]" 2 swaps
        # "]]]][[[[" -> "[][][][]" 2 swaps
        # "]]]]][[[[[" -> "[][][][][]" 3 swaps
        # ...
        # So, if there is `count` unbalanced brackets, we need `(count + 1) // 2` swaps

        return (count + 1) // 2
