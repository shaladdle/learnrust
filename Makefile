default:
	rustc -O main.rs -o echoserver
clean:
	rm -r echoserver echoserver.dSYM/
