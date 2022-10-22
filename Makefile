all: jav py rb c cpp ts golang rack hask
jav:
	java App.java "from JAVA JAVA JAVA"
py:
	python3 app.py
rb:
	ruby app.rb
c:
	gcc app.c -o hello_c
	./hello_c
cpp:
	clang++ app.cpp -o hello_cpp
	./hello_cpp
ts:
	tsc app.ts
	node app.js

golang:
	go build -o hellogolang app.go
	./hellogolang
rack:
	racket app.rkt
hask:
	ghc -o app_haskell app.hs
	./app_haskell
ocam:
	ocamlc -o app_ocaml app.ml
	./app_ocaml
