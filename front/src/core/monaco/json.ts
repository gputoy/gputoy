import type Monaco from 'monaco-editor'
import JSONSchema from "../../../../schemas/Runner.json"

const setJSONSchema = (monaco: typeof Monaco) => {
    monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
        validate: true,
        allowComments: false,
        schemas: [
            {
                uri: "http://gputoy.io/runner.schema.json",
                fileMatch: ["*"],
                schema: JSONSchema,
            },
        ],
        schemaValidation: "error",
    })
}

export default setJSONSchema