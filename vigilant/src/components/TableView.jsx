import React, { useState, useEffect } from 'react';
import { MDBTable, MDBTableHead, MDBTableBody } from 'mdb-react-ui-kit';
import 'mdb-react-ui-kit/dist/css/mdb.min.css'; // Import the CSS for proper styling

const VigilantTable = ({ headers, data }) => {
  useEffect(() => {
    console.log('Headers received in VigilantTable:', headers);
    console.log('Data received in VigilantTable:', data);
  }, [headers, data]);

  // Helper function to safely access data properties
  const getCellData = (row, header) => {
    const key = header.toLowerCase().trim(); // Normalize header for matching
    return row[key] || '-'; // Return the value if it exists, otherwise fallback to "-"
  };

  // If no data, render an empty table body
  return (
    <div style={{ margin: '20px' }}>
      <MDBTable striped hover>
        <MDBTableHead>
          <tr>
            {headers.map((header, index) => (
              <th key={index}>{header}</th>
            ))}
          </tr>
        </MDBTableHead>
        <MDBTableBody>
          {data && data.length > 0 ? (
            data.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {headers.map((header, headerIndex) => (
                  <td key={headerIndex}>{getCellData(row, header)}</td>
                ))}
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={headers.length || 1}></td> {/* Empty row */}
            </tr>
          )}
        </MDBTableBody>
      </MDBTable>
    </div>
  );
};

export default VigilantTable;

