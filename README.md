# rust-healthcheck-and-reconnect-sgad

**Languages:** [English](#english) · [Português (Brasil)](#português-brasil)

---

## English

Rust utility that runs an **Oracle**-backed **health check** and, when needed, **reconciles** connectivity state by reading and updating a row in `CORE.cosetting`.

It was written as an alternative to the **Go** and **Python** variants: to avoid newer **Go** toolchain telemetry friction and to get more predictable performance than **Python** (including 3.12.x) for this workload.

### What is SGAD here?

In the **TOSP** (terminal operating system) stack used in port operations, **SGAD** names an operational subsystem whose **connection health** is often mirrored as configuration in **Oracle** (`CORE.cosetting`). This binary checks the target row; if the value is not `TRUE`, it runs the corresponding `UPDATE` and `COMMIT` so the flag reflects a healthy/reconnected state.

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

### Run

The password is passed as the **first CLI argument**. The **user**, **connect descriptor** (host/port/service), and **SQL** are still embedded in `src/main.rs` in this prototype—replace them for your environment, or fork to read `ORACLE_*` environment variables before production use.

```bash
./target/release/rust-healthcheck-and-reconnect-sgad '<database_password>'
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

No cenário **TOSP** (sistema operacional de terminal portuário), **SGAD** identifica um subsistema operacional cujo **estado de conectividade** costuma ser refletido em configuração no **Oracle** (`CORE.cosetting`). Este binário consulta o registro configurado no código; se o valor não estiver em `TRUE`, executa o `UPDATE` e o `COMMIT` para restabelecer o estado esperado (fluxo “verificar + reconectar” materializado em dado de configuração).

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

### Executar

A **senha** do banco é passada como **primeiro argumento** na linha de comando. **Usuário**, **string de conexão** (host/porta/serviço) e **SQL** ainda estão fixos em `src/main.rs` neste protótipo—substitua pelo seu ambiente ou evolua o código para ler variáveis `ORACLE_*` antes de uso em produção.

```bash
./target/release/rust-healthcheck-and-reconnect-sgad '<senha_do_banco>'
```

### libc e portabilidade

Devido a **diferenças importantes** nas versões de **`libc`** e bibliotecas disponíveis no SO, pode ser necessário **recompilar o binário na máquina destino**, principalmente em Linux mais antigo (por exemplo **RHEL 7.9**).

### Repositórios relacionados

- [python3-healthcheck-and-reconnect-sgad](https://github.com/FabioLeitao/python3-healthcheck-and-reconnect-sgad)
- [go-healthcheck-and-reconnect-sgat](https://github.com/FabioLeitao/go-healthcheck-and-reconnect-sgat)
