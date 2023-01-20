# To Do

Create separate encode / decode switches to enable chacha saving keys/nonce.

## CLI

chacha --encrypt --key-file filename --plain-text filename --cipher-text filename
chacha --decrypt --key-file filename --cipher-test filename --plain-test filename

--key-file if file exists, read key/nonce; else write the new key/nonce
--encrypt use keys from file to encrypt
--decrypt use keys from file to dencrypt

###### darryl.west | 2023.01.19
