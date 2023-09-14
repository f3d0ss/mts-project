# IPFS GATEWAY
To use ipfs:
- install ipfs client (kubo)
- allow CORS: `ipfs config --json API.HTTPHeaders.Access-Control-Allow-Origin '["*"]'`
- start daemon: `ipfs daemon`


# SCAN QR
- to make it work properly disable strict mode in `next.config.js` https://github.com/scanapp-org/html5-qrcode-react/issues/9#issuecomment-1487105390