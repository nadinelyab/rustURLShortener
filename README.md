# Rust ShortURL.io
A simple URL Shortener in Rust using a Cryptographic Hash Function.

This shortener uses the first few digits of the SHA256 hash of a given url to create a short URL.

## Why use a cryptographic hash function for a URL Shortener?
1. Reduce the amount of collisions
2. One way of doing a URL shortener is to generate a random set of characters each time a new URL is given. However, using a cryptographic hash function has the added bonus any one with a URL registered with shortUrl can calculate the shortened URL themselves if they know the hash function used. That is because a URL always has the same short URL not matter how many times it is entered into the shortURL database. This also saves storage space.
3. Another way to do a URL shortener is to use a simple counter and map a long URL to the counter number. However, this provides less privacy than a cryptographic hash function. That is because one can easily guess a number and be redirected to some URL, thus revealing that this URL has been shortenered with shortURL, as well as revealing the URL itself. However, guessing a valid hash is more difficult, which provides more privacy. 
4. When there are collisions we can easily increase the number of digits saved of the hash. This allows for easy scalability. According to [this stackoverflow post](https://stackoverflow.com/questions/34764195/how-does-git-create-unique-commit-hashes-mainly-the-first-few-characters) it is unlikely that we would need more than 11 chars.
