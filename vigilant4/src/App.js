import React, { useEffect, useState } from 'react';
import VigilantTable from './components/TableView';
import TitleBanner from './components/TitleBanner';

function App() {
    const [message, setMessage] = useState('');
    const [ws, setWs] = useState(null);
    const [showData, setShowData] = useState([ ]);
    const [headers, setHeaders] = useState(['Name', 'Age', 'Country']); // Initial headers
    const [leftTitle, setLeftTitle] = useState('Every 10.0 secs');
    const [centerTitle, setCenterTitle] = useState('Main Title');
    const [rightTitle, setRightTitle] = useState(getCurrentTime());

    // Function to get the current time in the watch(1) format
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

            try {
                const jsonMessage = JSON.parse(event.data);
                if (jsonMessage.command === 'SetTitle') {
                    if (jsonMessage.left) setLeftTitle(jsonMessage.left);
                    if (jsonMessage.right) setRightTitle(jsonMessage.right);
                    if (jsonMessage.center) setCenterTitle(jsonMessage.center);
                } else if (jsonMessage.command === 'SetHeaders') {
                    //setHeaders(jsonMessage.headers); // Update headers
                } else if (jsonMessage.command === 'SetDataRow') {
                    // Extract index, value, name, country, and age from the message
                    const { index, name, country, age } = jsonMessage;
                    // Update the row in showData based on the received index
                    setShowData((prevData) => {
                        const updatedData = [...prevData];
                        // Check if the index exists; if not, push a new row
                        if (updatedData[index]) {
                            updatedData[index] = {
                                ...updatedData[index],  // Keep the other values intact
                                name,                   // Update name
                                country,                // Update country
                                age,                    // Update age
                            };
                        } else {
                            // If index does not exist, add a new row
                            updatedData.push({
                                name,
                                country,
                                age,
                            });
                        }
                        return updatedData;
                    });
                }
            } catch (error) {
                console.error('Failed to parse WebSocket message:', error);
            }
            console.log('Data now is', showData);

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

    return (
        <div>
            {/* Title Banner Component */}
            <TitleBanner leftTitle={leftTitle} centerTitle={centerTitle} rightTitle={rightTitle} />

            {/* Render table with dynamic headers and data */}
            <VigilantTable headers={headers} data={showData} />
        </div>
    );
}

export default App;

