import { Schema, model } from "mongoose";

interface IUser {
  name: string;
  id: string;
  accountHash: string;
}

const userSchema = new Schema<IUser>({
  name: {
    type: String,
    required: true,
  },
  id: {
    type: String,
    required: true,
    unique: true,
    dropDups: true,
  },
  accountHash: {
    type: String,
    required: true,
    unique: true,
    dropDups: true,
  },
});
