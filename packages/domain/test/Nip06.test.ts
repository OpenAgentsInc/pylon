import { describe, test, expect } from "vitest"
import { 
  Nip06KeyDerivation,
  Mnemonic,
  PrivateKeyHex,
  Nsec,
  Npub,
  Account
} from "../src/Nip06.js"
import { PubKey } from "../src/NostrEvent.js"

describe("NIP-06 Key Derivation", () => {
  describe("Test Vectors from NIP-06 Specification", () => {
    test("Test Vector 1: leader monkey parrot...", () => {
      const mnemonic = Mnemonic.make("leader monkey parrot ring guide accident before fence cannon height naive bean")
      const expectedPrivateKey = "7f7ff03d123792d6ac594bfa67bf6d0c0ab55b6b1fdb6249303fe861f1ccba9a"
      const expectedNsec = "nsec10allq0gjx7fddtzef0ax00mdps9t2kmtrldkyjfs8l5xruwvh2dq0lhhkp"
      const expectedPublicKey = "17162c921dc4d2518f9a101db33695df1afb56ab82f5ff3e5da6eec3ca5cd917"
      const expectedNpub = "npub1zutzeysacnf9rru6zqwmxd54mud0k44tst6l70ja5mhv8jjumytsd2x7nu"

      const result = Nip06KeyDerivation.deriveAllKeys(mnemonic, Account.make(0))

      expect(result.privateKeyHex).toBe(expectedPrivateKey)
      expect(result.nsec).toBe(expectedNsec)
      expect(result.publicKeyHex).toBe(expectedPublicKey)
      expect(result.npub).toBe(expectedNpub)
      expect(result.account).toBe(0)
      expect(result.derivationPath).toBe("m/44'/1237'/0'/0/0")
    })

    test("Test Vector 2: what bleak badge...", () => {
      const mnemonic = Mnemonic.make("what bleak badge arrange retreat wolf trade produce cricket blur garlic valid proud rude strong choose busy staff weather area salt hollow arm fade")
      const expectedPrivateKey = "c15d739894c81a2fcfd3a2df85a0d2c0dbc47a280d092799f144d73d7ae78add"
      const expectedNsec = "nsec1c9wh8xy5eqdzln7n5t0ctgxjcrdug73gp5yj0x03gntn67h83twssdfhel"
      const expectedPublicKey = "d41b22899549e1f3d335a31002cfd382174006e166d3e658e3a5eecdb6463573"
      const expectedNpub = "npub16sdj9zv4f8sl85e45vgq9n7nsgt5qphpvmf7vk8r5hhvmdjxx4es8rq74h"

      const result = Nip06KeyDerivation.deriveAllKeys(mnemonic, Account.make(0))

      expect(result.privateKeyHex).toBe(expectedPrivateKey)
      expect(result.nsec).toBe(expectedNsec)
      expect(result.publicKeyHex).toBe(expectedPublicKey)
      expect(result.npub).toBe(expectedNpub)
      expect(result.account).toBe(0)
      expect(result.derivationPath).toBe("m/44'/1237'/0'/0/0")
    })
  })

  describe("Mnemonic Generation and Validation", () => {
    test("should generate valid 12-word mnemonic", () => {
      const mnemonic = Nip06KeyDerivation.generateMnemonic(128)
      expect(mnemonic.split(' ')).toHaveLength(12)
      expect(Nip06KeyDerivation.validateMnemonic(mnemonic)).toBe(true)
    })

    test("should generate valid 15-word mnemonic", () => {
      const mnemonic = Nip06KeyDerivation.generateMnemonic(160)
      expect(mnemonic.split(' ')).toHaveLength(15)
      expect(Nip06KeyDerivation.validateMnemonic(mnemonic)).toBe(true)
    })

    test("should generate valid 18-word mnemonic", () => {
      const mnemonic = Nip06KeyDerivation.generateMnemonic(192)
      expect(mnemonic.split(' ')).toHaveLength(18)
      expect(Nip06KeyDerivation.validateMnemonic(mnemonic)).toBe(true)
    })

    test("should generate valid 21-word mnemonic", () => {
      const mnemonic = Nip06KeyDerivation.generateMnemonic(224)
      expect(mnemonic.split(' ')).toHaveLength(21)
      expect(Nip06KeyDerivation.validateMnemonic(mnemonic)).toBe(true)
    })

    test("should generate valid 24-word mnemonic", () => {
      const mnemonic = Nip06KeyDerivation.generateMnemonic(256)
      expect(mnemonic.split(' ')).toHaveLength(24)
      expect(Nip06KeyDerivation.validateMnemonic(mnemonic)).toBe(true)
    })

    test("should validate correct mnemonic", () => {
      const validMnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
      expect(Nip06KeyDerivation.validateMnemonic(validMnemonic)).toBe(true)
    })

    test("should reject invalid mnemonic", () => {
      const invalidMnemonic = "invalid mnemonic phrase with wrong words that do not exist"
      expect(Nip06KeyDerivation.validateMnemonic(invalidMnemonic)).toBe(false)
    })
  })

  describe("Key Derivation", () => {
    test("should derive same keys from same mnemonic", () => {
      const mnemonic = Mnemonic.make("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")
      
      const result1 = Nip06KeyDerivation.deriveAllKeys(mnemonic, Account.make(0))
      const result2 = Nip06KeyDerivation.deriveAllKeys(mnemonic, Account.make(0))
      
      expect(result1.privateKeyHex).toBe(result2.privateKeyHex)
      expect(result1.publicKeyHex).toBe(result2.publicKeyHex)
      expect(result1.nsec).toBe(result2.nsec)
      expect(result1.npub).toBe(result2.npub)
    })

    test("should derive different keys for different accounts", () => {
      const mnemonic = Mnemonic.make("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")
      
      const result0 = Nip06KeyDerivation.deriveAllKeys(mnemonic, Account.make(0))
      const result1 = Nip06KeyDerivation.deriveAllKeys(mnemonic, Account.make(1))
      
      expect(result0.privateKeyHex).not.toBe(result1.privateKeyHex)
      expect(result0.publicKeyHex).not.toBe(result1.publicKeyHex)
      expect(result0.nsec).not.toBe(result1.nsec)
      expect(result0.npub).not.toBe(result1.npub)
      expect(result0.derivationPath).toBe("m/44'/1237'/0'/0/0")
      expect(result1.derivationPath).toBe("m/44'/1237'/1'/0/0")
    })

    test("should derive valid private keys", () => {
      const mnemonic = Nip06KeyDerivation.generateMnemonic()
      const privateKey = Nip06KeyDerivation.derivePrivateKey(mnemonic, Account.make(0))
      
      expect(privateKey).toMatch(/^[a-f0-9]{64}$/)
      expect(privateKey.length).toBe(64)
    })

    test("should derive valid public keys from private keys", () => {
      const mnemonic = Nip06KeyDerivation.generateMnemonic()
      const privateKey = Nip06KeyDerivation.derivePrivateKey(mnemonic, Account.make(0))
      const publicKey = Nip06KeyDerivation.derivePublicKey(privateKey)
      
      expect(publicKey).toMatch(/^[a-f0-9]{64}$/)
      expect(publicKey.length).toBe(64)
    })

    test("should throw error for invalid mnemonic in derivation", () => {
      const invalidMnemonic = "invalid mnemonic phrase" as any as Mnemonic
      
      expect(() => {
        Nip06KeyDerivation.derivePrivateKey(invalidMnemonic, Account.make(0))
      }).toThrow("Invalid mnemonic")
    })
  })

  describe("Bech32 Encoding/Decoding", () => {
    test("should encode and decode nsec correctly", () => {
      const privateKeyHex = PrivateKeyHex.make("7f7ff03d123792d6ac594bfa67bf6d0c0ab55b6b1fdb6249303fe861f1ccba9a")
      const nsec = Nip06KeyDerivation.encodeNsec(privateKeyHex)
      const decodedPrivateKey = Nip06KeyDerivation.decodeNsec(nsec)
      
      expect(nsec).toMatch(/^nsec1[a-z0-9]+$/)
      expect(decodedPrivateKey).toBe(privateKeyHex)
    })

    test("should encode and decode npub correctly", () => {
      const publicKeyHex = PubKey.make("17162c921dc4d2518f9a101db33695df1afb56ab82f5ff3e5da6eec3ca5cd917")
      const npub = Nip06KeyDerivation.encodeNpub(publicKeyHex)
      const decodedPublicKey = Nip06KeyDerivation.decodeNpub(npub)
      
      expect(npub).toMatch(/^npub1[a-z0-9]+$/)
      expect(decodedPublicKey).toBe(publicKeyHex)
    })

    test("should validate nsec format", () => {
      const validNsec = Nsec.make("nsec10allq0gjx7fddtzef0ax00mdps9t2kmtrldkyjfs8l5xruwvh2dq0lhhkp")
      expect(validNsec).toBeDefined()
    })

    test("should validate npub format", () => {
      const validNpub = Npub.make("npub1zutzeysacnf9rru6zqwmxd54mud0k44tst6l70ja5mhv8jjumytsd2x7nu")
      expect(validNpub).toBeDefined()
    })

    test("should reject invalid nsec format", () => {
      expect(() => Nsec.make("invalid")).toThrow()
      expect(() => Nsec.make("npub1zutzeysacnf9rru6zqwmxd54mud0k44tst6l70ja5mhv8jjumytsd2x7nu")).toThrow()
    })

    test("should reject invalid npub format", () => {
      expect(() => Npub.make("invalid")).toThrow()
      expect(() => Npub.make("nsec10allq0gjx7fddtzef0ax00mdps9t2kmtrldkyjfs8l5xruwvh2dq0lhhkp")).toThrow()
    })

    test("should throw error for invalid nsec prefix in decode", () => {
      const invalidNsec = "npub1zutzeysacnf9rru6zqwmxd54mud0k44tst6l70ja5mhv8jjumytsd2x7nu" as any as Nsec
      
      expect(() => {
        Nip06KeyDerivation.decodeNsec(invalidNsec)
      }).toThrow("Invalid nsec prefix")
    })

    test("should throw error for invalid npub prefix in decode", () => {
      const invalidNpub = "nsec10allq0gjx7fddtzef0ax00mdps9t2kmtrldkyjfs8l5xruwvh2dq0lhhkp" as any as Npub
      
      expect(() => {
        Nip06KeyDerivation.decodeNpub(invalidNpub)
      }).toThrow("Invalid npub prefix")
    })
  })

  describe("Account Handling", () => {
    test("should support account numbers 0-999", () => {
      const mnemonic = Mnemonic.make("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")
      
      const accounts = [0, 1, 10, 100, 999]
      const results = accounts.map(acc => 
        Nip06KeyDerivation.deriveAllKeys(mnemonic, Account.make(acc))
      )
      
      // All accounts should produce different keys
      const privateKeys = results.map(r => r.privateKeyHex)
      const uniquePrivateKeys = new Set(privateKeys)
      expect(uniquePrivateKeys.size).toBe(accounts.length)
      
      // Check derivation paths
      expect(results[0].derivationPath).toBe("m/44'/1237'/0'/0/0")
      expect(results[1].derivationPath).toBe("m/44'/1237'/1'/0/0")
      expect(results[2].derivationPath).toBe("m/44'/1237'/10'/0/0")
      expect(results[3].derivationPath).toBe("m/44'/1237'/100'/0/0")
      expect(results[4].derivationPath).toBe("m/44'/1237'/999'/0/0")
    })

    test("should reject negative account numbers", () => {
      expect(() => Account.make(-1)).toThrow()
    })
  })

  describe("Edge Cases", () => {
    test("should handle empty string gracefully", () => {
      expect(Nip06KeyDerivation.validateMnemonic("")).toBe(false)
    })

    test("should handle single word gracefully", () => {
      expect(Nip06KeyDerivation.validateMnemonic("abandon")).toBe(false)
    })

    test("should handle too many words gracefully", () => {
      const tooManyWords = Array(25).fill("abandon").join(" ")
      expect(Nip06KeyDerivation.validateMnemonic(tooManyWords)).toBe(false)
    })

    test("should handle words with extra spaces", () => {
      const spacedMnemonic = "abandon  abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
      expect(Nip06KeyDerivation.validateMnemonic(spacedMnemonic)).toBe(false)
    })
  })

  describe("Branded Types", () => {
    test("should accept valid mnemonic patterns", () => {
      const valid12 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
      const valid24 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art"
      
      expect(() => Mnemonic.make(valid12)).not.toThrow()
      expect(() => Mnemonic.make(valid24)).not.toThrow()
    })

    test("should reject invalid mnemonic patterns", () => {
      expect(() => Mnemonic.make("")).toThrow()
      expect(() => Mnemonic.make("one two")).toThrow()
      expect(() => Mnemonic.make("too many words " + Array(20).fill("word").join(" "))).toThrow()
    })

    test("should accept valid private key hex", () => {
      const validHex = "7f7ff03d123792d6ac594bfa67bf6d0c0ab55b6b1fdb6249303fe861f1ccba9a"
      expect(() => PrivateKeyHex.make(validHex)).not.toThrow()
    })

    test("should reject invalid private key hex", () => {
      expect(() => PrivateKeyHex.make("invalid")).toThrow()
      expect(() => PrivateKeyHex.make("7f7ff03d123792d6ac594bfa67bf6d0c0ab55b6b1fdb6249303fe861f1ccba9")).toThrow() // too short
      expect(() => PrivateKeyHex.make("7f7ff03d123792d6ac594bfa67bf6d0c0ab55b6b1fdb6249303fe861f1ccba9aa")).toThrow() // too long
      expect(() => PrivateKeyHex.make("GFFFFFF03d123792d6ac594bfa67bf6d0c0ab55b6b1fdb6249303fe861f1ccba9a")).toThrow() // invalid hex
    })
  })
})