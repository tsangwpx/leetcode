import collections
from typing import List


class Solution:
    def numMatchingSubseq(self, s: str, words: List[str]) -> int:
        counter = collections.Counter(words)
        compact_words = list(counter.items())

        res = 0

        for word, count in compact_words:
            try:
                pos = 0
                for ch in word:
                    pos = s.index(ch, pos) + 1
            except ValueError:
                pass
            else:
                res += count

        return res

    def numMatchingSubseq3(self, s: str, words: List[str]) -> int:
        counter = collections.Counter(words)
        compact_words = list(counter.items())

        table = collections.defaultdict(list)

        for i, (word, count) in enumerate(compact_words):
            it = iter(word)
            table[next(it)].append((it, count))

        for i, ch in enumerate(s):
            targets = table[ch]
            table[ch] = []

            for it, count in targets:
                table[next(it, None)].append((it, count))

        return sum([done for _, done in table[None]])

    def numMatchingSubseq2(self, s: str, words: List[str]) -> int:
        counter = collections.Counter(words)

        unique_count = len(counter)
        unique_words = [''] * unique_count
        multiples = [0] * unique_count
        records = [0] * unique_count
        table = collections.defaultdict(list)

        for i, (word, count) in enumerate(counter.items()):
            unique_words[i] = word
            multiples[i] = count

            for ch in set(word):
                table[ch].append(i)

        res = 0

        for i, ch in enumerate(s):
            for unique_idx in table[ch]:
                word = unique_words[unique_idx]
                progress = records[unique_idx]

                if word[progress] == ch:
                    progress += 1
                    records[unique_idx] = progress

                    if progress >= len(word):
                        res += multiples[unique_idx]

                        for word_ch in set(word):
                            indices = table[word_ch].copy()
                            indices.remove(unique_idx)
                            table[word_ch] = indices

        return res
