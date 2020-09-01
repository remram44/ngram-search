import os
import unittest

from ngram_search import Ngrams, string_trigrams


ROOT_DIR = os.path.dirname(__file__)


class TestSearch(unittest.TestCase):
    def test_trigrams(self):
        self.assertEqual(
            list(string_trigrams('spam')),
            [
                tuple(ord(c) for c in trigram)
                for trigram in ['$$s', '$sp', 'spa', 'pam', 'am$', 'm$$']
            ],
        )

    def test_search(self):
        ngrams = Ngrams(os.path.join(ROOT_DIR, 'test.db'))
        self.assertEqual(
            ngrams.search("ham", 0.24),
            [(1, 1.0), (2, 0.25)],
        )
        self.assertEqual(
            ngrams.search("spa", 0.2),
            [(0, 0.375)],
        )


if __name__ == '__main__':
    unittest.main()
