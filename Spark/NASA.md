**HTTP requests to the NASA Kennedy Space Center WWW server**

Fonte oficial do dateset: http://ita.ee.lbl.gov/html/contrib/NASA-HTTP.html

**Dados:**

- Jul 01 to Jul 31, ASCII format, 20.7 MB gzip compressed, 205.2 MB.
- Aug 04 to Aug 31, ASCII format, 21.8 MB gzip compressed, 167.8 MB.

**Sobre o dataset**: Esses dois conjuntos de dados possuem todas as requisições HTTP para o servidor da NASA Kennedy Space Center WWW na Flórida para um período específico.

Os logs estão em arquivos ASCII com uma linha por requisição com as seguintes colunas:

- **Host fazendo a requisição**. Um hostname quando possível, caso contrário o endereço de internet se o nome não puder ser identificado.
- **Timestamp** no formato [DIA/MÊS/ANO:HH:MM:SS YYYY] onde YYYY é o fuso horário. 
- **Requisição**
- **Código do retorno HTTP**
- **Total de bytes retornados**



### Use Spark para responder:

```
Quantos hosts únicos acessaram o site?
```

```
Quantos erros "404" ocorreram?
```

```
Quais foram os 10 URLs que mais responderam "404"?
```

```
Quantos 404 ocorreram em cada dia? E no total?
```

```
Qual o total de bytes retornados por dia? E no total?
```
