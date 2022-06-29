import { Schema, model, connect } from "mongoose";

interface ISellOrder {
  creator: string;
  contractHash: string;
  tokenId: string;
  payToken?: string;
  price: string;
  startTime: number;
  buyer?: string;
  additionalRecipient?: string;
  status: "pending" | "suceed" | "canceled";
}

const sellOrderSchema = new Schema<ISellOrder>(
  {
    creator: { type: String, required: true },
    contractHash: { type: String, required: true },
    tokenId: { type: String, required: true },
    buyer: { type: String },
    payToken: { type: String },
    price: { type: String, required: true },
    startTime: { type: Number, required: true },
    additionalRecipient: { type: String },
    status: {
      type: String,
      enum: ["pending", "suceed", "canceled"],
      required: true,
    },
  },
  { timestamps: true }
);

const SellOrder = model<ISellOrder>("SellOrder", sellOrderSchema);

export default SellOrder;
