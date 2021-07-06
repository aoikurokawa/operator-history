import Web3 from "web3";
import { ARTTOKENADDRESS, ARTTOKENABI } from "../artTokenConfig";

const Moralis = require("moralis");

export const ipfsUpload =
  (name: string, uploadImage: any) => async (dispatch: any) => {
    const file = new Moralis.File(name, uploadImage);
    await file.saveIPFS().then(() => {
      dispatch({
        type: "IPFS_UPLOAD",
        ipfsHash: file.hash(),
        ipfsUrl: file.ipfs(),
        name: name,
      });
      dispatch({
        type: "CLOSE_LOADER",
      });
    });
  };

export const getNFTFromMoralis = () => async (dispatch: any) => {
  const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
  const accounts = await web3.eth.getAccounts();

  const NFT = Moralis.Object.extend("Nft");
  const query = new Moralis.Query(NFT);
  await query.find().then((data: any) => {
    const nftArray: any[] = [];
    for (let i = 0; i < data.length; i++) {
      if (data[i].attributes["Account"] === accounts[0]) {
        if (data[i].attributes["IpfsUrl"] !== undefined) {
          nftArray.push(data[i]);
        }
      }
    }
    
    dispatch({
      type: "GET_MORALISNFTDATA",
      nftArray: nftArray,
    });
  });
};

export const getArtTokenContract = () => async (dispatch: any) => {
  const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
  const accounts = await web3.eth.getAccounts();
  const contractInstance = new web3.eth.Contract(ARTTOKENABI, ARTTOKENADDRESS, {
    from: accounts[0],
  });

  dispatch({
    type: "GET_CONTRACTDATA",
    contractInstance: contractInstance,
    accounts: accounts,
  });
};

export const mint =
  (hash: string, url: string, name: string) => async (dispatch: any) => {
    const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
    const accounts = await web3.eth.getAccounts();
    const contractInstance = new web3.eth.Contract(
      ARTTOKENABI,
      ARTTOKENADDRESS,
      { from: accounts[0] }
    );

    contractInstance.methods
      .mint(url)
      .send()
      .on("receipt", (receipt: any) => {
        const tokenId = receipt.events.Transfer.returnValues[2];
        console.log(tokenId);
        const nft = new Moralis.Object("Nft");
        nft.set("TokenId", tokenId);
        nft.set("Account", accounts[0]);
        nft.set("IpfsHash", hash);
        nft.set("IpfsUrl", url);
        nft.set("Name", name);
        nft.save();

        dispatch({
          type: "IPFS_CLEAR",
        });
      });
  };

export const updateIsSell = (objectId: string) => async (dispatch: any) => {
  const Nft = new Moralis.Object.extend("Nft");
  const query = new Moralis.Query(Nft);
  query.get(objectId).then(
    (result: any) => {
      result.set("IsSelled", true);
      result.save();
    },
    (error: any) => {
      console.log("Error occured", error);
    }
  );

  dispatch({
    type: "CLOSE_MODAL",
    title: "",
    functionType: "",
  });
};

export const getToken = (tokenId: string) => async () => {
  const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
  const accounts = await web3.eth.getAccounts();
  const contractInstance = new web3.eth.Contract(ARTTOKENABI, ARTTOKENADDRESS, {
    from: accounts[0],
  });

  contractInstance.methods
    .getTokenDetail(tokenId)
    .call()
    .then((res: any) => {
      console.log(res);
    });
};
