const uWS = require('uWebSockets.js')

uWS
  .App()
  .ws('/*', {
    idleTimeout: 32,
    maxPayloadLength: 512,
    maxBackpressure: 1024,
    compression: uWS.DEDICATED_DECOMPRESSOR_8KB,

    message: (ws, message, isBinary) => {
      let ok = ws.send(message, isBinary, true)
    }
  })
  .listen(9001, (listenSocket) => {
    listenSocket && console.log('Listening to port 9001')
  })
