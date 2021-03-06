use crypto::{keccak256, Crypto};
use ethereum_types::{Address, H160, H256, U256};
use rust_crypto::digest::Digest;
use rust_crypto::sha3::Sha3;
use secp256k1::{self, key};
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Fail)]
pub enum AccountError {
    #[fail(display = "failed to derive public key: {}", error)]
    DerivePublicKeyError { error: secp256k1::Error },
    #[fail(display = "failed to sign: {}", error)]
    SignError { error: secp256k1::Error },
    #[fail(display = "failed to build signature message: {}", error)]
    MessageError { error: secp256k1::Error },
    #[fail(display = "failed to borrow")]
    BorrowError,
}

pub struct Account {
    secp: Arc<secp256k1::Secp256k1>,
    pub address: Address,
    secret: key::SecretKey,
    public: key::PublicKey,
}

impl Account {
    /// Create a new address with the give rng implementation.
    pub fn new(crypto: &mut Crypto) -> Result<Account, AccountError> {
        let Crypto {
            ref secp,
            ref mut rng,
        } = *crypto;

        let (secret, public, address) = {
            let secret = key::SecretKey::new(secp, rng);
            let public = key::PublicKey::from_secret_key(secp, &secret)
                .map_err(|error| AccountError::DerivePublicKeyError { error })?;

            let address = {
                let serialized = public.serialize_vec(secp, false);
                // NB: important that we convert from H256 since `H256 -> H160` trims the leading bits.
                // i.e.: 00 00 00 af ff ff ff ff -> af ff ff ff ff
                let hash = H256::from(keccak256(&serialized[1..]));
                Address::from(H160::from(hash))
            };

            (secret, public, address)
        };

        Ok(Self {
            secp: Arc::clone(secp),
            address,
            secret,
            public,
        })
    }

    /// Create a new signer.
    pub fn sign<'a>(&'a self) -> Signer<'a> {
        Signer::new(self)
    }
}

impl fmt::Debug for Account {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Account")
            .field("address", &self.address)
            .field("secret", &self.secret)
            .field("public", &self.public)
            .finish()
    }
}

pub struct Signer<'a> {
    account: &'a Account,
    checksum: Sha3,
}

impl<'a> Signer<'a> {
    pub fn new(account: &'a Account) -> Self {
        Self {
            account,
            checksum: Sha3::keccak256(),
        }
    }

    /// Input the given set of bytes.
    pub fn input<D: Digestable>(mut self, digestable: D) -> Self {
        digestable.digest(&mut self.checksum);
        self
    }

    /// Finish the signature.
    pub fn finish(self) -> Result<Signature, AccountError> {
        let Signer {
            account,
            mut checksum,
        } = self;

        let mut hash = [0u8; 32];
        checksum.result(&mut hash);

        let hash = Self::to_rpc_hash(&hash);
        Self::to_secp_signature(account, &hash)
    }

    /// Convert the given message into an rpc hash, with the expected envelope.
    fn to_rpc_hash(message: &[u8]) -> Vec<u8> {
        let mut checksum = Sha3::keccak256();

        checksum.input(&format!("\x19Ethereum Signed Message:\n{}", message.len()).into_bytes());
        checksum.input(message);

        let mut hash = vec![0u8; 32];
        checksum.result(&mut hash);

        hash
    }

    /// Build a secp256k1 signature.
    fn to_secp_signature(account: &Account, message: &[u8]) -> Result<Signature, AccountError> {
        let message = secp256k1::Message::from_slice(message)
            .map_err(|error| AccountError::MessageError { error })?;

        let sig = account
            .secp
            .sign_recoverable(&message, &account.secret)
            .map_err(|error| AccountError::SignError { error })?;

        let (rec_id, data) = sig.serialize_compact(&account.secp);

        let mut output = Vec::with_capacity(65);
        output.extend(&data[..]);
        output.push(rec_id.to_i32() as u8);
        Ok(Signature(output))
    }
}

#[derive(Debug)]
pub struct Signature(Vec<u8>);

impl From<Signature> for Vec<u8> {
    fn from(sig: Signature) -> Vec<u8> {
        sig.0
    }
}

impl ::std::convert::AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// Trait for things which can be digested.
pub trait Digestable {
    /// Digest the given type.
    fn digest(self, checksum: &mut Sha3);
}

impl<'a> Digestable for &'a str {
    fn digest(self, checksum: &mut Sha3) {
        checksum.input(self.as_bytes());
    }
}

impl<'a> Digestable for &'a [u8] {
    fn digest(self, checksum: &mut Sha3) {
        checksum.input(self);
    }
}

impl<'a> Digestable for &'a Vec<u8> {
    fn digest(self, checksum: &mut Sha3) {
        checksum.input(self);
    }
}

impl Digestable for U256 {
    fn digest(self, checksum: &mut Sha3) {
        checksum.input(&<[u8; 32]>::from(self));
    }
}

impl Digestable for H160 {
    fn digest(self, checksum: &mut Sha3) {
        checksum.input(&<[u8; 20]>::from(self));
    }
}

impl<'a> Digestable for &'a Signature {
    fn digest(self, checksum: &mut Sha3) {
        checksum.input(self.as_ref());
    }
}
