import { Schema, model } from "mongoose";

interface ICollectionNFT {
  contractHash: string;
  slug: string;
  name: string;
  symbol: string;
  description: string;
  image: string;
  twitter: string;
  discord: string;
  website: string;
  categories: any[];
}

const collectionNFTSchema = new Schema<ICollectionNFT>({
  contractHash: { type: String, required: true, unique: true, dropDups: true },
  slug: { type: String, required: true, unique: true, dropDups: true },
  symbol: { type: String, required: true },
  name: { type: String, required: true },
  description: { type: String, required: true },
  image: { type: String, required: true },
  twitter: { type: String },
  discord: { type: String },
  website: { type: String },
});

const CollectionNFT = model<ICollectionNFT>(
  "CollectionNFT",
  collectionNFTSchema
);

export default CollectionNFT;
