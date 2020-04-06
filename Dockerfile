FROM rust:1.41 as server_build

WORKDIR /usr/app
# Create blank project
#RUN USER=root cargo init
# Copy Cargo.toml to get dependencies
# COPY Cargo.lock Cargo.lock
#COPY Cargo.toml .
# This is a dummy build to get the dependencies cached
#RUN cargo build --release
# Copy sources
COPY . .
RUN cargo build --release

# Create a new stage with a minimal image
# because we already have a binary built
FROM rust:slim
RUN apt-get update && apt-get install -y \
apt-transport-https \
ca-certificates \
curl \
gnupg-agent \
software-properties-common
# ==== Envoy install ====
# RUN curl -sL 'https://getenvoy.io/gpg' | apt-key add -
# RUN add-apt-repository \
# "deb [arch=amd64] https://dl.bintray.com/tetrate/getenvoy-deb \
# $(lsb_release -cs) \
# stable"
# RUN apt-get update && apt-get install -y getenvoy-envoy

# Copies the binary from the "build" stage to the current stage
COPY --from=server_build /usr/app/target/release/pg_sse /bin/
COPY --from=server_build /usr/app/cert cert

# Configures the startup!
CMD pg_sse

# Run with Envoy
# ADD ./start_service.sh /usr/local/bin/start_service.sh
# RUN chmod u+x /usr/local/bin/start_service.sh
# ENTRYPOINT /usr/local/bin/start_service.sh
