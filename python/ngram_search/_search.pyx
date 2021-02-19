# cython: language_level=3, boundscheck=False
import typing


from libc.stdlib cimport malloc, free, realloc
from libc.stdio cimport FILE


ctypedef int (*compar_t)(const void*, const void*)


cdef extern from "<stdlib.h>":
    void qsort(void* base, size_t nmemb, size_t size, compar_t compar)


cdef extern from "stdio.h":
    FILE *fdopen(int fd, const char* mode)
    int fseek(FILE* stream, long offset, int whence)
    int getc(FILE* stream)
    size_t fread(void* ptr, size_t size, size_t nmemb, FILE* stream)


cdef extern from *:
    """
    #if defined(_WIN32) || defined(MS_WINDOWS) || defined(_MSC_VER)
      #include <winsock2.h>
    #else
      #include <arpa/inet.h>
    #endif
    """
    unsigned long ntohl(unsigned long number)


class SearchError(Exception):
    """Error searching in file."""


cdef struct Hit:
    unsigned long id
    unsigned char count
    unsigned char total_ngrams


cdef struct Hits:
    Hit* hits
    size_t len


cdef struct Match:
    unsigned int id
    float score


cdef char read_u8(FILE* file):
    return getc(file)


cdef long read_u32(FILE* file):
    cdef long number

    if fread(<char*>&number, 1, 4, file) != 4:
        raise SearchError("EOF")
    return ntohl(number)


cdef Hits search_trigrams(FILE* file, unsigned int* trigram):
    cdef int character
    cdef int size
    cdef int found
    cdef long c, p
    cdef Hits hits

    fseek(file, 0, 0)

    for i in range(3):
        character = trigram[i]

        # Check that this is a branch
        if read_u8(file) != 1:
            raise SearchError("Invalid branch record")

        # Look for the character we need
        size = read_u32(file)
        found = 0
        for _ in range(size):
            c = read_u32(file)
            p = read_u32(file)
            if c == character:
                found = 1
                fseek(file, p, 0)
                break

        if not found:
            # Return empty array
            hits.hits = <Hit*>malloc(0)
            hits.len = 0
            return hits

    # Read leaves
    if read_u8(file) != 2:
        raise SearchError("Invalid leaf record")
    size = read_u32(file)
    hits.hits = <Hit*>malloc(size * sizeof(Hit))
    try:
        hits.len = 0
        for _ in range(size):
            hits.hits[hits.len].id = read_u32(file)
            hits.hits[hits.len].count = read_u8(file)
            hits.hits[hits.len].total_ngrams = read_u8(file)
            hits.len += 1
        return hits
    except:
        free(hits.hits)
        raise


cdef int match_compar(const void* a, const void* b):
    cdef Match* ma
    cdef Match* mb
    cdef float diff
    ma = <Match*>a
    mb = <Match*>b
    if mb.score < ma.score:
        return -1
    else:
        return 1


def search(
    file: typing.BinaryIO,
    trigrams: typing.List[(typing.Tuple[int, ...], int)],
    threshold: float,
) -> typing.List[typing.Tuple[int, float]]:
    cdef FILE* c_file
    cdef Hits* hits = NULL
    cdef unsigned int[3] c_trigram
    cdef int total_ngrams
    cdef int* counts = NULL
    cdef size_t* positions = NULL
    cdef size_t nb_trigrams
    cdef unsigned long smallest_id
    cdef int match_total_ngrams
    cdef int shared
    cdef int allgrams
    cdef float score
    cdef Match* matches = NULL
    cdef size_t matches_len
    cdef size_t matches_size

    c_file = fdopen(file.fileno(), 'rb')

    try:
        nb_trigrams = len(trigrams)
        total_ngrams = sum(c for _, c in trigrams)
        counts = <int*>malloc(nb_trigrams * sizeof(int))
        for i in range(nb_trigrams):
            counts[i] = trigrams[i][1]

        # Look for all trigrams
        hits = <Hits*>malloc(nb_trigrams * sizeof(Hits))
        for i in range(nb_trigrams):
            hits[i].hits = NULL
        for i in range(nb_trigrams):
            trigram, _ = trigrams[i]
            for j in range(3):
                c_trigram[j] = trigram[j]
            hits[i] = search_trigrams(c_file, c_trigram)

        positions = <size_t*>malloc(nb_trigrams * sizeof(size_t))
        for i in range(nb_trigrams):
            positions[i] = 0

        # Build a list of results by merging all those hits together
        matches_size = 16
        matches_len = 0
        matches = <Match*>malloc(matches_size * sizeof(Match))
        while True:
            # Find the smallest next element and its count
            smallest_id = 0
            match_total_ngrams = -1
            for i in range(nb_trigrams):
                if positions[i] < hits[i].len:
                    if (
                        match_total_ngrams == -1
                        or hits[i].hits[positions[i]].id < smallest_id
                    ):
                        smallest_id = hits[i].hits[positions[i]].id
                        match_total_ngrams = hits[i].hits[positions[i]].total_ngrams

            # No next element: we're done
            if match_total_ngrams == -1:
                break

            # Compute the count and move forward
            shared = 0
            for i in range(nb_trigrams):
                if positions[i] < hits[i].len:
                    if hits[i].hits[positions[i]].id == smallest_id:
                        shared += min(counts[i], hits[i].hits[positions[i]].count)
                        positions[i] += 1

            # Compute score
            allgrams = total_ngrams + match_total_ngrams - shared
            score = shared / allgrams

            # Threshold
            if score < threshold:
                continue

            # Store result
            if matches_len + 1 > matches_size:
                # Grow result array
                matches_size *= 2
                matches = <Match*>realloc(matches, matches_size * sizeof(Match))
            matches[matches_len].id = smallest_id
            matches[matches_len].score = score
            matches_len += 1

        # Sort results
        qsort(matches, matches_len, sizeof(Match), match_compar)

        # Convert to Python array
        results = []
        for i in range(matches_len):
            results.append((matches[i].id, matches[i].score))
        return results
    finally:
        if hits != NULL:
            for i in range(nb_trigrams):
                if hits[i].hits != NULL:
                    free(hits[i].hits)
            free(hits)
        if matches != NULL:
            free(matches)
        if positions != NULL:
            free(positions)
        if counts != NULL:
            free(counts)
