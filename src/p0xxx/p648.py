from leetcode_prelude import *


# Problem 648
class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        """
        Trie is better structure to look up words.
        but the data size is small here.
        """

        roots = set()
        length = 0
        slices = []

        dictionary.sort(key=lambda s: len(s))

        for word in dictionary:
            redundant = False

            for slice_ in slices:
                if word[slice_] in roots:
                    redundant = True
                    break

            if redundant:
                continue

            roots.add(word)

            if len(word) > length:
                length = len(word)
                slices.append(slice(0, len(word)))

        parts = []

        for word in sentence.split():
            for slice_ in slices:
                root = word[slice_]
                if root in roots:
                    parts.append(root)
                    break
            else:
                parts.append(word)

        return " ".join(parts)

    def replaceWords2(self, dictionary: List[str], sentence: str) -> str:
        """
        Trie is better structure to look up words.
        but the data size is small here.
        """
        words = set(dictionary)

        parts = []

        for word in sentence.split():
            for stop in range(1, len(word)):
                root = word[:stop]

                if root in words:
                    parts.append(root)
                    break
            else:
                parts.append(word)

        return " ".join(parts)
