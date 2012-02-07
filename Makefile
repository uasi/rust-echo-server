.PHONY: all clean client server

all: client server

clean:
	rm -rf client client.dSYM
	rm -rf server server.dSYM

client:
	rustc client.rc

server:
	rustc server.rc

