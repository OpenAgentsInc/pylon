import { Schema } from "effect"
import * as bip39 from "@scure/bip39"
import { wordlist } from "@scure/bip39/wordlists/english"
import { HDKey } from "@scure/bip32"
import { bech32 } from "bech32"
import * as secp256k1 from "@noble/secp256k1"
import { PubKey } from "./NostrEvent.js"

// Nostr derivation path according to SLIP-44: m/44'/1237'/account'/0/0
const NOSTR_DERIVATION_PATH_BASE = "m/44'/1237'"

// Branded types for NIP-06
export const Mnemonic = Schema.String.pipe(
  Schema.pattern(/^(\w+\s){11}\w+$|^(\w+\s){14}\w+$|^(\w+\s){17}\w+$|^(\w+\s){20}\w+$|^(\w+\s){23}\w+$/), // 12, 15, 18, 21, or 24 words
  Schema.brand("Mnemonic")
)
export type Mnemonic = typeof Mnemonic.Type

export const PrivateKeyHex = Schema.String.pipe(
  Schema.pattern(/^[a-f0-9]{64}$/),
  Schema.brand("PrivateKeyHex")
)
export type PrivateKeyHex = typeof PrivateKeyHex.Type

export const Nsec = Schema.String.pipe(
  Schema.pattern(/^nsec1[a-z0-9]+$/),
  Schema.brand("Nsec")
)
export type Nsec = typeof Nsec.Type

export const Npub = Schema.String.pipe(
  Schema.pattern(/^npub1[a-z0-9]+$/),
  Schema.brand("Npub")
)
export type Npub = typeof Npub.Type

export const Account = Schema.Number.pipe(
  Schema.int(),
  Schema.greaterThanOrEqualTo(0),
  Schema.brand("Account")
)
export type Account = typeof Account.Type

// NIP-06 Key Derivation class
export class Nip06KeyDerivation {
  /**
   * Generate a new mnemonic phrase using BIP39
   */
  static generateMnemonic(strength: 128 | 160 | 192 | 224 | 256 = 128): Mnemonic {
    const mnemonic = bip39.generateMnemonic(wordlist, strength)
    return Mnemonic.make(mnemonic)
  }

  /**
   * Validate a mnemonic phrase
   */
  static validateMnemonic(mnemonic: string): boolean {
    return bip39.validateMnemonic(mnemonic, wordlist)
  }

  /**
   * Derive a private key from mnemonic using NIP-06 specification
   */
  static derivePrivateKey(mnemonic: Mnemonic, account: Account = Account.make(0)): PrivateKeyHex {
    // Validate mnemonic
    if (!this.validateMnemonic(mnemonic)) {
      throw new Error("Invalid mnemonic")
    }

    // Convert mnemonic to seed
    const seed = bip39.mnemonicToSeedSync(mnemonic)
    
    // Create master key from seed
    const masterKey = HDKey.fromMasterSeed(seed)
    
    // Derive key using Nostr path: m/44'/1237'/account'/0/0
    const derivationPath = `${NOSTR_DERIVATION_PATH_BASE}/${account}'/0/0`
    const derivedKey = masterKey.derive(derivationPath)
    
    if (!derivedKey.privateKey) {
      throw new Error("Failed to derive private key")
    }
    
    // Convert to hex string
    const privateKeyHex = Buffer.from(derivedKey.privateKey).toString('hex')
    return PrivateKeyHex.make(privateKeyHex)
  }

  /**
   * Derive public key from private key
   */
  static derivePublicKey(privateKeyHex: PrivateKeyHex): PubKey {
    const privateKeyBytes = Buffer.from(privateKeyHex, 'hex')
    const publicKeyBytes = secp256k1.getPublicKey(privateKeyBytes, false) // uncompressed
    
    // For Nostr, we only want the x-coordinate (first 32 bytes after the 0x04 prefix)
    // Uncompressed key format: 0x04 + 32 bytes x + 32 bytes y = 65 bytes total
    const publicKeyHex = Buffer.from(publicKeyBytes.slice(1, 33)).toString('hex')
    return PubKey.make(publicKeyHex)
  }

  /**
   * Encode private key as nsec (bech32)
   */
  static encodeNsec(privateKeyHex: PrivateKeyHex): Nsec {
    const privateKeyBytes = Buffer.from(privateKeyHex, 'hex')
    const words = bech32.toWords(privateKeyBytes)
    const encoded = bech32.encode('nsec', words)
    return Nsec.make(encoded)
  }

  /**
   * Decode nsec to private key hex
   */
  static decodeNsec(nsec: Nsec): PrivateKeyHex {
    const decoded = bech32.decode(nsec)
    if (decoded.prefix !== 'nsec') {
      throw new Error('Invalid nsec prefix')
    }
    const privateKeyBytes = Buffer.from(bech32.fromWords(decoded.words))
    const privateKeyHex = privateKeyBytes.toString('hex')
    return PrivateKeyHex.make(privateKeyHex)
  }

  /**
   * Encode public key as npub (bech32)
   */
  static encodeNpub(publicKeyHex: PubKey): Npub {
    const publicKeyBytes = Buffer.from(publicKeyHex, 'hex')
    const words = bech32.toWords(publicKeyBytes)
    const encoded = bech32.encode('npub', words)
    return Npub.make(encoded)
  }

  /**
   * Decode npub to public key hex
   */
  static decodeNpub(npub: Npub): PubKey {
    const decoded = bech32.decode(npub)
    if (decoded.prefix !== 'npub') {
      throw new Error('Invalid npub prefix')
    }
    const publicKeyBytes = Buffer.from(bech32.fromWords(decoded.words))
    const publicKeyHex = publicKeyBytes.toString('hex')
    return PubKey.make(publicKeyHex)
  }

  /**
   * Complete key derivation from mnemonic to all formats
   */
  static deriveAllKeys(mnemonic: Mnemonic, account: Account = Account.make(0)) {
    const privateKeyHex = this.derivePrivateKey(mnemonic, account)
    const publicKeyHex = this.derivePublicKey(privateKeyHex)
    const nsec = this.encodeNsec(privateKeyHex)
    const npub = this.encodeNpub(publicKeyHex)

    return {
      mnemonic,
      account,
      privateKeyHex,
      publicKeyHex,
      nsec,
      npub,
      derivationPath: `${NOSTR_DERIVATION_PATH_BASE}/${account}'/0/0`
    }
  }
}

// Schema for the complete key derivation result
export const KeyDerivationResult = Schema.Struct({
  mnemonic: Mnemonic,
  account: Account,
  privateKeyHex: PrivateKeyHex,
  publicKeyHex: PubKey,
  nsec: Nsec,
  npub: Npub,
  derivationPath: Schema.String
})
export type KeyDerivationResult = typeof KeyDerivationResult.Type

// Error types for NIP-06 operations
export class InvalidMnemonicError extends Schema.TaggedError<InvalidMnemonicError>()("InvalidMnemonicError", {
  mnemonic: Schema.String,
  reason: Schema.String
}) {}

export class KeyDerivationError extends Schema.TaggedError<KeyDerivationError>()("KeyDerivationError", {
  reason: Schema.String,
  derivationPath: Schema.optional(Schema.String)
}) {}

export class EncodingError extends Schema.TaggedError<EncodingError>()("EncodingError", {
  format: Schema.String,
  reason: Schema.String
}) {}