import React, { useState, useRef } from "react";
import { Typography, Grid } from "@material-ui/core";
import { EditorState } from "draft-js";
import { Editor } from "react-draft-wysiwyg";
import { convertToHTML } from 'draft-convert';
import "react-draft-wysiwyg/dist/react-draft-wysiwyg.css";
// import {} from "dra"

const Content = () => {
  const [editorState, setEditorState] = useState(() =>
    EditorState.createEmpty()
  );
  const [convertedContent, setConvertedContent] = useState<string>();
  const editor = useRef(null);
  //   function focusEditor() {
  //     editor.current.focus();
  //   }
  const handleEditorChange = (state: any) => {
    setEditorState(state);
    convertContentToHTML();
  }

  const convertContentToHTML = () => {
    let currentContentAsHTML = convertToHTML(editorState.getCurrentContent());
    setConvertedContent(currentContentAsHTML);
  }


  return (
    <React.Fragment>
      <Typography variant="h6" gutterBottom>
        Let's talk about your thougts
      </Typography>
      <Grid container spacing={3}>
        <div
          style={{
            border: "1px solid black",
            height: "22rem",
            cursor: "text",
            borderRadius: "0.2rem",
            padding: "0.4rem",
          }}
          //   onClick={focusEditor}
        >
          <Editor
            // ref={editor}
            editorState={editorState}
            onEditorStateChange={handleEditorChange}
            placeholder="Write something!"
            wrapperClassName="wrapper-class"
            editorClassName="editor-class"
            toolbarClassName="toolbar-class"
          />
        </div>
      </Grid>
    </React.Fragment>
  );
};

export default Content;
