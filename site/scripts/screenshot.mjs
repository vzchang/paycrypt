// Full-page screenshot of the built explainer.
// Run: `npx http-server dist -p 4173 &` then `node scripts/screenshot.mjs`.
import { chromium } from "playwright";

const URL = process.env.SITE_URL || "http://localhost:4173";
const OUT = process.env.OUT || "screenshot.png";

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
await page.goto(URL, { waitUntil: "networkidle" });
// Wait for the hero so a blank or erroring page never gets captured.
await page.waitForSelector("h1", { timeout: 10_000 });
await page.screenshot({ path: OUT, fullPage: true });
await browser.close();
console.log(`wrote ${OUT}`);
