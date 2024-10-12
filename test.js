const lib  = require('.')
const path = require('node:path')
const fs   = require('node:fs')

const files = [
	path.join(__dirname, 'test_images', 'bc1unorm.dds'),
	path.join(__dirname, 'test_images', 'bc7unorm.dds'),
	path.join(__dirname, 'test_images', '4kbc7unorm.dds'),
]

for ( const file of files ) {
	const fileContent = fs.readFileSync(file)
	const start = new Date().getTime()
	const _ = lib.convert_dds(fileContent.buffer, 1024)
	console.log('File:', path.basename(file), 'Time:', (new Date()).getTime() - start, 'ms')
}