# Using rage to encrypt/decrypt text with ed25519


1. generate the private/public keys with:

    `ssh-keygen -a 250 -t ed25519 -f ./ed25519`

2. encrypt the file with the public key

    `rage -R ./ed25519.pub <file> > file.enc

3. decrypt the file with private key

    `rage -d -i ./ed25519 <file.enc> > result


## Over the wire, machine to machine

players:

* reciving machine RM wants some data
* sending machine SM has data to send securely

1. RM creates a ed25519 priv/pub key pair
2. RM rm sends request for data to SM and includes the pub key (maybe in header?)
3. SM fetches the data, encrypts with the public key, sends to RM
4. RM recieves encrypted data and decrypts with private key 

### Notes

* the RM needs to store the private key in a temp location
* the RM needs to store the public key to a temp location
* 
