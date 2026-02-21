const { Secp256k1, Sha256, EnglishMnemonic, Bip39, Slip10, Slip10Curve, stringToPath } = require("@cosmjs/crypto");
const { toHex } = require("@cosmjs/encoding");

async function main() {
  const mnemonic = process.env.MNEMONIC;
  if (!mnemonic) {
    console.error("MNEMONIC env var required");
    process.exit(1);
  }
  const message = process.argv[2];
  if (!message) {
    console.error("Message argument required");
    process.exit(1);
  }

  // Derive private key
  const mnemonicChecked = new EnglishMnemonic(mnemonic);
  const seed = await Bip39.mnemonicToSeed(mnemonicChecked);
  // Standard Cosmos HD path: m/44'/118'/0'/0/0
  const path = stringToPath("m/44'/118'/0'/0/0");
  const { privkey } = Slip10.derivePath(Slip10Curve.Secp256k1, seed, path);

  // Hash message (sha256)
  const messageBytes = Buffer.from(message, "utf8");
  const hash = new Sha256(messageBytes).digest();

  // Sign hash
  const signature = await Secp256k1.createSignature(hash, privkey);
  const signatureBytes = signature.toFixedLength();

  console.log(toHex(signatureBytes));
}

main().catch(console.error);
