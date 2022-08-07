import { ApolloServer } from 'apollo-server';

const typeDefs = `
  enum PhotoCategory {
    SELFIE
    PORTRAIT
    ACTION
    LANDSCAPE
    GRAPHIC
  }

  type Photo {
    id: ID!
    url: String!
    name: String!
    description: String
    category: PhotoCategory!
  }

  type Query {
    totalPhotos: Int!
    allPhotos: [Photo!]!
  }

  input PostPhotoInput {
    name: String!
    category: PhotoCategory=PORTRAIT
    description: String
  }


  type Mutation {
    postPhoto(input: PostPhotoInput!): Photo!
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

interface PostPhotoInput {
  name?: string,
  description?: string,
}

const resolvers = {
  Query: {
    totalPhotos: (): number => photos.length,
    allPhotos: (): Photo[] => photos,
  },

  Mutation: {
    postPhoto(parent: string, args: { input: PostPhotoInput }) {
      const newPhoto = new Photo({
        id: _id++,
        ...args.input,
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
