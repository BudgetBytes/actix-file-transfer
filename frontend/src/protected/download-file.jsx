import React, {useContext, useState} from "react"
import { Button, Stack, Typography } from "@mui/material";
import { JWTContext } from "../App"
import { SPaper } from "../styled-components";
export default function DownloadFile() {
    const {token} = useContext(JWTContext);
    const [files, setFiles] = useState([]);
    const handleGetFiles = async () => {
        const response = await fetch("/user/getfiles", {
            headers: {'Authorization': token}
        })
        if(!response.ok) {
            console.error("Failed to load files from the server");
            return;
        }
        response.json().then((result) => {
            setFiles(result.files)
        });
    }

    const handleDownload = async (file) => {  
        const response = await fetch(`/user/download/${file}`, {
            headers: {'Authorization': token}
        });
    
        if (!response.ok) {
            console.error("Failed to load files from the server");
            return;
        }
    
        const blob = await response.blob();
        const link = document.createElement('a');
        const url = window.URL.createObjectURL(blob);
        link.href = url;
        link.download = file;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    
        window.URL.revokeObjectURL(url);
    };
    
    
    return <SPaper variant='outlined'>
        <Stack alignItems='center' justifyContent='center' spacing={2}>
        <Button variant="outlined" onClick={handleGetFiles}>Get files</Button>
        {files.length > 0 && <>
            {files.map((file) => (
                <Typography sx={{cursor: 'pointer'}} variant="h6" key={file} onClick={() => handleDownload(file)}>{file}</Typography>
            ))}
        </>}

        </Stack>
    </SPaper>
}