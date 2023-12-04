/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Human = {
  __typename?: 'Human';
  name: Scalars['String']['output'];
};

export type Query = {
  __typename?: 'Query';
  hero: Human;
};

export type Subscription = {
  __typename?: 'Subscription';
  helloWorld: Scalars['String']['output'];
};

export type GetHeroQueryVariables = Exact<{ [key: string]: never; }>;


export type GetHeroQuery = { __typename?: 'Query', hero: { __typename?: 'Human', name: string } };

export type MessageSubSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type MessageSubSubscription = { __typename?: 'Subscription', helloWorld: string };


export const GetHeroDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getHero"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"hero"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]} as unknown as DocumentNode<GetHeroQuery, GetHeroQueryVariables>;
export const MessageSubDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"MessageSub"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"helloWorld"}}]}}]} as unknown as DocumentNode<MessageSubSubscription, MessageSubSubscriptionVariables>;