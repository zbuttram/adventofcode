import {open} from 'node:fs/promises';

(async function main() {
    const file = await open('input.txt', 'r');
    const top3 = [0,0,0];
    let count = 0;

    for await (const line of file.readLines()) {
        if (line === '') {
            const min = Math.min(...top3);
            if (count > min) {
                top3[top3.indexOf(min)] = count;
            }
            count = 0;
        } else {
            count += Number(line);
        }
    }

    console.log(top3.reduce((a, b) => a + b, 0))
})()