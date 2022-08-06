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

let _id: number = 0;
let photos: Photo[] = [];

class Photo {
  constructor(init?: Partial<Photo>) {
    Object.assign(this, init);
  }

  id?: number;
  url?: string;
  name?: string;
  description?: string;
}

interface PhotoInput {
  name?: string,
  description?: string,
}

const resolvers = {
  Query: {
    totalPhotos: (): number => photos.length,
    allPhotos: (): Photo[] => photos,
  },

  Mutation: {
    postPhoto(parent: string, args: PhotoInput) {
      const newPhoto = new Photo({
        id: _id++,
        ...args,
      });

      photos.push(newPhoto);
      return newPhoto;
    }
  },

  Photo: {
    url: (parent: Photo) => `http://example.com/img/${parent.id}.jpg`
  }
};

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

server
  .listen()
  .then(({url}) => console.log(`GraphQL service running on ${url}`));
