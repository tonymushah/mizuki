/* eslint-disable */
import * as types from './graphql';
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n    subscription messages {\n        watchMessages\n    }\n": types.MessagesDocument,
    "\n    mutation sendMessage($message: String!) {\n        sendMessage(message: $message)\n    }\n": types.SendMessageDocument,
    "\nquery getHero {\n  hero {\n    name\n  }\n}\n": types.GetHeroDocument,
    "\n  query notHero {\n    notHero {\n      name\n    }\n  }\n": types.NotHeroDocument,
    "\n  subscription MessageSub {\n    helloWorld\n  }\n": types.MessageSubDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n    subscription messages {\n        watchMessages\n    }\n"): (typeof documents)["\n    subscription messages {\n        watchMessages\n    }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n    mutation sendMessage($message: String!) {\n        sendMessage(message: $message)\n    }\n"): (typeof documents)["\n    mutation sendMessage($message: String!) {\n        sendMessage(message: $message)\n    }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\nquery getHero {\n  hero {\n    name\n  }\n}\n"): (typeof documents)["\nquery getHero {\n  hero {\n    name\n  }\n}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query notHero {\n    notHero {\n      name\n    }\n  }\n"): (typeof documents)["\n  query notHero {\n    notHero {\n      name\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription MessageSub {\n    helloWorld\n  }\n"): (typeof documents)["\n  subscription MessageSub {\n    helloWorld\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;