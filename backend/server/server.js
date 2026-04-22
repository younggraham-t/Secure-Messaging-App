const express = require('express')

const app = express()
const port = 3000


const CA = require("../security/ca/ca")
app.get('/ca/public-key/:user', (req, res) => {
	const user = req.params.user
	const userPubKey = CA.getPublicKey(user);

	res.json({key: userPubKey});
	
})

app.listen(port, () => {
  console.log(`Messaging App backend on port: ${port}`)
})
