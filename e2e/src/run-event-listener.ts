import { config } from "dotenv";
config();
import { connect } from "mongoose";
import { EventName, EventStream } from "casper-js-sdk";
import {
  MarketplaceEventParser,
  MarketplaceEvents,
} from "./clients/marketplace";
import {
  CEP47Client,
  CEP47Events,
  CEP47EventParser,
} from "casper-cep47-js-client";

const { EVENT_STREAM_ADDRESS, MONGODB_URL } = process.env;

const startEventStream = () => {
  const es = new EventStream(EVENT_STREAM_ADDRESS!);
  const contractPackageHash =
    "5ede076610dedae5ec3aa581efcc9548c8a141350ce5b9713d87ed5d9bc56954";
  es.subscribe(EventName.DeployProcessed, async (event) => {
    const parsedEvents = MarketplaceEventParser(
      {
        contractPackageHash,
        eventNames: [MarketplaceEvents.SellOrderCreated],
      },
      event
    );
    if (parsedEvents && parsedEvents.success) {
      console.log("***  MARKETPLACE EVENT  ***");
      console.dir(parsedEvents.data, { depth: null });
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
