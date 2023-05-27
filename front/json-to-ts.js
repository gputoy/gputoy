import { compile } from 'json-schema-to-typescript'
import * as fs from 'node:fs/promises'

const bannerComment = `
  /**
 * This file was automatically generated from 'cargo types' command.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source code in gpu-common,
 * and run 'cargo types' again to regenerate this file.
 */
`
async function main() {
	let [_c, _p, schemaPath] = process.argv
	if (schemaPath == undefined) {
		console.error('Expected schema path as first and only argument')
	}

	let schemaFile = await fs.readFile(schemaPath)

	let compiledTypes = new Set()
	let schema = JSON.parse(schemaFile.toString())
	let compiled = await compile(schema, schema.title, {
		additionalProperties: false,
		format: true,
		declareExternallyReferenced: true,
		enableConstEnums: true,
		bannerComment
	})

	let eachType = compiled.split('export')
	for (let type of eachType) {
		if (!type) {
			continue
		}
		if (type.startsWith('/')) compiledTypes.add(type.trim())
		else compiledTypes.add('export ' + type.trim())
	}

	let output = Array.from(compiledTypes).join('\n\n')
	// stdout
	console.log(output)
}

await main()
