# jdn-squire

*Squire Solutions Candidate Assignment*

## Run the Server

* Clone this repository to a local directory.
* Execute `cargo build --release` in the local directory.
* Execute `./target/release/jdn-squire` to start the server. Note: the application expects to bind to `0.0.0.0:1042`

## Test with curl

### Challenge 1

* Execute `curl 127.0.0.1:1042/hello`. Observe the output `Hello World!`
* Execute `curl -X POST 127.0.0.1:1042/next`. Observe the output `1`. This operation can be repeated to obtain sequential Fibonacci numbers until the next number is greater than [u128::MAX](https://doc.rust-lang.org/std/primitive.u128.html#associatedconstant.MAX)

### Challenge 2 (with authentication)

* Execute `echo "" > cookies` to ensure the cookies file exists and is empty
* Execute `curl 127.0.0.1:1042/users -b cookies`. Observe that there is no output, and that the server denies the request
* Execute `curl -d '{"username": "tester", "password": "invalid"}' -H 'Content-Type: application/json' 127.0.0.1:1042/login -c cookies`. Observe the output `invalid credentials`
* Execute `curl -d '{"username": "tester", "password": "Squ!r3"}' -H 'Content-Type: application/json' 127.0.0.1:1042/login -c cookies`. Observe that the server finds a user with the ID of `1`
* Execute `curl 127.0.0.1:1042/users -b cookies`. Observe the output `[]` indicating there are currently no users
* Execute `curl -d '{"id": "test", "name": "John", "age": 42}' -H 'Content-Type: application/json' 127.0.01:1042/users -b cookies`. Observe that the server returns an OK status code
* Execute `curl 127.0.0.1:1042/users -b cookies`. Observe that the created user is displayed
* Execute `curl -d '{"id": "test", "name": "John", "age": 32}' -H 'Content-Type: application/json' 127.0.01:1042/users -b cookies`. Observe that the server returns an OK status code
* Execute `curl 127.0.0.1:1042/user/test -b cookies`. Observe that the user age has been updated
* Feel free to repeat with different values
