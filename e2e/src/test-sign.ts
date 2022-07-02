import { config } from "dotenv";
config();
import {
  Keys,
  CasperClient,
  CLValueBuilder,
  encodeBase16,
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,
  INSTALL_PAYMENT_AMOUNT,
  MARKETPLACE_CONTRACT,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const message = new TextEncoder().encode(
  `Signup with KUNFT with ${KEYS.publicKey.toHex()}`
);

const signedMessage = KEYS.sign(message);

console.log(encodeBase16(signedMessage));
