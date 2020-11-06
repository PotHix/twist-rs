const twist = require('../native/index.node');
const token = process.env.auth;
const query = "pothix";

console.log(twist.search(token, query));

module.exports = twist;
