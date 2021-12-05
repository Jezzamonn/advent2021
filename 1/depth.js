const fs = require('fs');
const path = require('path');

const filename = path.join(__dirname, 'input.txt');

let lastDepth;
let totalIncreases = 0;
for (const line of fs.readFileSync(filename, 'utf-8').split('\n')) {
    console.log();
    const depth = parseInt(line.trim());
    if (lastDepth !== undefined) {
        const isIncrease = depth > lastDepth;
        if (isIncrease) {
            totalIncreases++;
        }
    }
    lastDepth = depth;
}

console.log(totalIncreases);