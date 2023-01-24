---
title: ChaCha20Ploy1305
author: dpw
paginate: true
theme: gaia
header: 'ChaCha Vers 0.5.0'
footer: 'darryl.west | 2023.01.24'
---

# ChaCha CLI

Separate encode / decode switches enables chacha saving keys/nonce and encrypted file or restoring an encrypted file.

---

# Encrypt Example

To encrypt a plain text file while creating a new key.  

`chacha --encrypt --plain plain.txt  --cipher cipher.enc --key-file keys.json`

Saves the new key and encrypted files.

---

# Encrypt Example

To decript an encrypted file with the supplied keys. 

`chacha --decrypt --key-file keys.json --cipher cipher.enc --plain plain.txt`

Output is written to specified file.

---

# Notes

+ the **plain-text** file could actually be a binary or tar ball
+ key-file: if file exists, read key/nonce; else write the new key/nonce
+ encrypt use keys from file to encrypt
+ decrypt use keys from file to dencrypt


