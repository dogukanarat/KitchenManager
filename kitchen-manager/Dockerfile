FROM node:18-slim

WORKDIR /usr/src/kitchen-manager

COPY ./yarn.lock ./
COPY ./package.json ./

RUN yarn install

COPY . .

EXPOSE 3000

CMD [ "yarn", "start" ]