from setuptools import Extension, setup
import os

_DONT_WANT = ('', '0', 'n', 'no', 'false')

def _cython_exts(extensions):
    if os.environ.get('BUILD_CYTHON', '').lower() not in _DONT_WANT:
        from Cython.Build import cythonize
        # Use annotate=True to debug
        return cythonize(extensions)
    else:
        for extension in extensions:
            sources = []
            for sfile in extension.sources:
                path, ext = os.path.splitext(sfile)
                if ext in ('.pyx', '.py'):
                    sfile = path + '.c'
                sources.append(sfile)
            extension.sources[:] = sources
        return extensions


setup(
    name='ngram-search',
    version='0.1',
    packages=['ngram_search'],
    package_data={'ngram_search': ['*.pxd']},
    ext_modules=_cython_exts([Extension('ngram_search._search', ['ngram_search/_search.pyx'])]),
    zip_safe=False,
)
