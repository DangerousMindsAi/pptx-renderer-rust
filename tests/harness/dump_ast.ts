import { readFile, writeFile } from 'fs/promises';
import { JSDOM } from 'jsdom';

// Polyfill DOMParser for Node environment
const dom = new JSDOM();
global.DOMParser = dom.window.DOMParser;

import { parseZip, buildPresentation, serializePresentation } from '../../../pptx-renderer/src/index';

async function main() {
  try {
    const filePath = process.argv[2];
    const outPath = process.argv[3];
    if (!filePath || !outPath) {
        console.error("Usage: tsx dump_ast.ts <input_pptx> <output_json>");
        process.exit(1);
    }
    console.log(`Loading ${filePath}...`);
    
    const buffer = await readFile(filePath);
    console.log(`Parsing ZIP...`);
    const pptxFiles = await parseZip(buffer);
    
    console.log(`Building Presentation...`);
    const presentation = await buildPresentation(pptxFiles);
    
    console.log(`Serializing Presentation...`);
    const serialized = serializePresentation(presentation);
    
    await writeFile(outPath, JSON.stringify(serialized, null, 2));
    console.log(`Wrote ${outPath} successfully.`);
  } catch (err) {
    console.error(`Error:`, err);
    process.exit(1);
  }
}

main();
