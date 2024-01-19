CC=gcc
CFLAGS=-Wall -g

test: tests/test_bowling_game.c src/game.c
	$(CC) $(CFLAGS) -o test_bowling_game tests/test_bowling_game.c src/game.c -lcunit
	./test_bowling_game

clean:
	rm -f test_bowling_game
