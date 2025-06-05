import 'dotenv/config';
import { Db, MongoClient } from 'mongodb';
import { ApolloServer } from 'apollo-server-express';
import express from 'express';
import expressPlayGround from 'graphql-playground-middleware-express';
import { readFileSync } from 'fs';
import path from 'path';
import { resolvers } from './resolvers';

const typeDefs = readFileSync(path.resolve(__dirname, '../typeDefs.graphql'), 'utf-8');

const start = async () => {
  const app = express();
  const MONGO_DB = process.env.DB_HOST!;
  const options = {
    useNewUrlParser: true,
    useUnifiedTopology: true,
    auth: {
      username: process.env.MONGO_INITDB_USER!,
      password: process.env.MONGO_INITDB_PWD!,
    },
  };
  let db: Db;
  try {
    const client = await MongoClient.connect(MONGO_DB, options);
    db = client.db();
  } catch (e) {
    console.log(`
      Mongo DB Host not found!
      please add DB_HOST environment variable to .env file
      exiting...
    `)
    console.dir(e);
    process.exit(1);
  }

  const server = new ApolloServer({
    typeDefs,
    resolvers,
    context: async () => {
      return { db }
    }
  });

  await server.start();
  server.applyMiddleware({ app });

  app.get('/', (req, res) => res.end('Welcome to the PhotoShare API'));
  app.get('/playground', expressPlayGround({ endpoint: '/graphql' }));

  app.listen({ port: 4000 }, () => {
    console.log(`GraphQL Server runnig @ http:/localhost:4000${server.graphqlPath}`)
  })
}

start();
