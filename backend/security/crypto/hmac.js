const crypto = require("crypto");

function generateHMAC(data, key) {
  return crypto.createHmac("sha256", key)
    .update(data)
    .digest("base64");
}

function verifyHMAC(data, key, received) {
  const expected = generateHMAC(data, key);

  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(received)
  );
}

module.exports = { generateHMAC, verifyHMAC };