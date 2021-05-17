# http-calculator-rs

This project contains an NGINX module written in C that calls out to a library
implemented in Rust.

To compile the module, download the NGINX source in the same directory as this
README file:

```shell
$ curl -L -o nginx-1.19.3.tar.gz https://nginx.org/download/nginx-1.19.3.tar.gz
$ tar -xf nginx-1.19.3.tar.gz
```

then, configure the NGINX project and build it, pointing to the module
directory.

```shell
$ cd nginx-1.19.3
$ ./configure --add-dynamic-module=../module
$ make -j16 build modules
```