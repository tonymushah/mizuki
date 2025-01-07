import {CodegenConfig} from '@graphql-codegen/cli'

const config: CodegenConfig = {
  schema: './myschema.graphqls',
  documents: ['src/**/*.ts', 'src/**/*.svelte'],
  ignoreNoDocuments: true, // for better experience with the watcher
  generates: {
    './src/lib/gql/': {
      preset: 'client',
      plugins: [],
      config: {
        useTypeImports: true
      }
    }
  }
}

export default config
