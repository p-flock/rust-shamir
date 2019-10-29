## A rust implementation of [Shamir secret sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing)

This library demonstrates a k/n shamir secret sharing scheme. It supports: Creating shares from a secret, recovering a secret from a set of shares, addition and subtraction between secret-shares.

At the moment the library uses the 16-bit prime 65413 as the field, this avoids any possible overflow from single operations on signed 64-bit integers (i64 in rust);
