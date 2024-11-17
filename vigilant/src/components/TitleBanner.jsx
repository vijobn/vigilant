import React, { useState, useEffect } from 'react';

const TitleBanner = () => {
  // State to hold the dynamic title values
  const [leftTitle, setLeftTitle] = useState('Left Title');
  const [centerTitle, setCenterTitle] = useState('Center Title');
  const [rightTitle, setRightTitle] = useState('Right Title');

  // Setup WebSocket connection
  useEffect(() => {
    const ws = new WebSocket('ws://127.0.0.1:8080'); // Connect to the WebSocket server

    ws.onopen = () => {
      console.log('WebSocket connection established');
    };

    ws.onmessage = (event) => {
      // Parse incoming message and update state based on the content
      const message = JSON.parse(event.data);

      // Update the titles based on the message content
      if (message.command === 'SetTitle') {
        setLeftTitle(message.leftTitle || leftTitle); // Update leftTitle if available
        setCenterTitle(message.centerTitle || centerTitle); // Update centerTitle if available
        setRightTitle(message.rightTitle || rightTitle); // Update rightTitle if available
      }
    };

    ws.onclose = () => {
      console.log('WebSocket connection closed');
    };

    return () => {
      ws.close(); // Close WebSocket when component unmounts
    };
  }, [leftTitle, centerTitle, rightTitle]);

  return (
    <div style={{
      display: 'flex',
      justifyContent: 'space-between', // Space out left, center, and right
      alignItems: 'center',
      backgroundColor: '#007bff', // Banner background color (blue)
      color: 'white', // Text color (white)
      padding: '10px 20px',
      fontSize: '18px',
      fontWeight: 'bold',
      whiteSpace: 'nowrap', // Prevent text wrapping
      height: '60px', // Ensure the banner has a sufficient height
      border: '1px solid #000', // Temporary border for debugging
    }}>
      {/* Left Section */}
      <div style={{ textAlign: 'left', paddingLeft: '10px', whiteSpace: 'nowrap' }}>
        {leftTitle}
      </div>

      {/* Center Section */}
      <div style={{ textAlign: 'center', whiteSpace: 'nowrap' }}>
        {centerTitle}
      </div>

      {/* Right Section */}
      <div style={{ textAlign: 'right', paddingRight: '10px', whiteSpace: 'nowrap' }}>
        {rightTitle}
      </div>
    </div>
  );
};

export default TitleBanner;
