import React, { useEffect, useState } from 'react';
import VigilantTable from './components/TableView';

function App() {
    const [message, setMessage] = useState('');
    const [ws, setWs] = useState(null);
    const data = [
        { name: 'John Doe', age: 25, country: 'USA' },
        { name: 'Jane Smith', age: 30, country: 'Canada' },
        { name: 'Sam Johnson', age: 22, country: 'UK' },
        { name: 'Aby Thomas', age: 36, country: 'India' },
        { name: 'Karen Joy', age: 39, country: 'USA' }
      ];
    const headers = ['Name', 'Age', 'Country']; // Dynamic header


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

    // Method to send a hello message in JSON format
    const sendJsonMessage = () => {
        if (ws) {
            const messageObject = { greeting: 'Hello', from: 'React' };
            const jsonMessage = JSON.stringify(messageObject); // Convert object to JSON string
            ws.send(jsonMessage); // Send the JSON message over WebSocket
            console.log('Sent JSON message:', jsonMessage);
        }
    };

    const sendMessage = () => {
        if (ws) {
            ws.send('Hello from React!');
        }
    };

    return (
        <div>
            <h1>WebSocket Communication</h1>
            <button onClick={sendJsonMessage}>Send Message</button>
            <p>Message from Rust: {message}</p>

            <VigilantTable headers={headers} data={data}/>
        </div>
    );
}

export default App;

