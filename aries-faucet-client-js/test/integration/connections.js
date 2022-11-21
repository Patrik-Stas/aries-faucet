/* eslint-env jest */

const { loadEnvVariables } = require('../common/config-loader')
const { createApiClient } = require('../../src/client')
const uuid = require('uuid')
const logger = require('../common/logger')(__filename)

loadEnvVariables()

let apiClient

beforeAll(async () => {
  try {
    jest.setTimeout(1000 * 60 * 4)
    apiClient = createApiClient(process.env.API_URL, logger)
  } catch (err) {
    logger.error(`Error in beforeAll: ${err.stack}`)
    throw err
  }
})

describe('connections', () => {
  it('create and get connection', async () => {
    const connectionId = await apiClient.createConnection('Lord Of The Rings', 'Tolkien')
    const connection = await apiClient.getConnection(connectionId)
    expect(connection.title).toBe('Lord Of The Rings')
    expect(connection.author).toBe('Tolkien')
  })

  it('create and get connections', async () => {
    const title1 = uuid.v4()
    const title2 = uuid.v4()
    const connectionId1 = await apiClient.createConnection(title1, 'Tolkien')
    const connectionId2 = await apiClient.createConnection(title2, 'Harry')
    const connections = await apiClient.getConnections()
    expect(connections.find(b => connectionId1 === b.id && b.title === title1)).toBeDefined()
    expect(connections.find(b => connectionId2 === b.id && b.title === title2)).toBeDefined()
  })
})
