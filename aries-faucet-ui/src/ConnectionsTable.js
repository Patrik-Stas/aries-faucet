import React, { Component } from 'react'
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';

function createData( name, calories, fat, carbs, protein) {
  return { name, calories, fat, carbs, protein };
}

const rows = [
  createData('Frozen yoghurt', 159, 6.0, 24, 4.0),
  createData('Ice cream sandwich', 237, 9.0, 37, 4.3),
  createData('Eclair', 262, 16.0, 24, 6.0),
  createData('Cupcake', 305, 3.7, 67, 4.3),
  createData('Gingerbread', 356, 16.0, 49, 3.9),
];


export class ConnectionsTable extends Component {
  constructor (props) {
    super(props)
    this.state = {
      resourceToBeDeleted: null,
    }
  }

  render () {
    return (
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell>Id</TableCell>
              <TableCell align="right">Label</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {this.props.resources.map((row) => (
              <TableRow
                key={row.id}
                sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
              >
                <TableCell component="th" scope="row">{row.id}</TableCell>
                <TableCell align="right">{row.label}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
      // <div className="animated fadeIn">
      //   <Card>
      //     <CardHeader>
      //       <h3>Connections</h3>
      //     </CardHeader>
      //     <CardBody>
      //       <Table responsive size="sm">
      //         <thead>
      //         <tr>
      //           <th>Id</th>
      //           <th>Label</th>
      //         </tr>
      //         </thead>
      //         <tbody>
      //         {
      //           _(this.props.resources).values().map((resource) => {
      //             return (
      //               <tr key={resource.id}>
      //                 <td>{resource.id}</td>
      //                 <td>{resource.label}</td>
      //               </tr>
      //             )
      //           }).value()
      //         }
      //         </tbody>
      //       </Table>
      //     </CardBody>
      //   </Card>
      // </div>
    )
  }
}
