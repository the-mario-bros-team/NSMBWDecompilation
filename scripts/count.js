const fs = require('fs');
let data = '';
process.stdin.on('data', d => data += d);
process.stdin.on('end', () => {
  const lines = data.split(/\r?\n/).filter(x => x.trim());
  console.log(lines.length);
});
