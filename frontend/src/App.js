import React, { useState, useEffect } from "react";


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
    const [error, setError] = useState('');

    const handleSave = async () => {
        const hash = await sha256(password);

        fetch("http://localhost:8080/auth", {
            method: "post",
            headers: { "content-type": "application/json" },
            body: JSON.stringify({
                username: username,
                password: hash
            })
        }).then(response => {
            if (response.ok) return response.text();
            else return null;
        }).then(data => {
            if (data !== null)
                setToken(data)
            else
                setError("Wrong credentials")

        })
    }

    return (
        <>
            {token === null ?
                <>
                    <input type="text" value={username} onChange={(e) => setUsername(e.target.value)} />
                    <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
                    <button onClick={handleSave}>Login</button>
                    {error && <h1>Wrong credentials</h1>}

                </>
                :
                <>
                    <h1>Logged successfully</h1>
                </>
            }

        </>

    )
}

export default App
