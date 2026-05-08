import puppeteer from 'puppeteer';
import { readFile } from 'fs/promises';
import { resolve } from 'path';

async function main() {
    const jsonPath = process.argv[2];
    const outPrefix = process.argv[3];
    
    if (!jsonPath || !outPrefix) {
        console.error("Usage: tsx generate_baseline.ts <input_html_json> <output_png_prefix>");
        process.exit(1);
    }
    
    const content = JSON.parse(await readFile(jsonPath, 'utf-8'));
    const browser = await puppeteer.launch({ headless: true, args: ['--no-sandbox', '--disable-setuid-sandbox'] });
    
    for (const slide of content) {
        const page = await browser.newPage();
        
        // Wrap the raw HTML in a proper document structure to ensure predictable layout
        const fullHtml = `
            <!DOCTYPE html>
            <html>
                <head>
                    <style>
                        body { margin: 0; padding: 0; }
                    </style>
                </head>
                <body>
                    ${slide.html}
                </body>
            </html>
        `;
        
        await page.setContent(fullHtml, { waitUntil: 'networkidle0' });
        
        // Extract dimensions from the outer slide div
        const dimensions = await page.evaluate(() => {
            const div = document.querySelector('body > div') as HTMLElement;
            return {
                width: div ? parseInt(div.style.width) || 960 : 960,
                height: div ? parseInt(div.style.height) || 720 : 720
            };
        });
        
        await page.setViewport(dimensions);
        
        const outPath = `${outPrefix}_slide_${slide.index}.png`;
        await page.screenshot({ path: outPath, clip: { x: 0, y: 0, width: dimensions.width, height: dimensions.height } });
        console.log(`Generated ${outPath}`);
        
        await page.close();
    }
    
    await browser.close();
}

main();
