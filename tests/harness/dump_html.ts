import { readFile, writeFile } from 'fs/promises';
import { JSDOM } from 'jsdom';

// Polyfill DOM environment
const dom = new JSDOM();
global.DOMParser = dom.window.DOMParser;
global.document = dom.window.document;
global.window = dom.window as any;
global.HTMLElement = dom.window.HTMLElement;

import { parseZip, buildPresentation, renderSlide } from '../../../pptx-renderer/src/index';

async function main() {
  try {
    const filePath = process.argv[2];
    const outPath = process.argv[3];
    if (!filePath || !outPath) {
        console.error("Usage: tsx dump_html.ts <input_pptx> <output_html_json>");
        process.exit(1);
    }
    console.log(`Loading ${filePath}...`);
    
    const buffer = await readFile(filePath);
    console.log(`Parsing ZIP...`);
    const pptxFiles = await parseZip(buffer);
    
    console.log(`Building Presentation...`);
    const presentation = await buildPresentation(pptxFiles);
    
    console.log(`Rendering HTML for Slides...`);
    const slideHtmls = [];
    for (const slide of presentation.slides) {
      const handle = renderSlide(presentation, slide);
      // Normalize float values to 2 decimal places to avoid JS/Rust parity issues
      let html = handle.element.outerHTML.replace(/(\d+\.\d+)px/g, (match, p1) => {
          let num = parseFloat(p1);
          let str = num.toFixed(2).replace(/\.?0+$/, '');
          return str + 'px';
      });
      slideHtmls.push({
        index: slide.index,
        html
      });
    }
    
    await writeFile(outPath, JSON.stringify(slideHtmls, null, 2));
    console.log(`Wrote ${outPath} successfully.`);
  } catch (err) {
    console.error(`Error:`, err);
    process.exit(1);
  }
}

main();
