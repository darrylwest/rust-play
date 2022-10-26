# age encrypt/decrypt tests

## Summary

Based on [age encryption crate](https://docs.rs/age/0.8.1/age/) `age` provides strong encryption and is well suited 
for 'data-at-rest'.  It's slow which is good for data at rest encryption but not suited when you need to quickly
encrypt and decrypts.  for that, openssl is maybe/probably the best choice...

###### dpw | 2022-10-26
