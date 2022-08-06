import { ApolloServer } from 'apollo-server';

const typeDefs = `
  type Photo {
    id: ID!
    url: String!
    name: String!
    description: String
  }

  type Query {
    totalPhotos: Int!
    allPhotos: [Photo!]!
  }

  type Mutation {
    postPhoto(name: String! description: String): Photo!
  }
`;

let _id = 0;
let photos: any[] = [];

const resolvers = {
  Query: {
    totalPhotos: (): number => photos.length,
    allPhotos: (): string[] => photos,
  },

  Mutation: {
    postPhoto(parent: string, args: any) {
      const newPhoto = {
        id: _id++,
        ...args,
      };

      photos.push(newPhoto);
      return newPhoto;
    }
  },

  Photo: {
    url: (parent: any) => `http://example.com/img/${parent.id}.jpg`
  }
};

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

server
  .listen()
  .then(({url}) => console.log(`GraphQL service running on ${url}`));
