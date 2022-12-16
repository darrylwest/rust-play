## Using rage to encrypt/decrypt text

players:

RM = receiving machine
SM = sending machine

Use case;

* RM wants some data and has a secure way to authenticate with the SM
* SM has data to send securely by encrypting with the RM's public key

1. RM generates a private/public key with:

    `rage-keygen` -> pub/priv keys to stdout

2. RM requests data from SM and sends the new public key

3. SM receives the request, validates the RM identity, and encrypts data with the RM's public key

    `rage -r public-key <file> | base64` -> to stdout`

4. SM returns the encrypted/base64 encoded data to RM

5. RM decrypts the data with private key

    `echo -n <b64-data> | base64 -d | rage -d -i priv.key` -> data to stdout`

## Over the wire, machine to machine

1. RM creates a priv/pub key pair
2. RM rm sends request for data to SM and includes the pub key?
3. SM fetches the data, encrypts with the public key, returns to RM
4. RM recieves encrypted data and decrypts with private key 

### Notes

* the RM needs to store the private key in a temp location
* the RM needs to store the public key to a temp location
* identity? passphrase? 

## With ed25519 key pair

This almost works:

    `ssh-keygen -P '' -a 250 -t ed25519 -f ./ed25519`
    `rage -R ./ed25519.pub <file> > file.enc`
    `rage -d -i ./ed25519 <file.enc> > result`

But rage returns 'failed' on the second line.  It actually isn't a fail though.

## Better with symmetric keys

Use AES256 with 32 byte key and 32 byte iv to encrypt text.  Use ssh key-exchange
with secure websockets (websocat?) to exchange the symmetric key.

1. create the key/iv symm key and wrap in json or toml:

```
    {
        "key": key,
        "iv": iv,
        "version": create_date,
        "expires": expire_date
    }
```

