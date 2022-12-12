# age encrypt/decrypt tests

## Age Key Use

* create a passprase protected key.age file
* save the public key 
* save the generated phrase in a key-store or remote machine (s3?)
* encrypt files/messages with the public key (no password required)
* decrypt with the key.age file (prompted for pw)

## Summary

Based on [age encryption crate](https://docs.rs/age/0.8.1/age/) `age` provides strong encryption and is well suited 
for 'data-at-rest'.  It's slow, so not so good for point-to-point transfers when you need to quickly encrypt and 
decrypt payloads.  for that, openssl/aes is maybe/probably the best choice...

###### dpw | 2022-10-26
