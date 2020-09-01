import typing
import unicodedata

from . import _search


__all__ = ['SearchError', 'Ngrams']


SearchError = _search.SearchError


END_MARKER = '$'


def string_trigrams(string: str) -> (int, int, int):
    if len(string) == 0:
        yield END_MARKER, END_MARKER, END_MARKER
    else:
        end = ord(END_MARKER)
        c1 = c2 = end
        for c3 in string:
            c3 = ord(c3)
            yield c1, c2, c3
            c1 = c2
            c2 = c3
        yield c1, c2, end
        yield c2, end, end


class Ngrams(object):
    def __init__(self, path):
        self._file = open(path, 'rb')

    def search(
        self,
        string: str,
        threshold: float = 0.3,
    ) -> typing.List[typing.Tuple[int, float]]:
        # Normalize string
        string = string.lower()
        string = unicodedata.normalize('NFC', string)

        # Build list of trigrams
        trigrams = {}
        for trigram in string_trigrams(string):
            if trigram in trigrams:
                trigrams[trigram] += 1
            else:
                trigrams[trigram] = 1
        trigrams = list(trigrams.items())

        # Call search code
        return _search.search(self._file, trigrams, threshold)
