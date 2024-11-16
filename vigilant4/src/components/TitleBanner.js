import React, { useState, useEffect } from 'react';

const TitleBanner = ({ leftTitle, centerTitle, rightTitle }: { leftTitle: string, centerTitle: string, rightTitle: string }) => {
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

