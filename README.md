# rust-healthcheck-and-reconnect-sgad
rust-healthcheck-and-reconnect-sgad

Tentando recriar o healthcheck da conectividade com SGAD em Rust para não ter de me preocupar com a telemetria nova da Google se estivermos usando em Go ou a performance do Python (mesmo usando 3.12.1).

Requer instalação do driver Instant Client 19 (https://www.oracle.com/database/technologies/instant-client/downloads.html) do OracleDB, que funcionou sem dificuldades no Windows, mas infelizmente por ser distribuido em rpm, não pude instalar facilmente em Debian 12.

A versão compilada do código Rust ainda exigiu libc.so razoavelmente atualizada, não sendo compativel com OralceLinux ou RHEL 7.9 (atualmente em PRD)
