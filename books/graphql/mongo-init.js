const user = {
  user: 'mongo',
  pwd: 'mongo',
  roles: [
    {
      role: 'readWrite',
      db: 'photo_share',
    },
  ],
};

db.createUser(user);
