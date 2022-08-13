const user = {
  user: 'mongo',
  pwd: 'mongo',
  roles: [
    {
      role: 'dbOwner',
      db: 'photo_share',
    },
  ],
};

db.createUser(user);
