import React, { useState, createContext } from "react";
import Main from "./protected/main";

export const JWTContext = createContext({ token: '' })

async function sha256(message) {
    const msgBuffer = new TextEncoder().encode(message);
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    return hashHex;
}

const App = () => {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [token, setToken] = useState(null);
    const [error, setError] = useState(null);

    const handleSave = async () => {
        const hash = await sha256(password);
        fetch('http://localhost:8080/user/encode', {
            method: 'post',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({
                username,
                hash
            })
        })
            .then(response => response.json())
            .then(data => {
                if (data.token !== '' ) {
                    setToken(data.token)
                } else {
                    setError(data.message)
                }
            });

    }

    return (
        <>
            {token === null ?
                <>
                    <input type="text" value={username} onChange={(e) => setUsername(e.target.value)} />
                    <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
                    <button onClick={handleSave}>Login</button>
                    {error && <h1>{error}</h1>}
                </>
                :
                <JWTContext.Provider value={{ token: token }}>
                    <Main />
                </JWTContext.Provider>
            }

        </>

    )
}

export default App
