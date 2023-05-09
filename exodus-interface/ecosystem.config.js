module.exports = {
  apps: [{
    name: 'dunkirk-web-server',
    script: './server/index.js',
    watch: '.',
    env: {
      "PORT": process.env.PORT || 8081,
      "DATABASE_URL": process.env.DATABASE_URL,
    }
  }]
};
