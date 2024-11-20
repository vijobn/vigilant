import React, { useState, useEffect } from 'react';
import { MDBTable, MDBTableHead, MDBTableBody } from 'mdb-react-ui-kit';
import 'mdb-react-ui-kit/dist/css/mdb.min.css'; // Import the CSS for proper styling

const VigilantTable = ({ headers, data }) => {
  useEffect(() => {
    console.log('Headers received in VigilantTable:', headers);
    console.log('Data received in VigilantTable:', data);
  }, [headers, data]);

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
              row.values ? ( // Access the 'values' array from the row object
                <tr key={rowIndex}>
                  {row.values.map((cell, cellIndex) => (
                    <td key={cellIndex}>{cell || '-'}</td> // Fallback to "-" if cell is empty
                  ))}
                </tr>
              ) : (
                <tr key={rowIndex}>
                  <td colSpan={headers.length || 1}>Invalid row format</td>
                </tr>
              )
            ))
          ) : (
            <tr>
              <td colSpan={headers.length || 1}>No data available</td>
            </tr>
          )}
        </MDBTableBody>
      </MDBTable>
    </div>
  );
};

export default VigilantTable;
