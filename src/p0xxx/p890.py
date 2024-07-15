from typing import List


class Solution:
    def findAndReplacePattern(self, words: List[str], pattern: str) -> List[str]:
        print("pattern", pattern)
        res = []
        for word in words:
            mapping = {}
            used = set()

            for a, b in zip(word, pattern):
                c = mapping.get(a)
                if c is None and b not in used:
                    used.add(b)
                    mapping[a] = c = b

                if c == b:
                    continue
                else:
                    break
            else:
                res.append(word)

            print(word, mapping)

        return res
