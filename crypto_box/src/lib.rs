//! Pure Rust implementation of the [`crypto_box`] public-key authenticated
//! encryption scheme from [NaCl]-family libraries (e.g. libsodium, TweetNaCl)
//! which combines the [X25519] Diffie-Hellman function and the
//! [XSalsa20Poly1305] authenticated encryption cipher into an Elliptic Curve
//! Integrated Encryption Scheme ([ECIES]).
//!
//! # Introduction
//!
//! Imagine Alice wants something valuable shipped to her. Because it's
//! valuable, she wants to make sure it arrives securely (i.e. hasn't been
//! opened or tampered with) and that it's not a forgery (i.e. it's actually
//! from the sender she's expecting it to be from and nobody's pulling the old
//! switcheroo).
//!
//! One way she can do this is by providing the sender (let's call him Bob)
//! with a high-security box of her choosing. She provides Bob with this box,
//! and something else: a padlock, but a padlock without a key. Alice is
//! keeping that key all to herself. Bob can put items in the box then put the
//! padlock onto it, but once the padlock snaps shut, the box cannot be opened
//! by anyone who doesn't have Alice's private key.
//!
//! Here's the twist though, Bob also puts a padlock onto the box. This padlock
//! uses a key Bob has published to the world, such that if you have one of
//! Bob's keys, you know a box came from him because Bob's keys will open Bob's
//! padlocks (let's imagine a world where padlocks cannot be forged even if you
//! know the key). Bob then sends the box to Alice.
//!
//! In order for Alice to open the box, she needs two keys: her private key
//! that opens her own padlock, and Bob's well-known key. If Bob's key doesn't
//! open the second padlock then Alice knows that this is not the box she was
//! expecting from Bob, it's a forgery.
//!
//! # Usage
//!
//! ```rust
//! use crypto_box::{Box, PublicKey, SecretKey, aead::Aead};
//!
//! //
//! // Encryption
//! //
//!
//! // Generate a random secret key.
//! // NOTE: It can be serialized as bytes by calling `secret_key.to_bytes()`
//! let mut rng = rand::thread_rng();
//! let alice_secret_key = SecretKey::generate(&mut rng);
//!
//! // Get the public key for the secret key we just generated
//! let alice_public_key_bytes = alice_secret_key.public_key().as_bytes().clone();
//!
//! // Obtain your recipient's public key.
//! let bob_public_key = PublicKey::from([
//!    0xe8, 0x98, 0xc, 0x86, 0xe0, 0x32, 0xf1, 0xeb,
//!    0x29, 0x75, 0x5, 0x2e, 0x8d, 0x65, 0xbd, 0xdd,
//!    0x15, 0xc3, 0xb5, 0x96, 0x41, 0x17, 0x4e, 0xc9,
//!    0x67, 0x8a, 0x53, 0x78, 0x9d, 0x92, 0xc7, 0x54,
//! ]);
//!
//! // Create a `Box` by performing Diffie-Hellman key agreement between
//! // the two keys.
//! let alice_box = Box::new(&bob_public_key, &alice_secret_key);
//!
//! // Get a random nonce to encrypt the message under
//! let nonce = crypto_box::generate_nonce(&mut rng);
//!
//! // Message to encrypt
//! let plaintext = b"Top secret message we're encrypting";
//!
//! // Encrypt the message using the box
//! let ciphertext = alice_box.encrypt(&nonce, &plaintext[..]).unwrap();
//!
//! //
//! // Decryption
//! //
//!
//! // Either side can encrypt or decrypt messages under the Diffie-Hellman key
//! // they agree upon. The example below shows Bob's side.
//! let bob_secret_key = SecretKey::from([
//!     0xb5, 0x81, 0xfb, 0x5a, 0xe1, 0x82, 0xa1, 0x6f,
//!     0x60, 0x3f, 0x39, 0x27, 0xd, 0x4e, 0x3b, 0x95,
//!     0xbc, 0x0, 0x83, 0x10, 0xb7, 0x27, 0xa1, 0x1d,
//!     0xd4, 0xe7, 0x84, 0xa0, 0x4, 0x4d, 0x46, 0x1b
//! ]);
//!
//! // Deserialize Alice's public key from bytes
//! let alice_public_key = PublicKey::from(alice_public_key_bytes);
//!
//! // Bob can compute the same Box as Alice by performing the reciprocal
//! // key exchange operation.
//! let bob_box = Box::new(&alice_public_key, &bob_secret_key);
//!
//! // Decrypt the message, using the same randomly generated nonce
//! let decrypted_plaintext = bob_box.decrypt(&nonce, &ciphertext[..]).unwrap();
//!
//! assert_eq!(&plaintext[..], &decrypted_plaintext[..]);
//! ```
//!
//! ## In-place Usage (eliminates `alloc` requirement)
//!
//! This crate has an optional `alloc` feature which can be disabled in e.g.
//! microcontroller environments that don't have a heap.
//!
//! The [`Aead::encrypt_in_place`] and [`Aead::decrypt_in_place`]
//! methods accept any type that impls the [`aead::Buffer`] trait which
//! contains the plaintext for encryption or ciphertext for decryption.
//!
//! Note that if you enable the `heapless` feature of this crate,
//! you will receive an impl of `aead::Buffer` for [`heapless::Vec`]
//! (re-exported from the `aead` crate as `aead::heapless::Vec`),
//! which can then be passed as the `buffer` parameter to the in-place encrypt
//! and decrypt methods.
//!
//! A `heapless` usage example can be found in the documentation for the
//! `xsalsa20poly1305` crate:
//!
//! <https://docs.rs/xsalsa20poly1305/latest/xsalsa20poly1305/#in-place-usage-eliminates-alloc-requirement>
//!
//! [NaCl]: https://nacl.cr.yp.to/
//! [`crypto_box`]: https://nacl.cr.yp.to/box.html
//! [X25519]: https://cr.yp.to/ecdh.html
//! [XSalsa20Poly1305]: https://nacl.cr.yp.to/secretbox.html
//! [ECIES]: https://en.wikipedia.org/wiki/Integrated_Encryption_Scheme
//! [`Aead::encrypt_in_place`]: https://docs.rs/aead/latest/aead/trait.Aead.html#method.encrypt_in_place
//! [`Aead::decrypt_in_place`]: https://docs.rs/aead/latest/aead/trait.Aead.html#method.decrypt_in_place
//! [`aead::Buffer`]: https://docs.rs/aead/latest/aead/trait.Buffer.html
//! [`heapless::Vec`]: https://docs.rs/heapless/latest/heapless/struct.Vec.html

#![no_std]
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png")]
#![warn(missing_docs, rust_2018_idioms)]

pub use x25519_dalek::PublicKey;
pub use xsalsa20poly1305::aead;

use aead::generic_array::{
    typenum::{U0, U16, U24},
    GenericArray,
};
use aead::{Aead, Buffer, Error, NewAead};
use core::fmt::{self, Debug};
use rand_core::{CryptoRng, RngCore};
use salsa20::hsalsa20;
use xsalsa20poly1305::{Tag, XSalsa20Poly1305};

/// Size of a `crypto_box` public or secret key in bytes.
pub const KEY_SIZE: usize = 32;

/// Generate a random nonce: every message MUST have a unique nonce!
///
/// Do *NOT* ever reuse the same nonce for two messages!
pub fn generate_nonce<T>(csprng: &mut T) -> GenericArray<u8, U24>
where
    T: RngCore + CryptoRng,
{
    let mut nonce = GenericArray::default();
    csprng.fill_bytes(&mut nonce);
    nonce
}

/// `crypto_box` secret key
#[derive(Clone)]
pub struct SecretKey(x25519_dalek::StaticSecret);

impl SecretKey {
    /// Generate a random [`SecretKey`].
    pub fn generate<T>(csprng: &mut T) -> Self
    where
        T: RngCore + CryptoRng,
    {
        SecretKey(x25519_dalek::StaticSecret::new(csprng))
    }

    /// Get the [`PublicKey`] which corresponds to this [`SecretKey`]
    pub fn public_key(&self) -> PublicKey {
        self.into()
    }

    /// Get the serialized bytes for this [`SecretKey`]
    pub fn to_bytes(&self) -> [u8; KEY_SIZE] {
        self.0.to_bytes()
    }
}

impl From<[u8; KEY_SIZE]> for SecretKey {
    fn from(bytes: [u8; KEY_SIZE]) -> SecretKey {
        SecretKey(bytes.into())
    }
}

impl Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretKey(...)")
    }
}

impl From<&SecretKey> for PublicKey {
    fn from(secret_key: &SecretKey) -> PublicKey {
        PublicKey::from(&secret_key.0)
    }
}

/// Alias for [`SalsaBox`].
pub type Box = SalsaBox;

/// Public-key encryption scheme based on the [X25519] Elliptic Curve
/// Diffie-Hellman function and the [XSalsa20Poly1305] authenticated encryption
/// cipher.
///
/// This type impls the [`aead::Aead`] trait, and otherwise functions as a
/// symmetric Authenticated Encryption with Associated Data (AEAD) cipher
/// once instantiated.
///
/// [X25519]: https://cr.yp.to/ecdh.html
/// [XSalsa20Poly1305]: https://github.com/RustCrypto/AEADs/tree/master/xsalsa20poly1305
pub struct SalsaBox(XSalsa20Poly1305);

impl SalsaBox {
    /// Create a new [`SalsaBox`], performing X25519 Diffie-Hellman to derive
    /// a shared secret from the provided public and secret keys.
    pub fn new(public_key: &PublicKey, secret_key: &SecretKey) -> Self {
        let shared_secret = secret_key.0.diffie_hellman(public_key);

        // Use HSalsa20 to create a uniformly random key from the shared secret
        let key = hsalsa20(
            &GenericArray::clone_from_slice(shared_secret.as_bytes()),
            &GenericArray::default(),
        );

        SalsaBox(XSalsa20Poly1305::new(key))
    }
}

impl Aead for SalsaBox {
    type NonceSize = U24;
    type TagSize = U16;
    type CiphertextOverhead = U0;

    fn encrypt_in_place(
        &self,
        nonce: &GenericArray<u8, Self::NonceSize>,
        associated_data: &[u8],
        buffer: &mut impl Buffer,
    ) -> Result<(), Error> {
        self.0.encrypt_in_place(nonce, associated_data, buffer)
    }

    fn encrypt_in_place_detached(
        &self,
        nonce: &GenericArray<u8, Self::NonceSize>,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> Result<Tag, Error> {
        self.0
            .encrypt_in_place_detached(nonce, associated_data, buffer)
    }

    fn decrypt_in_place(
        &self,
        nonce: &GenericArray<u8, Self::NonceSize>,
        associated_data: &[u8],
        buffer: &mut impl Buffer,
    ) -> Result<(), Error> {
        self.0.decrypt_in_place(nonce, associated_data, buffer)
    }

    fn decrypt_in_place_detached(
        &self,
        nonce: &GenericArray<u8, Self::NonceSize>,
        associated_data: &[u8],
        buffer: &mut [u8],
        tag: &Tag,
    ) -> Result<(), Error> {
        self.0
            .decrypt_in_place_detached(nonce, associated_data, buffer, tag)
    }
}