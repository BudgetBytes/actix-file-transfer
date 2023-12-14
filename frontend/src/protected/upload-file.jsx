import React, { useContext, useState } from "react"
import { Paper, styled, Button, Stack, Typography } from "@mui/material";
import CloudUploadIcon from '@mui/icons-material/CloudUpload';
import { JWTContext } from "../App"
import { SPaper, VisuallyHiddenInput } from "../styled-components";


export default function UploadFile() {
    const { token } = useContext(JWTContext);
    const [files, setFiles] = useState([]);

    const handleFileChange = async (e) => {
        if (e.target.files.length > 0) {
            const selectedFiles = [...e.target.files];
            const formData = new FormData();

            for (const file of selectedFiles) {
                formData.append('file', file);
            }

            const response = await fetch("/user/upload", {
                method: 'post',
                headers: { 'Authorization': token },
                body: formData
            });

            if (!response.ok) console.error("Probably no authorization header provided.")

            setFiles([...selectedFiles]);
        }
    }



    return <SPaper variant="outlined">
        <Stack alignItems='center' justifyContent='center' spacing={2}>
            <Button component="label" variant="outlined" startIcon={<CloudUploadIcon />}>
                Upload files
                <VisuallyHiddenInput type="file" multiple onChange={handleFileChange} />
            </Button>
            {files.length > 0 && <>
                {files.map((file) => (
                    <Typography key={file.name} variant="h6">{file.name}</Typography>
                ))}
            </>}
        </Stack>
    </SPaper>
}