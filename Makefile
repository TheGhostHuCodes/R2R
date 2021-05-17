PHONY: build format

build:
	clang calculator.c -o calculator -L target/debug/ -l calculate

format:
	clang-format -i calculator.c
