const path = require('path')
const dotenv = require('dotenv')

module.exports.loadEnvVariables = function loadEnvVariables () {
  const env = process.env.ENVIRONMENT || 'localhost:3800'
  const pathToConfig = path.resolve(__dirname, `../../config/${env}.env`)
  console.log(`Loading config: ${pathToConfig}`)
  dotenv.config({ path: pathToConfig })
}
