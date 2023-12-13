import React, { useContext } from "react"
import { JWTContext } from "../App"


export default function UploadFile() {
    const {token} = useContext(JWTContext);
    return <>
    {token}
    </>
}