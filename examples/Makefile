.PHONY: run-sax-example
run-sax-example: sax-example
	./sax-example ../test/messagepack/data-1-2.mp

sax-example: Makefile sax-example.c sax-example.h $(shell echo ../reference-c/*)
	cc -g -Wall -Werror -DMPACK_EXTENSIONS=1 sax-example.c ../reference-c/mpack/*.c -I../reference-c -o sax-example

clean:
	rm -f sax-example
