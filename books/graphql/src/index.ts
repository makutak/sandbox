import { ApolloServer } from 'apollo-server';
import { readFileSync } from 'fs';
import { GraphQLScalarType } from 'graphql';
import path from 'path';
import { MutationPostPhotoArgs, Photo, PhotoCategory, QueryAllPhotosArgs, User } from './generated/graphql';

const typeDefs = readFileSync(path.resolve(__dirname, '../typeDefs.graphql'), 'utf-8');

let users = [
  { githubLogin: 'mHattrup', name: 'Mike Hattrub' },
  { githubLogin: 'gPlake', name: 'Glen Plake' },
  { githubLogin: 'sSchmidt', name: 'Scot Schmidt' },
];

let photos = [
  {
    id: 1,
    name: 'Dropping the Heart Chute',
    description: 'The heart chute is one of my favorite chutes',
    category: 'ACTION',
    githubUser: 'gPlake' ,
    created: '3-28-1977',
  },
  {
    id: 2,
    name: 'Enjoying the sunshine',
    category: 'SELFIE',
    githubUser: 'sSchmidt',
    created: '1-2-1985',
  },
  {
    id: 3,
    name: 'Gunbarrel 25',
    description: '25 laps on gunbarrel today',
    category: 'LANDSCAPE',
    githubUser: 'sSchmidt',
    created: '2018-04-15',
  },
];

const tags = [
  { photoID: 1, userID: 'gPlake' },
  { photoID: 2, userID: 'sSchmidt' },
  { photoID: 2, userID: 'mHattrup' },
  { photoID: 2, userID: 'gPlake' },
  { photoID: 3, userID: 'mHattrup' },
]

let _id: number =  4;

const resolvers = {
  Query: {
    totalPhotos: () => photos.length,
    allPhotos: (after: QueryAllPhotosArgs) => photos,
  },

  Mutation: {
    postPhoto(parent: string, args: MutationPostPhotoArgs) {
      const newPhoto = {
        id: _id++,
        ...args.input,
        created: new Date(),
      } as any;

      photos.push(newPhoto);
      return newPhoto;
    }
  },

  Photo: {
    url: (parent: Photo) => `http://example.com/img/${parent.id}.jpg`,
    postedBy: (parent: any) => {
      return users.find(u => u.githubLogin === parent.githubUser);
    },
    taggedUsers: (parent: any) => {
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

  DateTime: new GraphQLScalarType({
    name: 'DateTime',
    description: 'A valid date time value.',
    parseValue: (value: any) => new Date(value),
    serialize: (value: any) => new Date(value).toISOString(),
    parseLiteral: (ast: any) => ast.value,
  })
};

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

server
  .listen()
  .then(({url}) => console.log(`GraphQL service running on ${url}`));
