FROM rust:1.89.0
WORKDIR /app_leptos
RUN /bin/bash -c "cargo install trunk"
RUN /bin/bash -c "rustup target add wasm32-unknown-unknown"
COPY . .
EXPOSE 8080
CMD /bin/bash