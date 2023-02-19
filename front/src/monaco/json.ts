import type * as monaco from 'monaco-editor'
import JSONSchema from '../../../schemas/Runner.json'

const setJSONSchema = (m: typeof monaco) => {
	m.languages.json.jsonDefaults.setDiagnosticsOptions({
		validate: true,
		allowComments: false,
		schemas: [
			{
				uri: 'http://gputoy.io/runner.schema.json',
				fileMatch: ['*'],
				schema: JSONSchema
			}
		],
		schemaValidation: 'error'
	})
}

export default setJSONSchema
