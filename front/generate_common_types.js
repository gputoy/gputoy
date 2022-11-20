import { compile } from 'json-schema-to-typescript'
import * as fs from 'node:fs/promises'
import * as path from 'path'
import * as url from 'url'

const dirname = path.dirname(url.fileURLToPath(import.meta.url))

async function main() {
  let schemasPath = path.join(dirname, '..', 'schemas')
  let schemaFiles = (await fs.readdir(schemasPath)).filter((x) => x.endsWith('.json'))

  // Compile all types, stripping out duplicates. This is a bit dumb but the easiest way to
  // do it since we can't suppress generation of definition references.
  let compiledTypes = new Set()
  for (let filename of schemaFiles) {
    let filePath = path.join(schemasPath, filename)
    let schema = JSON.parse(await fs.readFile(filePath))
    let compiled = await compile(schema, schema.title, {
      additionalProperties: false,
      format: true,
      declareExternallyReferenced: true,
    })

    let eachType = compiled.split('export')
    for (let type of eachType) {
      if (!type) {
        continue
      }
      if (type.startsWith("/"))
        compiledTypes.add(type)
      else
        compiledTypes.add('export ' + type)
    }
  }

  let output = Array.from(compiledTypes).join('\n\n')
  let outputPath = path.join(dirname, 'src', 'core/common.ts')

  try {
    let existing = await fs.readFile(outputPath)
    if (existing == output) {
      // Skip writing if it hasn't changed, so that we don't confuse any sort of incremental builds.
      // This check isn't ideal but the script runs quickly enough and rarely enough that it doesn't matter.
      console.log('Schemas are up to date')
      return
    }
  } catch (e) {
    // It's fine if there's no output from a previous run.
    if (e.code !== 'ENOENT') {
      throw e
    }
  }

  await fs.writeFile(outputPath, output)
  console.log(`Wrote Typescript types to ${outputPath}`)
}

main().catch((e) => {
  console.error(e)
  process.exit(1)
})