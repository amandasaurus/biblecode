Bible Code finder
=================

In 1997 Michael Drosnin published [The Bible Code](https://en.wikipedia.org/wiki/The_Bible_Code_%28book%29), a book about [bible codes](https://en.wikipedia.org/wiki/Bible_code), or [equidistant letter sequences](https://en.wikipedia.org/wiki/Bible_code#Equidistant_Letter_Sequence_method). This programme helps you find and search for them.

Installation
============



Usage
=====

Getting some sample data
------------------------

This command will download the [King James Bible from Project Gutenberg](https://www.gutenberg.org/ebooks/10), remove the header & footer, save it in a file `kjv.txt`. This file (or any plain text file) can be searched.

    curl  https://www.gutenberg.org/ebooks/10.txt.utf-8 | sed -n '/^\*\*\* START OF THIS PROJECT GUTENBERG EBOOK/,/^\*\*\* END OF THIS PROJECT GUTENBERG EBOOK/p' 10.txt.utf-8 | sed  -e '1d' -e '$d' > kjv.txt

Looking for a string
--------------------

    cargo run FILENAME_TO_SEARCH.txt STRING_TO_SEARCH_FOR


Copyright
=========

This is currently available under the GNU Affero GPL. If you'd like a different licence, please contact Rory McCann <rory@technomancy.org>
