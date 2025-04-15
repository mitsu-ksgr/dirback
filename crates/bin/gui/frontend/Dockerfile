FROM node:23-bookworm-slim

# Development user: node
ARG USER_NAME=node
USER 1000
WORKDIR /app

# Copy the files required for dependency resolution.
COPY --chown=$USER_NAME:$USER_NAME package.json yarn.lock ./
COPY --chown=$USER_NAME:$USER_NAME src/ ./src/

RUN yarn install

CMD ["yarn", "vite", "dev", "--host", "0.0.0.0"]

