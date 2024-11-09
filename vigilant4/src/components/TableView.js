import React from 'react';
import { MDBTable, MDBTableHead, MDBTableBody } from 'mdb-react-ui-kit';
import 'mdb-react-ui-kit/dist/css/mdb.min.css'; // Import the CSS for proper styling

const VigilantTable = () => {
  const data = [
    { name: 'John Doe', age: 25, country: 'USA' },
    { name: 'Jane Smith', age: 30, country: 'Canada' },
    { name: 'Sam Johnson', age: 22, country: 'UK' },
    { name: 'Aby Thomas', age: 36, country: 'India' }
  ];

  return (
    <div style={{ margin: '20px' }}>
      <h2>My React Table with MDB UI Kit</h2>
      <MDBTable striped hover>
        <MDBTableHead>
          <tr>
            <th>Name</th>
            <th>Age</th>
            <th>Country</th>
          </tr>
        </MDBTableHead>
        <MDBTableBody>
          {data.map((row, index) => (
            <tr key={index}>
              <td>{row.name}</td>
              <td>{row.age}</td>
              <td>{row.country}</td>
            </tr>
          ))}
        </MDBTableBody>
      </MDBTable>
    </div>
  );
};

export default VigilantTable;
