import { GraphQLScalarType } from 'graphql';
import { Db } from 'mongodb';
import {
  MutationPostPhotoArgs,
  Photo,
  QueryAllPhotosArgs,
} from './generated/graphql';

export const resolvers = {

  Query: {
    totalPhotos: (_parent: any, _args: any, { db }: { db: Db }) => {
      return db.collection('photos').estimatedDocumentCount();
    },

    allPhotos: (_parent: any, _after: QueryAllPhotosArgs, { db }: { db: Db }) => {
      return db.collection('photos').find().toArray();
    },

    totalUsers: (_parent: any, _args: any, { db }: { db: Db }) => {
      return db.collection('users').estimatedDocumentCount();
    },

    allUsers: (_parent: any, _args: any, { db }: { db: Db }) => {
      return db.collection('users').find().toArray();
    },
  },

  /*
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
  */
};
