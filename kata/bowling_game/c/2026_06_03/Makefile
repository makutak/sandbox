CC=gcc
CFLAGS=-Wall -g

test: test_bowling_game.c bowling_game.c
	$(CC) $(CFLAGS) -o test_bowling_game test_bowling_game.c bowling_game.c -lcunit
	./test_bowling_game

clean:
	rm -f test_bowling_game
