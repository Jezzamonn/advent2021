const fs = require('fs');
const path = require('path');

const filename = path.join(__dirname, 'input.txt');

function countIncreases(arr) {
    let count = 0;
    for (let i = 1; i < arr.length; i++) {
        if (arr[i] > arr[i - 1]) {
            count++;
        }
    }
    return count;
}

const depths = fs.readFileSync(filename, 'utf8').split('\n').map(x => parseInt(x.trim()));

const sampleCount = 3;

const depthSums = [];
for (let i = 0; i < depths.length - (sampleCount - 1); i++) {
    let sum = 0;
    for (let j = 0; j < sampleCount; j++) {
        sum += depths[i + j];
    }
    depthSums.push(sum);
}

console.log(countIncreases(depthSums));
