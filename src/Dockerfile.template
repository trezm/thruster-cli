FROM rust

# Expose ports for localhost:port to forward into container
EXPOSE 4321

WORKDIR /server


# CD-ing into api folder in container
RUN cd /server

# Copy the Cargo file so that we don't rebuild the whole container
COPY ./Cargo.toml .

RUN cargo install diesel_cli
RUN cargo install cargo-watch
RUN cargo install thruster-cli

# Adding all api files to container
COPY . .
RUN cargo build --release --example ping
RUN cargo build --release
