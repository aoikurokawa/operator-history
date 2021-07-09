import React, { ChangeEvent, useState } from "react";
import { Typography, Grid, TextField } from "@material-ui/core";
import { DropzoneArea } from "material-ui-dropzone";

const UploadImage = () => {
  const [image, setImage] = useState<File | undefined>();

  const handleChange = (e: any) => {
      console.log(e);
  } 
  
  console.log(image);
  return (
    <React.Fragment>
      <Typography variant="h6" gutterBottom>
        Show off your catchy photos
      </Typography>
      <Grid container spacing={3}>
        <DropzoneArea
          onChange={handleChange}
        />
      </Grid>
    </React.Fragment>
  );
};

export default UploadImage;
