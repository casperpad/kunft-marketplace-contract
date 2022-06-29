import { config } from "dotenv";
config();
import { Keys, CasperClient } from "casper-js-sdk";
import { CEP47Client } from "casper-cep47-js-client";
import { BigNumberish, parseFixed } from "@ethersproject/bignumber";
import { getAccountNamedKeyValue, getDeploy, getBinary } from "./utils";
import { MarketplaceClient } from "./clients/marketplace";

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

const deployMarketplace = async () => {
  const marketplace = new MarketplaceClient(NODE_ADDRESS!, CHAIN_NAME!);
  const contractName = "kunft_marketplace";
  const deploy = marketplace.install(
    getBinary(MARKETPLACE_CONTRACT!),
    { feeWallet: KEYS.publicKey, contractName },
    INSTALL_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );

  const installDeployHash = await deploy.send(NODE_ADDRESS!);

  console.log({ installDeployHash });

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... installed successfully.`);

  const casperClient = new CasperClient(NODE_ADDRESS!);

  const contractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${contractName}_contract_hash`
  );

  console.log({ contractHash });
};

deployMarketplace();
