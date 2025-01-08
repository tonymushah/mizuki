import {CodegenConfig} from '@graphql-codegen/cli'

const config: CodegenConfig = {
  schema: './myschema.graphqls',
  documents: ['src/**/*.tsx'],
  ignoreNoDocuments: true, // for better experience with the watcher
  generates: {
    './src/gql/': {
      preset: 'client',
      plugins: [],
      config: {
        useTypeImports: true
      }
    }
  }
}

export default config
