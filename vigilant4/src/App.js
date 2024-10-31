import React, { useEffect, useState } from 'react';

function App() {
    const [message, setMessage] = useState('');
    const [ws, setWs] = useState(null);

    useEffect(() => {
        const websocket = new WebSocket('ws://127.0.0.1:8080');

        websocket.onopen = () => {
            console.log('WebSocket connection established');
        };

        websocket.onmessage = (event) => {
            console.log('Message from server:', event.data);
            setMessage(event.data); // Update state with incoming message
        };

        websocket.onclose = () => {
            console.log('WebSocket connection closed');
        };

        setWs(websocket);

        return () => {
            websocket.close();
        };
    }, []);

    const sendMessage = () => {
        if (ws) {
            ws.send('Hello from React!');
        }
    };

    return (
        <div>
            <h1>WebSocket Communication</h1>
            <button onClick={sendMessage}>Send Message</button>
            <p>Message from Rust: {message}</p>
        </div>
    );
}

export default App;

