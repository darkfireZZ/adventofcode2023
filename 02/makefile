
ARGS = -O --edition 2021

BIN_NAME = solution

$(BIN_NAME): main.rs
	rustc $(ARGS) -o $(BIN_NAME) main.rs

.PHONY: clean
clean:
	rm $(BIN_NAME)
