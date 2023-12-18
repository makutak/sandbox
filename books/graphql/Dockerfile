FROM node:16-alpine

WORKDIR /app

COPY ./src ./src
COPY package.json .
COPY pnpm-lock.yaml .
COPY tsconfig.json .
COPY codegen.yml .
COPY typeDefs.graphql .
COPY .env .

RUN npm install -g pnpm
