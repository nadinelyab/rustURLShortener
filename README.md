# Rust ShortURL.io
A simple URL Shortener in Rust using a Cryptographic Hash Function.

This shortener uses the first 8 digits of the SHA256 hash of a given url to create a short URL. As it grows it adds one more digit of the hash at a time to account for collisions.

## Why use a cryptographic hash function for a URL Shortener?
1. A cryptographic hash function makes collisions on the whole hash practically impossible.
2. One way of doing a URL shortener is to generate a random set of characters each time a new URL is given. However, using a cryptographic hash function allows any one with a URL registered with shortUrl to calculate the shortened URL themselves given the hash function used. That is because a URL always has the same short URL not matter how many times it is entered into the shortURL database. This also saves storage space.
3. Another way to do a URL shortener is to use a simple counter and map a long URL to the counter number. However, this provides less privacy than a cryptographic hash function. That is because one can easily guess a number and be redirected to some URL, thus revealing that this URL has been shortened with shortURL, as well as revealing the URL itself. However, guessing a valid hash is more difficult, which provides more privacy. 
4. When there are collisions we can easily increase the number of digits saved of the hash. This is modeled after the way git manages commit hashes. According to the [Pro Git Book](https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#Short-SHA-1) the URLShortener should be able to scale without requiring more than 11 characters.
