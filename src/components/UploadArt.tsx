import React, { useState } from "react";
import { Button, Container, Typography, TextField } from "@material-ui/core";
import { makeStyles } from "@material-ui/core";
import { useDispatch, useSelector } from "react-redux";

import { ipfsUpload } from "../actions/artTokenAction";

const useStyles = makeStyles({
  inputContainer: {
    width: "100%",
    textAlign: "center",
    padding: '1rem',
  },
  input: {
    display: "none",
  },
  imgContainer: {
    borderWidth: "5px",
    borderStyle: "double",
    borderColor: "arkslategray",
    height: "25rem",
  },
  img: {
    width: "100%",
    height: "100%",
    padding: "24px 0px",
  },
  buttonContainer: {
    textAlign: "center",
  },
});

interface RootState {
  auction: any;
}

const UploadArt = () => {
  const dispatch = useDispatch();
  const classes = useStyles();
  const [uploadImage, setUploadImage] = useState<File | undefined>();
  const [imageUrl, setImageUrl] = useState<string | undefined>();
  const [name, setName] = useState("");
  const { accounts } = useSelector((state: RootState) => state.auction);

  const createNftHandler = async () => {
    dispatch({
      type: "SHOW_LOADER",
    });
    const metadata = {
      name: name,
      accounts: accounts[0],
    };

    dispatch(ipfsUpload(name, uploadImage));
  };

  return (
    <>
      <Container className={classes.inputContainer}>
        <input
          accept="image/*"
          className={classes.input}
          id="contained-button-file"
          multiple
          type="file"
          onChange={(event: any) => {
            let img: File = event.target.files[0];
            setUploadImage(img);
            setImageUrl(URL.createObjectURL(img));
          }}
        />
        <label htmlFor="contained-button-file">
          <Button variant="contained" color="primary" component="span">
            {imageUrl === undefined ? <>Upload Your Work</> : <>Choose another</>}
          </Button>
        </label>
      </Container>
      <Container className={classes.imgContainer}>
        <img src={imageUrl} alt="" className={classes.img} />
      </Container>
      <form noValidate>
        <TextField
          variant="outlined"
          margin="normal"
          fullWidth
          id="name"
          label="Name"
          autoComplete="name"
          autoFocus
          value={name}
          onChange={(event) => setName(event.currentTarget.value)}
        />
        <TextField
          variant="outlined"
          margin="normal"
          fullWidth
          id="accounts"
          label="Accounts"
          autoComplete="accounts"
          autoFocus
          aria-readonly="true"
          disabled
          value={accounts}
        />
      </form>
      <Container className={classes.buttonContainer}>
        <Typography component="p">
          We will upload the your work to IPFS to create your own NFT
        </Typography>
        <Button variant="contained" color="primary" onClick={createNftHandler}>
          UPLOAD TO IPFS
        </Button>
      </Container>
    </>
  );
};

export default UploadArt;
