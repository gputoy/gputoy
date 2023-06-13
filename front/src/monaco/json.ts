import schema from '$gen/runner.schema.json'
import type * as monaco from 'monaco-editor'

const setJSONSchema = (m: typeof monaco) => {
	m.languages.json.jsonDefaults.setDiagnosticsOptions({
		validate: true,
		allowComments: false,
		schemas: [
			{
				uri: 'http://gputoy.io/runner.schema.json',
				fileMatch: ['*'],
				schema
			}
		],
		schemaValidation: 'error'
	})
}

export default setJSONSchema
