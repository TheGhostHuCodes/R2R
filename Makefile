PHONY: build format

build:
	clang calculator.c -o calculator

format:
	clang-format -i calculator.c
