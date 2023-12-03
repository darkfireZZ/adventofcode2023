
ARGS = -O --edition 2021
ARGS += -C debug-assertions

BIN_NAME = solution

$(BIN_NAME): main.rs
	rustc $(ARGS) -o $(BIN_NAME) main.rs

.PHONY: clean
clean:
	rm -f $(BIN_NAME)
