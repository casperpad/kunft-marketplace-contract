import fs from "fs";
import path from "path";

interface IResult {
  name: string;
  description: string;
  tokens: {
    tokenId: number;
    name: string;
    trait: {
      [key: string]: any;
    };
  }[];
}

const generate = () => {
  const paths = [
    path.resolve(__dirname, "../../../../metadata/1.Human Male/metadata"),
    path.resolve(__dirname, "../../../../metadata/2.Human Female/metadata"),
    path.resolve(__dirname, "../../../../metadata/3.God Male/metadata"),
    path.resolve(__dirname, "../../../../metadata/4.God Female/metadata"),
  ];

  const result: IResult = {
    name: "KUNFT",
    description: "",
    tokens: [],
  };

  let tokenId = 0;

  paths.forEach((p) => {
    for (let i = 0; i < 1250; i++) {
      const tokenData = JSON.parse(
        fs.readFileSync(path.join(p, `${i}.json`), "utf8")
      );
      const trait: IResult["tokens"][number]["trait"] = {};
      tokenData.attributes.forEach(
        (att: { trait_type: string; value: string }) => {
          trait[att.trait_type.split("-")[1].toLowerCase()] = att.value;
        }
      );
      trait["image"] = tokenData.image;
      result.tokens.push({ tokenId, name: `Humans & Gods#${tokenId}`, trait });
      tokenId++;
    }
  });

  fs.writeFileSync(
    path.resolve(__dirname, "./output.json"),
    JSON.stringify(result, null, 2)
  );

  console.log(`Generated successfully.`);
};

generate();
