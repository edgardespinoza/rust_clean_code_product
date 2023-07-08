
## Architecture hexagonal - Clean Code 


  

[![](https://www.planttext.com/api/plantuml/svg/RP71Ri8m38RlVWgFU_i82127dNUjAkIcfaHBaoXn1DFKTyUKjHlIV4XY-y_tEyek2oKfeuCtkV38sBd2CLal021M7ED7oO9x8_Q6L-wmmdqo5ojif2J6TxxH43mv01zwncz0ZKZcamR6ZVNdH5bIcTKRf8psHuFOn91bI9nFYVLe9QIlzY5PwhDkmgSSrcMb7yD8ri_KVgKgjsG-6ShSOhN5vYyS_nFLDJQZ3k_Qf-6h2SNeh26nGPqqrkafsWKumRnf1Qi-NBtQBlYvyJi2LexFelsNDou9re_RzHQF1u09UXBgbA6fsgh9ZdsldtK7
)](https://www.planttext.com/api/plantuml/svg/RP71Ri8m38RlVWgFU_i82127dNUjAkIcfaHBaoXn1DFKTyUKjHlIV4XY-y_tEyek2oKfeuCtkV38sBd2CLal021M7ED7oO9x8_Q6L-wmmdqo5ojif2J6TxxH43mv01zwncz0ZKZcamR6ZVNdH5bIcTKRf8psHuFOn91bI9nFYVLe9QIlzY5PwhDkmgSSrcMb7yD8ri_KVgKgjsG-6ShSOhN5vYyS_nFLDJQZ3k_Qf-6h2SNeh26nGPqqrkafsWKumRnf1Qi-NBtQBlYvyJi2LexFelsNDou9re_RzHQF1u09UXBgbA6fsgh9ZdsldtK7 
)


## vertical slicing and screaming
``` 
├── app
│   └── src
│       ├── application
│       │   ├── add_product.rs
│       │   ├── openapi.rs
│       │   └── router.rs
│       ├── domain
│       │   ├── model
│       │   │   ├── product.rs
│       │   │   └── value_object.rs
│       │   ├── repository.rs
│       │   └── use_case
│       │       └── add_product.rs
│       ├── infrastructure
│       │   ├── model.rs
│       │   └── postgres_repository.rs
│       ├── lib.rs
│       ├── main.rs
│       └── state.rs
├── config
│   └── src
│       └── lib.rs
├── error
│   └── src
│       └── lib.rs
└── sql
    └── database.sql

```

try use this

1. change la variable in the file .env ``DATABASE_URL``
2. `cargo run`
3. use the curl command
    ```
    curl -i -X 'POST' \
    'http://127.0.0.1:8080/api/product' \
    -H 'accept: application/json' \
    -H 'Content-Type: application/json' \
    -d '{
        "name": "product_1",
        "price": 2.0,
        "stock": 3}'
    ```