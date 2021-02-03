/*
 * Shimport benchmarks for comparison
 */

import fs from 'fs';

const n = 25;

const files = fs.readdirSync('fixtures')
	.map(f => `fixtures/${f}`)
	.filter(x => x.endsWith('.js'))
	.map(file => {
		const source = fs.readFileSync(file);
		return {
			file,
			code: source.toString(),
			size: source.byteLength
		};
	});

Promise.resolve().then(async () => {
	function timeRun (code) {
		const start = process.hrtime.bigint();
		const parsed = parse(code);
		const end = process.hrtime.bigint();
		return Math.round(Number(end - start) / 1e6);
	}

	console.log('Module load time');
	{
		const start = process.hrtime.bigint();
		var { parse } = await import('../wasm_node/es_module_lexer.js');
		// await init();
		console.log(`> ${Math.round(Number(process.hrtime.bigint() - start) / 1e6) + 'ms'}`);
	}

	console.log('Cold Run, All Fixtures');
	let totalSize = 0;
	{
		let total = 0;
		files.forEach(({ code, size }) => {
			totalSize += size;
			total += timeRun(code);
		});
		console.log(`fixtures/*.js (${Math.round(totalSize / 1e3)} KiB)`);
		console.log(`> ${total + 'ms'}`);
		gc();
	}

	console.log(`\nWarm Runs (average of ${n} runs)`);
	files.forEach(({ file, code, size }) => {
		console.log(`${file} (${Math.round(size / 1e3)} KiB)`);

		let total = 0;
		for (let i = 0; i < n; i++) {
			total += timeRun(code);
			gc();
		}

		console.log(`> ${(total / n) + 'ms'}`);
	});

	console.log(`\nWarm Runs, All Fixtures (average of ${n} runs)`);
	{
		let total = 0;
		for (let i = 0; i < n; i++) {
			files.forEach(({ code }) => {
				total += timeRun(code);
			});
		}
		console.log(`fixtures/*.js (${Math.round(totalSize / 1e3)} KiB)`);
		console.log(`> ${(total / n) + 'ms'}`);
	}
});
