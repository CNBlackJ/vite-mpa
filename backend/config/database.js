module.exports = ({ env }) => ({
  defaultConnection: 'default',
  connections: {
    default: {
      connector: 'bookshelf',
      settings: {
        client: 'mysql',
        host: env('DATABASE_HOST', 'rm-wz9wj2p4z62d8mm70yo.mysql.rds.aliyuncs.com'),
        port: env.int('DATABASE_PORT', 3306),
        database: env('DATABASE_NAME', 'feed'),
        username: env('DATABASE_USERNAME', 'feed'),
        password: env('DATABASE_PASSWORD', 'feed@2019'),
      },
      options: {},
    },
  },
});
