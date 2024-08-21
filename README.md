# Blockchain Explorer

O **Blockchain Explorer** é uma aplicação web desenvolvida em Rust, utilizando o framework Rocket, para explorar e interagir com uma blockchain. Este projeto tem como objetivo fornecer uma interface para visualizar transações, endereços e o estado atual da blockchain. Além disso, ele inclui uma implementação de "mempool" para gerenciar transações pendentes.

## Estrutura do Projeto

O projeto é organizado da seguinte maneira:

### Diretórios e Arquivos

- **src**: Contém o código-fonte da aplicação.
  - **config**: Configurações da aplicação.
    - `config.rs`: Define e carrega as configurações necessárias para a aplicação.
  - **controllers**: Controladores da aplicação que gerenciam as rotas e a lógica de entrada/saída.
    - `mod.rs`: Declara e expõe os controladores.
    - `transaction_controller.rs`: Lida com as rotas e operações relacionadas às transações.
    - `address_controller.rs`: Lida com as rotas e operações relacionadas aos endereços.
  - **db**: Interação com o banco de dados.
    - `mod.rs`: Declara e expõe os módulos de banco de dados.
    - `schema.rs`: Define o esquema do banco de dados e as tabelas.
    - `connection.rs`: Gerencia a conexão com o banco de dados.
  - **mempool**: Implementação da memória de transações pendentes.
    - `mod.rs`: Declara e expõe o módulo de mempool.
    - `mempool.rs`: Implementa a lógica de gerenciamento da mempool.
  - **models**: Definições de modelos de dados.
    - `mod.rs`: Declara e expõe os modelos.
    - `transaction.rs`: Define o modelo de dados para transações.
    - `address.rs`: Define o modelo de dados para endereços.
  - **services**: Serviços que encapsulam a lógica de negócios.
    - `mod.rs`: Declara e expõe os serviços.
    - `transaction_service.rs`: Lida com a lógica de negócios relacionada às transações.
    - `address_service.rs`: Lida com a lógica de negócios relacionada aos endereços.
  - **tests**: Testes para a aplicação.
    - `mod.rs`: Declara e organiza os testes.
    - `transaction_tests.rs`: Contém testes para o módulo de transações.
    - `address_tests.rs`: Contém testes para o módulo de endereços.
    - `mempool_tests.rs`: Contém testes para o módulo de mempool.
  - **utils**: Funções utilitárias e auxiliares.
    - `mod.rs`: Declara e expõe os utilitários.
    - `crypto.rs`: Funções e estruturas relacionadas a criptografia.
    - `parser.rs`: Funções para análise e processamento de dados.

- **target**: Diretório onde o Cargo compila os arquivos de código.

- **Cargo.lock**: Arquivo de bloqueio de dependências do Cargo.

- **Cargo.toml**: Arquivo de configuração do Cargo que define as dependências e configurações do projeto.

- **Rocket.toml**: Arquivo de configuração específico para o Rocket.

## Funcionalidades

- **Exploração de Transações**: Permite visualizar e buscar informações sobre transações na blockchain. 
- **Visualização de Endereços**: Permite visualizar e buscar informações sobre endereços na blockchain.
- **Mempool**: Gerencia transações pendentes antes de serem confirmadas na blockchain.

## Instalação

Para rodar o projeto localmente, clone o repositório e execute:

```bash
cargo run
