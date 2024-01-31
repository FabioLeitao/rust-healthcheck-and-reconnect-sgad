# rust-healthcheck-and-reconnect-sgad
rust-healthcheck-and-reconnect-sgad

Tentando recriar o healthcheck da conectividade com SGAD em Rust para não ter de me preocupar com a telemetria nova da Google se estivermos usando em Go ou a performance do Python (mesmo usando 3.12.1).

Requer instalação do driver Instant Client 19 (https://www.oracle.com/database/technologies/instant-client/downloads.html) do OracleDB, que funcionou sem dificuldades no Windows, Debian, e RHEL ou equivalentes.

sudo dnf install oracle-instantclient-release-el9 libnsl libnsl2 libnsl2-devel libaio libaio-devel

Devido a difernças importantes na versão disponiveis no OS das bibliotecas de libc.so pode ser necessário recompilar o binário na máquina destino (principalmente caso seja um Linux muito antigo, como RHEL 7.9)
