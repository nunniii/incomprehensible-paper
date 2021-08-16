
<img src="https://doc.rust-lang.org/book/img/ferris/not_desired_behavior.svg" width="300" align="right">

# incomprehensible paper



## ⚠️ This code is under construction, so it still doesn't produce the desired behavior.



Você pode salvar as sementes de sua carteira ou qualquer senha de maneira segura em um papel  caso não queira armazená-las na internet ou em seus dispositivos.

## Uso

incp [action]


- [ new-key <qnt*>] Gerar chave; ex.:
```
incp new-key 4
```
É necessário passar a quantidade de palavras que a semente contém.
- [ code <seed> -k <key>] Codificar seed com chave já existente; ex.:

```
incp code "iden print word cube" -k "3412"
```
Caso não passe o argumento `<k>`, uma chave será criada.

- [ decode ] Decodificar seed. ex.:

```
incp decode "word cube iden print" -k "3412"
```
Neste caso, a chave é obrigatória.


o argumento `<-k>` indica ao programa que uma chave será passada; *`<qnt>` é a quantidade de palavras existentes na seed.

## Documentações para acompanhamento

* https://doc.rust-lang.org/stable/std/env/fn.args.html

* https://docs.rs/rand/0.8.4/rand/
