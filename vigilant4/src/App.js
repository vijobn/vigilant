import React, { useEffect, useState } from 'react';
import VigilantTable from './components/TableView';
import TitleBanner from './components/TitleBanner';

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
    const [leftTitle, setLeftTitle] = useState('Every 2.0 secs');
    const [centerTitle, setCenterTitle] = useState('Main Title');
    // Initialize rightTitle with the current time in the watch(1) format
    const [rightTitle, setRightTitle] = useState(getCurrentTime());

    // Function to get the current time in the watch(1) format: `Tue Nov  7 13:42:15`
    function getCurrentTime() {
        return new Intl.DateTimeFormat('en-US', {
            weekday: 'short', // Abbreviated day (e.g., Tue)
            month: 'short',   // Abbreviated month (e.g., Nov)
            day: '2-digit',   // Day of the month with leading zero (e.g., 07)
            hour: '2-digit',  // Hour (24-hour clock)
            minute: '2-digit',// Minute
            second: '2-digit',// Second
            hour12: false      // 24-hour clock
        }).format(new Date());
    }
 
    // Update the rightTitle every 10 seconds
    useEffect(() => {
        const intervalId = setInterval(() => {
            const newRightTitle = getCurrentTime(); // Get the current time in watch(1) format
            setRightTitle(newRightTitle); // Update the rightTitle state
        }, 10000); // 10000ms = 10 seconds
 
        // Clean up the interval on component unmount
        return () => {
            clearInterval(intervalId);
        };
    }, []);

    useEffect(() => {
        const websocket = new WebSocket('ws://127.0.0.1:8080');

        websocket.onopen = () => {
            console.log('WebSocket connection established');
        };

        websocket.onmessage = (event) => {
            console.log('Message from server:', event.data);

                  // Check if the message contains the `settitle` command
            try {
                const jsonMessage = JSON.parse(event.data);
                if (jsonMessage.cmd === 'settitle' && jsonMessage.title) {
                    if (jsonMessage.left) {
                        setLeftTitle(jsonMessage.left);
                    }
                    if (jsonMessage.right) {
                        setRightTitle(jsonMessage.right);
                    }
                    if (jsonMessage.center) {
                        setCenterTitle(jsonMessage.center);
                    }
                //setTitle(jsonMessage.title); // Update the title based on the `settitle` command
                }
            } catch (error) {
                console.error('Failed to parse WebSocket message:', error);
            }

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
            {/* Title Banner Component */}
            <TitleBanner leftTitle={leftTitle} centerTitle={centerTitle} rightTitle={rightTitle} />

            <button onClick={sendJsonMessage}>Send Message</button>
            <p>Message from Rust: {message}</p>

            <VigilantTable headers={headers} data={data}/>
        </div>
    );
}

export default App;

