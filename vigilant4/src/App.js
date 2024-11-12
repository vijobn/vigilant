import React, { useState, useEffect } from 'react';
import VigilantTable from './components/TableView';
import TitleBanner from './components/TitleBanner';

function App() {
    const [message, setMessage] = useState('');
    const [ws, setWs] = useState(null);
    const [showData, setShowData] = useState([]);
    const [headers, setHeaders] = useState(['Name', 'Age', 'Country']); // Initial headers
    const [leftTitle, setLeftTitle] = useState('Every 10.0 secs');
    const [centerTitle, setCenterTitle] = useState('Main Title');
    const [rightTitle, setRightTitle] = useState(getCurrentTime());
    const [isConnected, setIsConnected] = useState(false);  // To track WebSocket connection status

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

        return () => clearInterval(intervalId);
    }, []);

    const createWebSocket = () => {
        const websocket = new WebSocket('ws://127.0.0.1:8080');

        websocket.onopen = () => {
            console.log('WebSocket connection established');
            setIsConnected(true);  // Update connection status
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
                    // setHeaders(jsonMessage.headers); // Update headers
                } else if (jsonMessage.command === 'SetDataRow') {
                    const { index, name, country, age } = jsonMessage;
                    setShowData((prevData) => {
                        const updatedData = [...prevData];
                        if (updatedData[index]) {
                            updatedData[index] = { ...updatedData[index], name, country, age };
                        } else {
                            updatedData.push({ name, country, age });
                        }
                        return updatedData;
                    });
                }
            } catch (error) {
                console.error('Failed to parse WebSocket message:', error);
            }
            setMessage(event.data);
        };

        websocket.onerror = (error) => {
            console.error('WebSocket error:', error);
            setIsConnected(false);  // Update connection status
            reconnectWebSocket();   // Try to reconnect when an error occurs
        };

        websocket.onclose = () => {
            console.log('WebSocket connection closed');
            setIsConnected(false);  // Update connection status
            reconnectWebSocket();   // Try to reconnect when the connection closes
        };

        setWs(websocket);
    };

    const reconnectWebSocket = () => {
        // Wait for a few seconds before trying to reconnect
        setTimeout(() => {
            console.log('Reconnecting WebSocket...');
            createWebSocket();  // Recreate the WebSocket connection
        }, 3000);  // Retry after 3 seconds
    };

    useEffect(() => {
        createWebSocket();

        return () => {
            if (ws) {
                ws.close();  // Close the WebSocket connection when the component unmounts
            }
        };
    }, []);

    // Method to send a hello message in JSON format
    const sendJsonMessage = () => {
        if (ws && isConnected) {
            const messageObject = { greeting: 'Hello', from: 'React' };
            const jsonMessage = JSON.stringify(messageObject);
            ws.send(jsonMessage);
            console.log('Sent JSON message:', jsonMessage);
        } else {
            console.log('WebSocket is not connected');
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
