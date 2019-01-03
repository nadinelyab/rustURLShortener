//INTERESTING INFO ON SHA1: https://stackoverflow.com/questions/34764195/how-does-git-create-unique-commit-hashes-mainly-the-first-few-characters

// Why use a cryptographic function?
// 1) Reduce the amount of collisions
// 2) Unlike using a randomness function, any one with a URL can know what the short url is. 
// 3) On the other hand, you cannot guess a hash value and be redirected to some site because you must know the original
// url to register it with its short url. This allows privacy for sites that cannot be easily found on the internet.
// 4) When there are collisions we can easily increase the number of digits saved of the hash. This allows for easy scalability.
// according to the above stackoverflow post it is unlikely that we would need more than 11 chars.

use std::io;
extern crate url;
use url::{Url};
use std::collections::HashMap;
extern crate crypto;
use crypto::sha2::Sha256;
use crypto::digest::Digest;


fn main() {
	let mut HASH_LENGTH: usize = 8;
	let mut url_mapping: HashMap<String, String> = HashMap::new();
	println!("WELCOME to the url shortener ShortURL.\nTo begin enter a url you wish to shorten.\n");
	loop {
		let input = readUserInput();
		if input == "exit" || input == "quit"{ 
			break;
		} else {
			match Url::parse(&input){
				Err(_) => println!("You entered an invalid URL. Please try again.\n"),
				Ok(url) =>  {
					if url.host_str() == Some("shorturl.io") {
						// get original url from map
				 		getLongURLFromHash(url.path(), &url_mapping);
					} else {
						let mut hex: String = hashURL(&input);
						// If we have a hash collision && its not the same url, add one to the hash length.
						if url_mapping.contains_key(&hex.to_string()) && *url_mapping.get::<str>(&hex.to_string()).unwrap() != input {
							HASH_LENGTH += 1;
						}
						hex.truncate(HASH_LENGTH);
						hex.insert_str(0, "/");
						println!("Your shortened URL is: https://shorturl.io{}\n", hex);
						if !url_mapping.contains_key(&hex.to_string()) {
							url_mapping.insert(String::from(hex), String::from(input));
						}
					}
				}
			}
		}
	}
}

fn readUserInput() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().to_string();
}

fn getLongURLFromHash(hash: &str, url_mapping: &HashMap<String, String>) {
	match url_mapping.get(&String::from(hash)) {
		Some(long_url) => println!("Redirecting to {}\n", long_url),
		None => println!("That short URL does not exist. Please try again.\n")
	}
}

fn hashURL(input: &str) -> String{
	let mut hasher = Sha256::new();
	hasher.input_str(&input);
	return hasher.result_str();
}
