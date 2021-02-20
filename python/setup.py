from setuptools import Extension, setup
import io
import os
import platform


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


with io.open('README.rst', encoding='utf-8') as fp:
    description = fp.read()
extensions = _cython_exts([
    Extension(
        'ngram_search._search',
        ['ngram_search/_search.pyx'],
        libraries=['ws2_32'] if platform.system() == 'Windows' else [],
    ),
])
setup(
    name='ngram-search',
    version='0.1.2',
    packages=['ngram_search'],
    package_data={'ngram_search': ['*.pxd']},
    ext_modules=extensions,
    zip_safe=False,
    description="Ngram-based indexing of strings into a binary file",
    author="Remi Rampin",
    author_email="remirampin@gmail.com",
    project_urls={
        'Homepage': 'https://github.com/remram44/ngram-search',
        'Source': 'https://github.com/remram44/ngram-search',
        'Tracker': 'https://github.com/remram44/ngram-search/issues',
    },
    long_description=description,
    license='MIT',
    keywords=['ngram', 'indexing', 'full-text', 'text-search', 'full-text-search'],
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Cython',
        'Topic :: Database',
        'Topic :: Text Processing',
    ],
)
