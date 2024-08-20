# Use a imagem base oficial do Rust
FROM rust:1.80

# Instala dependências do sistema
RUN apt-get update -qq && apt-get install -y libpq-dev

# Define o diretório de trabalho dentro do contêiner
WORKDIR /app

# Copia o arquivo Cargo.toml e Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Compila as dependências da aplicação
RUN cargo build --release

# Copia o restante do código da aplicação
COPY . .

# Compila a aplicação
RUN cargo install --path .

# Exponha a porta da aplicação
EXPOSE 8000

# Comando padrão para iniciar o servidor Rocket
CMD ["rocket_explorer"]
