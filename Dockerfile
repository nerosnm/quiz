FROM node:latest AS build

# Install rustup and wasm-pack into /root
WORKDIR /root

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
ENV PATH=/root/.cargo/bin:$PATH

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install node modules
WORKDIR /src

COPY package.json yarn.lock ./
RUN yarn install

# Build the project
COPY . .
RUN yarn build

FROM node:latest AS run

# Copy the bundled app and deps
WORKDIR /app

COPY --from=build /src/dist ./dist
COPY --from=build /src/node_modules ./node_modules

ENTRYPOINT ["node", "./dist/server.js"]
