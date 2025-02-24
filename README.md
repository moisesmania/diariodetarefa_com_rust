# Diário de Tarefas

Este é um aplicativo simples de Diário de Tarefas, desenvolvido com o **Rust** e o **Rocket framework**. O objetivo do projeto é permitir aos usuários adicionar, editar, visualizar e excluir tarefas de forma fácil e intuitiva. O banco de dados utilizado é o **MySQL**, e a interface do usuário foi construída utilizando **HTML** e **CSS**. O projeto é ideal para aprender sobre desenvolvimento web com Rust e integração com banco de dados.

## Tecnologias Utilizadas

- **Frontend**: HTML, CSS
- **Backend**: Rust, Rocket
- **Banco de Dados**: MySQL

## Funcionalidades

- **Adicionar Tarefa**: Permite adicionar uma nova tarefa com título e descrição.
- **Ver Tarefas**: Exibe todas as tarefas cadastradas.
- **Editar Tarefa**: Permite editar o título e a descrição de uma tarefa.
- **Excluir Tarefa**: Permite excluir uma tarefa existente.

## Imagens da Interface

Aqui estão duas capturas da interface do aplicativo:

### 1. Tela Inicial

![Tela Inicial do Diário de Tarefas](assets/interface1.png)

### 2. Tela de Tarefas Cadastradas

![Tela de Tarefas Cadastradas](assets/interface2.png)

## Como Rodar

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) instalado.
- [MySQL](https://dev.mysql.com/downloads/) instalado e configurado.
- [Rocket](https://rocket.rs/) instalado.

### Passos

1. Clone este repositório:
    ```bash
    git clone <URL do repositório>
    ```

2. Instale as dependências do projeto:
    ```bash
    cargo build
    ```

3. Configure o banco de dados:
    - Crie o banco de dados `diario_tarefas` no MySQL.
    - Adicione a tabela `tarefas` com os campos `id`, `titulo` e `descricao`.

    Exemplo de script SQL:
    ```sql
    CREATE DATABASE diario_tarefas;
    USE diario_tarefas;

    CREATE TABLE tarefas (
        id INT AUTO_INCREMENT PRIMARY KEY,
        titulo VARCHAR(255) NOT NULL,
        descricao TEXT NOT NULL
    );
    ```

4. Inicie o servidor:
    ```bash
    cargo run
    ```

5. Acesse o aplicativo em [http://localhost:8000](http://localhost:8000).


**Obs:** Pesquise sobre a estrutura de diretórios e arquivos em projetos Rust. Isso o ajudará a organizar melhor seu projeto e seguir boas práticas no desenvolvimento.

## Contribuindo

Sinta-se à vontade para abrir *issues* ou *pull requests*.

## Licença

Este projeto está licenciado sob a MIT License - veja o arquivo [LICENSE](LICENSE) para mais detalhes.




