import { config } from "dotenv";
config();
import { connect } from "mongoose";
import {
  CasperClient,
  CLByteArray,
  CLMap,
  CLOption,
  CLString,
  CLStringType,
  CLU256,
  CLValue,
  CLValueBuilder,
  EventName,
  EventStream,
  Keys,
} from "casper-js-sdk";
import {
  MarketplaceEventParser,
  MarketplaceEvents,
} from "./clients/marketplace";
import {
  CEP47Client,
  CEP47Events,
  CEP47EventParser,
} from "casper-cep47-js-client";
import { getAccountNamedKeyValue } from "./utils";
import BuyOrder from "./schema/buyoder.model";
import SellOrder from "./schema/sellOrder.model";

const {
  EVENT_STREAM_ADDRESS,
  MONGODB_URL,
  NODE_ADDRESS,
  MARKETPLACE_CONTRACT_NAME,
  MASTER_KEY_PAIR_PATH,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const startEventStream = async () => {
  const es = new EventStream(EVENT_STREAM_ADDRESS!);
  const casperClient = new CasperClient(NODE_ADDRESS!);
  const contractPackageHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${MARKETPLACE_CONTRACT_NAME!}_contract_package_hash`
  );

  es.subscribe(EventName.DeployProcessed, async (event) => {
    const parsedEvents = MarketplaceEventParser(
      {
        contractPackageHash: contractPackageHash.slice(5),
        eventNames: [MarketplaceEvents.SellOrderCreated],
      },
      event
    );
    if (parsedEvents && parsedEvents.success) {
      console.log("***  MARKETPLACE EVENT  ***");

      const promises = parsedEvents.data.map(async (event: any) => {
        const eventName = event.name as MarketplaceEvents;
        const eventParams: CLMap<CLString, CLString> = event.clValue;
        console.info(`Handling ${eventName} event`);
        const creator = eventParams.get(CLValueBuilder.string("creator"));
        const collection = eventParams.get(CLValueBuilder.string("collection"));
        const tokenId = eventParams.get(CLValueBuilder.string("token_id"));
        const payToken = eventParams.get(CLValueBuilder.string("pay_token"));
        const price = eventParams.get(CLValueBuilder.string("price"));
        const startTime = eventParams.get(CLValueBuilder.string("start_time"));
        const buyer = eventParams.get(CLValueBuilder.string("buyer"));
        const additionalRecipient = eventParams.get(
          CLValueBuilder.string("additional_recipient")
        );
        switch (eventName) {
          case MarketplaceEvents.SellOrderCreated:
            const order = await SellOrder.findOne({
              creator: creator!.value(),
              contractHash: collection!.value(),
              tokenId: tokenId!.value(),
            });
            if (order) break;
            const sellOrder = new SellOrder({
              creator: creator!.value(),
              contractHash: collection!.value(),
              tokenId: tokenId!.value(),
              payToken:
                payToken!.value() === "None" ? undefined : payToken!.value(),
              price: price!.value(),
              startTime: startTime!.value(),
              status: "pending",
            });
            await sellOrder.save();
            break;
          case MarketplaceEvents.SellOrderCanceled:
            await SellOrder.findOneAndUpdate(
              {
                creator: creator!.value(),
                contractHash: collection!.value(),
                tokenId: tokenId!.value(),
              },
              { status: "canceled" }
            );
            break;
          case MarketplaceEvents.SellOrderBought:
            await SellOrder.findOneAndUpdate(
              {
                creator: creator!.value(),
                contractHash: collection!.value(),
                tokenId: tokenId!.value(),
              },
              {
                buyer: buyer!.value(),
                additionalRecipient: additionalRecipient!.value(),
                status: "succed",
              }
            );
            break;
          default:
            console.error(`Unhandled event: ${eventName}`);
        }
      });
      await Promise.all(promises);
      console.log("***     ***");
    }
  });
  es.start(0);
};

const storeEvent = async () => {
  try {
    await connect(MONGODB_URL!);
    console.log(`Connected to ${MONGODB_URL}`);
    startEventStream();
  } catch (err: any) {
    console.error(err);
  }
};
storeEvent();
