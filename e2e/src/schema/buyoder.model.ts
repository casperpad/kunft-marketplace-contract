import { Schema, model, connect } from "mongoose";

interface IBuyOrder {
  creator: string;
  collection: string;
  tokenId: string;
  owner: string;
  payToken?: string;
  price: string;
  startTime: number;
  additionalRecipient?: string;
}

const buyOrderSchema = new Schema<IBuyOrder>({
  creator: { type: String, required: true },
  collection: { type: String, required: true },
  tokenId: { type: String, required: true },
  owner: { type: String, required: true },
  payToken: { type: String },
  price: { type: String, required: true },
  startTime: { type: Number, required: true },
  additionalRecipient: { type: String },
});

const BuyOrder = model<IBuyOrder>("BuyOrder", buyOrderSchema);

export default BuyOrder;
