const fs = require('fs');
const path = require('path');

const filename = path.join(__dirname, 'input.txt');

const lines = fs.readFileSync(filename, 'utf8').split('\n');
const numLines = lines.length;
const onesCount = Array(lines[0].length).fill(0);

for (const line of lines) {
    for (let i = 0; i < line.length; i++) {
        if (line[i] === '1') {
            onesCount[i]++;
        }
    }
}

const maxBinaryString = onesCount.map(v => v > numLines / 2 ? '1' : '0').join('');
const minBinaryString = onesCount.map(v => v > numLines / 2 ? '0' : '1').join('');

const gamma = parseInt(maxBinaryString, 2);
const epsilon = parseInt(minBinaryString, 2);
console.log(gamma * epsilon);