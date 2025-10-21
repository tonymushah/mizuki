/* eslint-disable */
import type {TypedDocumentNode as DocumentNode} from '@graphql-typed-document-node/core'
export type Maybe<T> = T | null
export type InputMaybe<T> = Maybe<T>
export type Exact<T extends {[key: string]: unknown}> = {[K in keyof T]: T[K]}
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]?: Maybe<T[SubKey]>
}
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & {
  [SubKey in K]: Maybe<T[SubKey]>
}
export type MakeEmpty<T extends {[key: string]: unknown}, K extends keyof T> = {
  [_ in K]?: never
}
export type Incremental<T> =
  | T
  | {[P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never}
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: {input: string; output: string}
  String: {input: string; output: string}
  Boolean: {input: boolean; output: boolean}
  Int: {input: number; output: number}
  Float: {input: number; output: number}
}

export type Human = {
  __typename?: 'Human'
  name: Scalars['String']['output']
}

export type Mutation = {
  __typename?: 'Mutation'
  sendMessage: Scalars['Boolean']['output']
}

export type MutationSendMessageArgs = {
  message: Scalars['String']['input']
}

export type Query = {
  __typename?: 'Query'
  hero: Human
  notHero: Human
}

export type Subscription = {
  __typename?: 'Subscription'
  helloWorld: Scalars['String']['output']
  watchMessages: Scalars['String']['output']
}

export type MessagesSubscriptionVariables = Exact<{[key: string]: never}>

export type MessagesSubscription = {
  __typename?: 'Subscription'
  watchMessages: string
}

export type SendMessageMutationVariables = Exact<{
  message: Scalars['String']['input']
}>

export type SendMessageMutation = {
  __typename?: 'Mutation'
  sendMessage: boolean
}

export type GetHeroQueryVariables = Exact<{[key: string]: never}>

export type GetHeroQuery = {
  __typename?: 'Query'
  hero: {__typename?: 'Human'; name: string}
}

export type NotHeroQueryVariables = Exact<{[key: string]: never}>

export type NotHeroQuery = {
  __typename?: 'Query'
  notHero: {__typename?: 'Human'; name: string}
}

export type MessageSubSubscriptionVariables = Exact<{[key: string]: never}>

export type MessageSubSubscription = {
  __typename?: 'Subscription'
  helloWorld: string
}

export const MessagesDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'subscription',
      name: {kind: 'Name', value: 'messages'},
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {kind: 'Field', name: {kind: 'Name', value: 'watchMessages'}}
        ]
      }
    }
  ]
} as unknown as DocumentNode<
  MessagesSubscription,
  MessagesSubscriptionVariables
>
export const SendMessageDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'mutation',
      name: {kind: 'Name', value: 'sendMessage'},
      variableDefinitions: [
        {
          kind: 'VariableDefinition',
          variable: {kind: 'Variable', name: {kind: 'Name', value: 'message'}},
          type: {
            kind: 'NonNullType',
            type: {kind: 'NamedType', name: {kind: 'Name', value: 'String'}}
          }
        }
      ],
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: {kind: 'Name', value: 'sendMessage'},
            arguments: [
              {
                kind: 'Argument',
                name: {kind: 'Name', value: 'message'},
                value: {
                  kind: 'Variable',
                  name: {kind: 'Name', value: 'message'}
                }
              }
            ]
          }
        ]
      }
    }
  ]
} as unknown as DocumentNode<SendMessageMutation, SendMessageMutationVariables>
export const GetHeroDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'query',
      name: {kind: 'Name', value: 'getHero'},
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: {kind: 'Name', value: 'hero'},
            selectionSet: {
              kind: 'SelectionSet',
              selections: [{kind: 'Field', name: {kind: 'Name', value: 'name'}}]
            }
          }
        ]
      }
    }
  ]
} as unknown as DocumentNode<GetHeroQuery, GetHeroQueryVariables>
export const NotHeroDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'query',
      name: {kind: 'Name', value: 'notHero'},
      selectionSet: {
        kind: 'SelectionSet',
        selections: [
          {
            kind: 'Field',
            name: {kind: 'Name', value: 'notHero'},
            selectionSet: {
              kind: 'SelectionSet',
              selections: [{kind: 'Field', name: {kind: 'Name', value: 'name'}}]
            }
          }
        ]
      }
    }
  ]
} as unknown as DocumentNode<NotHeroQuery, NotHeroQueryVariables>
export const MessageSubDocument = {
  kind: 'Document',
  definitions: [
    {
      kind: 'OperationDefinition',
      operation: 'subscription',
      name: {kind: 'Name', value: 'MessageSub'},
      selectionSet: {
        kind: 'SelectionSet',
        selections: [{kind: 'Field', name: {kind: 'Name', value: 'helloWorld'}}]
      }
    }
  ]
} as unknown as DocumentNode<
  MessageSubSubscription,
  MessageSubSubscriptionVariables
>
