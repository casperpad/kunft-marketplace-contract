import { Schema, model } from "mongoose";

interface IAsset {
  collectionNFT: Schema.Types.ObjectId;
  tokenId: string;
  image: string;
  name: string;
  mintDate: number;
  owner: Schema.Types.ObjectId;
  metadata: string;
}

const nftSchema = new Schema<IAsset>({
  collectionNFT: {
    type: Schema.Types.ObjectId,
    required: true,
    ref: "CollectionNFT",
  },
  tokenId: {
    type: String,
    required: true,
  },
  image: {
    type: String,
  },
  name: {
    type: String,
    required: true,
  },
  mintDate: {
    type: Number,
    required: true,
  },
  owner: {
    type: Schema.Types.ObjectId,
    ref: "User",
  },
  metadata: {
    type: String,
    required: true,
  },
});

const Asset = model<IAsset>("Asset", nftSchema);

export default Asset;
