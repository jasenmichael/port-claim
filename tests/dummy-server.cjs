const http = require("http");

const port = process.argv[2]; // Port passed as argument from test

const server = http.createServer((req, res) => {
  res.end();
});

server.listen(port, () => {
  console.log(`Dummy server listening on port ${port}`);
});
