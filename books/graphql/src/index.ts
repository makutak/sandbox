import { ApolloServer } from 'apollo-server';

const typeDefs = `
  enum PhotoCategory {
    SELFIE
    PORTRAIT
    ACTION
    LANDSCAPE
    GRAPHIC
  }

  type User {
    githubLogin: ID!
    name: String
    avatar: String
    postedPhotos: [Photo!]!
  }

  type Photo {
    id: ID!
    url: String!
    name: String!
    description: String
    category: PhotoCategory!
    postedBy: User!
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

let users: User[] = [
  { githubLogin: 'mHttrup', name: 'Mike Hattrub' },
  { githubLogin: 'gPlake', name: 'Glen Plake' },
  { githubLogin: 'sSchmidt', name: 'Scot Schmidt' },
];

let photos: Photo[] = [
  {
    id: 1,
    name: 'Dropping the Heart Chute',
    description: 'The heart chute is one of my favorite chutes',
    category: 'ACTION',
    githubUser: 'gPlake',
  },
  {
    id: 2,
    name: 'Enjoying the sunshine',
    category: 'SELFIE',
    githubUser: 'sSchmidt',
  },
  {
    id: 3,
    name: 'Gunbarrel 25',
    description: '25 laps on gunbarrel today',
    category: 'LANDSCAPE',
    githubUser: 'sSchmidt',
  }
];

class Photo {
  constructor(init?: Partial<Photo>) {
    Object.assign(this, init);
  }

  id?: number;
  url?: string;
  name?: string;
  description?: string;
  category?: string;
  githubUser?: string;
}

class User {
  constructor(init?: Partial<User>) {
    Object.assign(this, init);
  }

  githubLogin?: string;
  name?: string;
  avater?: string;
}

interface PostPhotoInput {
  name?: string,
  description?: string,
}

let _id: number =  4;

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
    url: (parent: Photo) => `http://example.com/img/${parent.id}.jpg`,
    postedBy: (parent: Photo) => {
      return users.find(u => u.githubLogin === parent.githubUser);
    },
  },

  User: {
    postedPhotos: (parent: User) => {
      return photos.filter(p => p.githubUser === parent.githubLogin);
    },
  },
};

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

server
  .listen()
  .then(({url}) => console.log(`GraphQL service running on ${url}`));
