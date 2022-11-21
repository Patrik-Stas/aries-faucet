import * as React from 'react'
import { Component } from 'react'
import Button from '@mui/material/Button'
import apiClient from './api-client'
import { ConnectionsTable } from './ConnectionsTable'
import Grid from '@mui/material/Unstable_Grid2'

export default class App extends Component {
  constructor () {
    super()
    this.state = {
      connections: []
    }
  }

  async reloadData () {
    try {
      const connections = await apiClient.getConnections()
      console.log(JSON.stringify(connections))
      this.setState({ connections })
    } catch (err) {
      if (err.response) {
        this.setState({ error: JSON.stringify(err.response) })
      } else {
        this.setState({ error: JSON.stringify(err) })
      }
    }
  }

  async createConnection () {
    await apiClient.createConnection('foo')
    await this.reloadData()
  }

  async componentDidMount () {
    await this.reloadData()
  }

  render () {
    return (
      <div>
        <Grid container spacing={2}>
          <Grid xs={2}></Grid>
          <Grid xs={8}>
            <Grid container spacing={2}>
              <Grid xs={12}>
                <Button variant="contained" onClick={() => this.createConnection()}>
                  Create connection
                </Button>
              </Grid>
              <Grid xs={12}>
                <ConnectionsTable resources={this.state.connections}/>
              </Grid>
            </Grid>
          </Grid>
          <Grid xs={2}></Grid>
        </Grid>


      </div>
    )
  }
}

