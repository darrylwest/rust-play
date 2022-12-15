# Using rage to encrypt/decrypt text with ed25519

players:

RM = receiving machine
SM = sending machine

Use case;

* RM wants some data and has a secure way to authenticate with the SM
* SM has data to send securely by encrypting with the RM's public key

1. RM generates a private/public key with:

    `ssh-keygen -a 250 -t ed25519 -f ./ed25519`
    `rage-keygen` -> pub/priv keys to stdout

2. RM requests data from SM and sends the new public key

3. SM receives the request, validates the RM identity, and encrypts data with the RM's public key

    `rage -R ./ed25519.pub <file> > file.enc
    `rage -r public-key | base64` -> to stdout

4. SM returns the encrypted/base64 encoded data to RM

5. RM decrypts the data with private key

    `rage -d -i ./ed25519 <file.enc> > result
    `rage -d -i priv.key` -> data to stdout

## Over the wire, machine to machine

1. RM creates a ed25519 priv/pub key pair
2. RM rm sends request for data to SM and includes the pub key (maybe in header?)
3. SM fetches the data, encrypts with the public key, sends to RM
4. RM recieves encrypted data and decrypts with private key 

### Notes

* the RM needs to store the private key in a temp location
* the RM needs to store the public key to a temp location
* identity? passphrase? 
