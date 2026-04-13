# rust-healthcheck-and-reconnect-sgad

**Languages:** [English](#english) · [Português (Brasil)](#português-brasil)

---

## English

Rust utility that runs an **Oracle**-backed **health check** and, when needed, **reconciles** connectivity state by reading and updating a row in `CORE.cosetting`.

It was written as an alternative to the **Go** and **Python** variants: to avoid newer **Go** toolchain telemetry friction and to get more predictable performance than **Python** (including 3.12.x) for this workload.

### What is SGAD here?

Public context (Rio de Janeiro port ecosystem): **[sgad.portosrio.gov.br](https://sgad.portosrio.gov.br/)**.

In the **TOSP** (terminal operating system) stack, **SGAD** names an operational subsystem whose **connection health** is often mirrored as configuration in **Oracle** (`CORE.cosetting`). This binary checks the target row; if the value is not `TRUE`, it runs the corresponding `UPDATE` and `COMMIT` so the flag reflects a healthy/reconnected state.

If you do not use TOSP, you can still read this repo as a **pattern**: query a settings table and reconcile state from a small, static binary.

### Prerequisites

- **Rust** (2021 edition) and **Cargo**
- **Oracle Instant Client 19** — [Oracle downloads](https://www.oracle.com/database/technologies/instant-client/downloads.html)  
  Validated on **Windows**, **Debian**, and **RHEL**-like systems with matching client libraries.

**Example (RHEL 9 / compatible):**

```bash
sudo dnf install oracle-instantclient-release-el9 libnsl libnsl2 libnsl2-devel libaio libaio-devel
```

### Build

```bash
cargo build --release
```

### Configuration (required)

| Variable | Meaning |
| -------- | ------- |
| `ORACLE_USER` | Database user |
| `ORACLE_CONNECT_STRING` | Oracle Easy Connect, e.g. `host:1521/SERVICE` |
| `ORACLE_PASSWORD` | Optional if you pass the password as the **first CLI argument** instead |
| `COSETTING_ROW_ID` | Optional; defaults to `681` (numeric row in `CORE.cosetting`) |

Copy `.env.example` to `.env` (gitignored) or export variables in your shell. **Do not** commit real hostnames, users, or passwords.

### Run

```bash
export ORACLE_USER='…'
export ORACLE_CONNECT_STRING='host:1521/ORCL'
./target/release/rust-healthcheck-and-reconnect-sgad
# or: ORACLE_PASSWORD='…' ./target/release/rust-healthcheck-and-reconnect-sgad
# or pass password only as argv: ./target/release/rust-healthcheck-and-reconnect-sgad '…'
```

### libc / portability

Because **glibc** and Instant Client builds differ across OS versions, you may need to **rebuild on the target machine**—especially on older Linux (e.g. **RHEL 7.9**).

### Related repositories

- [python3-healthcheck-and-reconnect-sgad](https://github.com/FabioLeitao/python3-healthcheck-and-reconnect-sgad)
- [go-healthcheck-and-reconnect-sgat](https://github.com/FabioLeitao/go-healthcheck-and-reconnect-sgat)

---

## Português (Brasil)

Utilitário em **Rust** que faz **health check** contra **Oracle** e, quando necessário, **reconcilia** o estado de conectividade lendo e atualizando um registro em `CORE.cosetting`.

O objetivo foi ter uma alternativa às versões em **Go** e **Python**: evitar o atrito com a **telemetria** do toolchain Go mais novo e obter desempenho mais previsível do que no **Python** (inclusive 3.12.x) para esse tipo de tarefa.

### O que é o SGAD?

Contexto público (ecossistema portuário do Rio): **[sgad.portosrio.gov.br](https://sgad.portosrio.gov.br/)**.

No cenário **TOSP** (sistema operacional de terminal portuário), **SGAD** identifica um subsistema operacional cujo **estado de conectividade** costuma ser refletido em configuração no **Oracle** (`CORE.cosetting`). Este binário consulta o registro indicado por `COSETTING_ROW_ID`; se o valor não estiver em `TRUE`, executa o `UPDATE` e o `COMMIT` para restabelecer o estado esperado.

Se você não usa TOSP, trate o repositório como **exemplo de padrão**: consultar tabela de parâmetros e corrigir estado com binário pequeno e estático.

### Pré-requisitos

- **Rust** (edition 2021) e **Cargo**
- **Oracle Instant Client 19** — [download Oracle](https://www.oracle.com/database/technologies/instant-client/downloads.html)  
  Testado com sucesso no **Windows**, **Debian** e **RHEL** (ou equivalentes), com bibliotecas alinhadas.

**Exemplo (RHEL 9 ou compatível):**

```bash
sudo dnf install oracle-instantclient-release-el9 libnsl libnsl2 libnsl2-devel libaio libaio-devel
```

### Compilar

```bash
cargo build --release
```

### Configuração (obrigatório)

| Variável | Significado |
| -------- | ----------- |
| `ORACLE_USER` | Usuário do banco |
| `ORACLE_CONNECT_STRING` | Easy Connect, ex.: `host:1521/SERVICO` |
| `ORACLE_PASSWORD` | Opcional se a senha for passada como **primeiro argumento** |
| `COSETTING_ROW_ID` | Opcional; padrão `681` |

Use `.env.example` como modelo. **Não** commite host real, usuário ou senha.

### Executar

```bash
export ORACLE_USER='…'
export ORACLE_CONNECT_STRING='host:1521/ORCL'
./target/release/rust-healthcheck-and-reconnect-sgad
# ou senha por argv: ./target/release/rust-healthcheck-and-reconnect-sgad '…'
```

### libc e portabilidade

Devido a **diferenças importantes** nas versões de **`libc`** e bibliotecas disponíveis no SO, pode ser necessário **recompilar o binário na máquina destino**, principalmente em Linux mais antigo (por exemplo **RHEL 7.9**).

### Repositórios relacionados

- [python3-healthcheck-and-reconnect-sgad](https://github.com/FabioLeitao/python3-healthcheck-and-reconnect-sgad)
- [go-healthcheck-and-reconnect-sgat](https://github.com/FabioLeitao/go-healthcheck-and-reconnect-sgat)
