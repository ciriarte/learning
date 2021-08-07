## Better defaults for make (thanks https://tech.davis-hansson.com/p/make/)
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

SRC_DIR := src
OBJ_DIR := obj
BIN_DIR := bin
EXE := clox

CC := cc
COMMONFLAGS := -c -g -Wall -Wextra -O2
CFLAGS := $(COMMONFLAGS) -I$(SRC_DIR)/ -std=c17
LDFLAGS := -g

SRCS := $(wildcard $(SRC_DIR)/*.c)
OBJS := $(subst $(SRC_DIR)/,$(OBJ_DIR)/,$(SRCS:.c=.o))

.PHONY: all clean distclean

all: $(BIN_DIR)/$(EXE)

$(BIN_DIR)/$(EXE): $(OBJS)
		mkdir -p $(BIN_DIR)
		$(CC) $(LDFLAGS) $? -o $@

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
		mkdir -p $(OBJ_DIR)
		$(CC) $(CFLAGS) $< -o $@

clean:
		rm -rf $(OBJ_DIR)/ $(BIN_DIR)/
