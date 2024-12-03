EXEC_DIR = ./target/release
EXEC = computorv1

.PHONY: all
all:
	cargo build --release
	cp $(EXEC_DIR)/$(EXEC) .

.PHONY: clean
clean :
	@cargo clean --release

.PHONY: fclean
fclean :
	@cargo clean

.PHONY: re
re : fclean all
