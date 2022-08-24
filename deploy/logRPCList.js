let rpclist = require('./rpslist.json')

rpclist.forEach(e => {
  let address = e.address
  let splits = address.split(":")
  let ip = splits[0]
  console.log(`"http://${ip}:7777/rpc",`)
})