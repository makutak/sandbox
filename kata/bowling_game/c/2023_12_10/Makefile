# Makefile for Bowling Game Kata

CC=gcc
CFLAGS=-Wall -g -Isrc -Itests
LDFLAGS=-lcunit

# ソースファイルとオブジェクトファイル
SRC=src/bowling_game.c
OBJ=$(SRC:.c=.o)
TEST_SRC=tests/test_bowling_game.c
TEST_OBJ=$(TEST_SRC:.c=.o)
TEST_EXE=bowling_game_test

# 'all' ターゲットはテストをビルドします
all: $(TEST_EXE)

$(TEST_EXE): $(OBJ) $(TEST_OBJ)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

# オブジェクトファイルの生成規則
%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

# テストの実行
test: $(TEST_EXE)
	./$(TEST_EXE)

# クリーンアップ
clean:
	rm -f src/*.o tests/*.o $(TEST_EXE)

.PHONY: all test clean
