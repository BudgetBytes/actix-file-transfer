import React from "react";
import { Paper, styled } from "@mui/material";

export const SPaper = styled(Paper)({
    padding: 4,
    height: '100%',
    display: 'flex', alignItems: 'center',
    justifyContent: 'center'
});

export const VisuallyHiddenInput = styled('input')({
    clip: 'rect(0 0 0 0)',
    clipPath: 'inset(50%)',
    height: 1,
    overflow: 'hidden',
    position: 'absolute',
    bottom: 0,
    left: 0,
    whiteSpace: 'nowrap',
    width: 1,
});
