
# Pytest Markdown Report

Este projeto gera um relatório Markdown com base na execução do pytest. Desenvolvido em rust o projeto também conta com a possibilidade de customização.

## Demonstração de uso

### Automático

O primeiro caso de uso é deixar a ferramenta no automático

as configurações neste caso ficam da seguinte forma

key | value
--- | ---
output | ./report.md
no-auto | false
target-folder | ./
config-file | does not exist
passed | ✅
failed | ❌
skipped | ⚠️

Uso:

```bash
pytest_markdown_report
```

### Modo Automático Desligado

Neste modo o programa não irá executar o pytest automaticamente,
então você precisa passar a saída do pytest para o programa.

Quando este modo é ativo a configuração de no-auto deve ser true.

key | value
--- | ---
no-auto | true

Uso:

```bash
pytest -v | pytest_markdown_report --no-auto
```

Ou:

```bash
pytest -v > report.txt
cat report.txt | pytest_markdown_report --no-auto
```

Note que o pytest deve ser chamado com a flag -v para o programa funcionar

### Passando vários argumentos

Você pode utilizar várias configurações juntas.

Execute pytest_markdown_report ```-h``` ou ```--help``` para obter ajuda:

```bash
pytest_markdown_report -h
```

saída:

```text
pytest markdown report

Usage: pytest_markdown_report [OPTIONS]

Options:
  -o, --output <OUTPUT>                [default: report.md]
      --no-auto                        Turn off auto run command pytest
  -t, --target-folder <TARGET_FOLDER>  directory where the tests are [default: ./]
  -c, --config-file <CONFIG_FILE>      set a config file [default: ]
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

Exemplo de uso:

```bash
pytest_markdown_report -c src/examples/config.txt
```

## Documentação do arquivo de configurações

Este é um arquivo opcional que pode ser passado com a flag ```-c```, vale lembrar que o tipo do arquivo não importa desde que ele siga as regras mostradas a baixo.

A estrutura básica é: ```prefix name = value```.

Você pode encontrar um exemplo de configuração [aqui](https://raw.githubusercontent.com/walefy/pytest-markdown-report/main/src/examples/config.txt)

### Prefixos

tag | description
--- | ---
e | tag utilizada para modificar os emojis padrões
c | tag utilizada para modificar algumas configurações da ferramenta
h | tag utilizada para adicionar uma seção header no report.md
f | tag utilizada para adicionar uma seção footer no report.md

### Nomes permitidos na tag ```e```

name | description
--- | ---
passed | emoji para quando o teste passa
failed | emoji para quando o teste falha
skipped | emoji para quando o teste é pulado

### Nomes permitidos na tag ```c```

name | description
--- | ---
output | caminho para o arquivo de saída. Ex: ```./report.md```
target-folder | caminho onde estão os testes do pytest

### Tags de seção ```h``` e ```f```

A tag ```h``` que declara a seção header e a tag ```f``` que declara a seção footer tem nomes anônimos.

Exemplo:

```text
h = # Titulo
h =
h = Acima tem uma linha vazia
h = tanto a tag h quanto a tag f aceitam markdown assim:
h =
h = ``` bash
h = echo "Oiii"
h = ```
```

## Rodando localmente

Clone o projeto

```bash
git clone git@github.com:walefy/pytest-markdown-report.git
```

Entre no diretório do projeto

```bash
cd pytest-markdown-report
```

Rode a versão de desenvolvimento

```bash
cargo run
```

Ou a versão otimizada

```bash
cargo run --release
```

Passando flags

```bash
cargo run --release -- -c ./config.txt
```

## Instalação

Instale Pytest Markdown Report

Você pode simplismente baixar o executável para linux em releases ou buildar o projeto no seu computador como mostra a baixo.

```bash
cargo build --release
cp ./target/release/pytest_markdown_report caminho_destino
```

Desta forma você vai ter o executável no caminho de destino
e pode executa-lo com ```./pytest_markdown_report```

Se quiser executar em qualquer lugar do seu pc você pode copiar para o /bin, mas isso só vai funcionar em sistemas baseados no Linux.

```bash
cargo build --release
cp ./target/release/pytest_markdown_report /bin
```

## Autores

- [@walefy](https://www.github.com/walefy)
