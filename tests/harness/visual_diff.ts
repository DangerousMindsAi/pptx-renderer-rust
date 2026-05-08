import pixelmatch from 'pixelmatch';
import { PNG } from 'pngjs';
import { readFileSync, writeFileSync } from 'fs';

function main() {
    const baselinePath = process.argv[2];
    const actualPath = process.argv[3];
    const diffPath = process.argv[4];

    if (!baselinePath || !actualPath || !diffPath) {
        console.error("Usage: tsx visual_diff.ts <baseline.png> <actual.png> <diff.png>");
        process.exit(1);
    }

    const baseline = PNG.sync.read(readFileSync(baselinePath));
    const actual = PNG.sync.read(readFileSync(actualPath));
    const { width, height } = baseline;
    
    if (width !== actual.width || height !== actual.height) {
        console.error(`Dimensions mismatch! Baseline: ${width}x${height}, Actual: ${actual.width}x${actual.height}`);
        process.exit(1);
    }
    
    const diff = new PNG({ width, height });
    
    const numDiffPixels = pixelmatch(
        baseline.data,
        actual.data,
        diff.data,
        width,
        height,
        { threshold: 0.1 }
    );
    
    writeFileSync(diffPath, PNG.sync.write(diff));
    
    const totalPixels = width * height;
    const diffPercent = (numDiffPixels / totalPixels) * 100;
    
    console.log(`Mismatch: ${diffPercent.toFixed(2)}% (${numDiffPixels} pixels)`);
    if (diffPercent > 1.0) {
        console.error(`Diff exceeded 1.0% threshold!`);
        process.exit(1);
    }
}

main();
