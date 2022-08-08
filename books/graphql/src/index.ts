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
    inPhotos: [Photo!]!
  }

  type Photo {
    id: ID!
    url: String!
    name: String!
    description: String
    category: PhotoCategory!
    postedBy: User!
    taggedUsers: [User!]!
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
  { githubLogin: 'mHattrup', name: 'Mike Hattrub' },
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

const tags = [
  { photoID: 1, userID: 'gPlake' },
  { photoID: 2, userID: 'sSchmidt' },
  { photoID: 2, userID: 'mHattrup' },
  { photoID: 2, userID: 'gPlake' },
  { photoID: 3, userID: 'mHattrup' },
]

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
    taggedUsers: (parent: Photo) => {
        return tags
          .filter(tag => tag.photoID === parent.id)
          .map(tag => tag.userID)
          .map(userID => users.find(u => u.githubLogin === userID))
    },
  },

  User: {
    postedPhotos: (parent: User) => {
      return photos.filter(p => p.githubUser === parent.githubLogin);
    },
    inPhotos: (parent: User) => {
      return tags
        .filter(tag => tag.userID === parent.githubLogin)
        .map(tag => tag.photoID)
        .map(photoID => photos.find(p => p.id === photoID))
    }
  },
};

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

server
  .listen()
  .then(({url}) => console.log(`GraphQL service running on ${url}`));
