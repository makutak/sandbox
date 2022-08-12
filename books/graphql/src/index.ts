import { ApolloServer } from 'apollo-server-express';
import express from 'express';
import expressPlayGround from 'graphql-playground-middleware-express';
import { readFileSync } from 'fs';
import path from 'path';
import { resolvers } from './resolver';

const typeDefs = readFileSync(path.resolve(__dirname, '../typeDefs.graphql'), 'utf-8');

const start = async () => {
  const app = express();

  const server = new ApolloServer({ typeDefs, resolvers });

  await server.start();
  server.applyMiddleware({ app });

  app.get('/', (req, res) => res.end('Welcome to the PhotoShare API'));
  app.get('/playground', expressPlayGround({ endpoint: '/graphql' }));

  app.listen({ port: 4000 }, () => {
    console.log(`GraphQL Server runnig @ http:/localhost:4000${server.graphqlPath}`)
  })
}

start();