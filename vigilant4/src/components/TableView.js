import React from 'react';
import { MDBTable, MDBTableHead, MDBTableBody } from 'mdb-react-ui-kit';
import 'mdb-react-ui-kit/dist/css/mdb.min.css'; // Import the CSS for proper styling

// Define the props types
interface RowData {
  [key: string]: any; // Row data has dynamic keys
}

interface VigilantTableProps {
  headers: string[]; // Array of header names (e.g., ["Name", "Age", "Country"])
  data: RowData[]; // Array of row objects passed as a prop
}

const VigilantTable: React.FC<VigilantTableProps> = ({ headers, data }) => {
  return (
    <div style={{ margin: '20px' }}>
      <h2>My React Table with MDB UI Kit</h2>
      <MDBTable striped hover>
        <MDBTableHead>
          <tr>
            {/* Render the headers dynamically */}
            {headers.map((header, index) => (
              <th key={index}>{header}</th>
            ))}
          </tr>
        </MDBTableHead>
        <MDBTableBody>
          {data.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {/* Render each cell dynamically based on the row's properties */}
              {headers.map((header, headerIndex) => (
                <td key={headerIndex}>{row[header.toLowerCase()]}</td>
              ))}
            </tr>
          ))}
        </MDBTableBody>
      </MDBTable>
    </div>
  );
};

export default VigilantTable;

