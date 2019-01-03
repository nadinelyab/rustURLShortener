# rustURLShortener
A simple URL Shotener in Rust using a Cryptographic Hash Function

## Why use a cryptographic hash function for a URL Shortener?
1. Reduce the amount of collisions
2. Unlike using a randomness function, any one with a URL can know what the short url is. 
3. On the other hand, you cannot guess a hash value and be redirected to some site because you must know the original
url to register it with its short url. This allows privacy for sites that cannot be easily found on the internet.
4. When there are collisions we can easily increase the number of digits saved of the hash. This allows for easy scalability.
according to [this stackoverflow post](https://stackoverflow.com/questions/34764195/how-does-git-create-unique-commit-hashes-mainly-the-first-few-characters)
it is unlikely that we would need more than 11 chars.
