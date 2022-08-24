let blake = require("blakejs")
let unlockId = "test_string_haha_123-45628374287474827847298749827498kaldjkajhdajhdakmzmcvnzdfncsdkncs";
function strToBytes(str) {
  const bytes = [];
  for (ii = 0; ii < str.length; ii++) {
    const code = str.charCodeAt(ii); // x00-xFFFF
    bytes.push(code & 255); // low, high
  }
  return bytes;
}

let unlockIdBytes = strToBytes(unlockId)
let h = blake.blake2b(Buffer.from(unlockIdBytes), null, 32)
console.log('h', Buffer.from(h).toString('hex'))