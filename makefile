
all: compile
	./target/debug/virtual_synthesizer.exe



clean:
	rm -rf resources/basic/*


.PHONY: clean