import axios from 'axios'

function _axiosErrorToString (err) {
  if (err.response) {
    return `Request [${err.response.config.method}] ${err.response.config.url} failed! Response data: ${JSON.stringify(err.response.data)} HttpStatus: ${err.response.status} Headers: ${JSON.stringify(err.response.headers)}`
  } else {
    return `Request failed! No response received, error: ${err.stack}`
  }
}

function _axiosResponseToString (res) {
  return JSON.stringify(res.data, null, 2)
}

const noLog = {
  info: () => {},
  debug: () => {},
  error: () => {}
}

export function createApiClient (baseUrl, logger = noLog) {
  let headers = {}

  const processApiPrefix = '/api'

  async function getRequest (url) {
    if (logger) {
      logger.debug(`[Request] [GET] ${url}`)
    }
    let res
    try {
      res = await axios.get(url, { headers })
    } catch (err) {
      if (logger) {
        logger.error(`[Response] [GET] ${url} \n${_axiosErrorToString(err)}`)
      }
      throw err
    }
    if (logger) {
      logger.debug(`[GET] ${url} \nStatus code: ${res.status} \nResponse body: ${_axiosResponseToString(res)}`)
    }
    return res.data
  }

  async function postRequest (url, payload) {
    if (logger) {
      logger.debug(`[Request] [POST] ${url}\n Request body: ${JSON.stringify(payload, null, 2)}`)
    }
    let res
    try {
      res = await axios.post(url, payload, { headers })
    } catch (err) {
      if (logger) {
        logger.error(`[Response] [POST] ${url} \n ${_axiosErrorToString(err)}`)
      }
      throw err
    }
    if (logger) {
      logger.debug(`[Response] [POST] ${url} \nStatus code: ${res.status} \nResponse body: ${_axiosResponseToString(res)}`)
    }
    return res.data
  }

  async function createConnection(label) {
    const { id } = await postRequest(`${baseUrl}${processApiPrefix}/connections`, {label})
    return id
  }

  async function getConnection(resourceId) {
    return getRequest(`${baseUrl}${processApiPrefix}/connections/${resourceId}`)
  }

  async function getConnections(_filter) {
    return getRequest(`${baseUrl}${processApiPrefix}/connections`)
  }

  return {
    createConnection,
    getConnection,
    getConnections
  }
}
