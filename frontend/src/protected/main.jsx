import React from "react";
import { Grid } from "@mui/material"
import UploadFile from "./upload-file";
import DownloadFile from "./download-file";

export default function Main() {

    return <Grid container spacing={2} alignItems='stretch' justifyContent='center'>
        <Grid item xs={12} lg={6}>
            <UploadFile />
        </Grid>
        <Grid item xs={12} lg={6}>
            <DownloadFile />
        </Grid>
    </Grid>
}