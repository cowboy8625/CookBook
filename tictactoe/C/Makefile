CC=gcc
exec = tictactoe.out
sources = $(wildcard src/*.c)
objects = $(sources:.c=.o)
flags = -g


$(exec): $(objects)
	$(CC) $(objects) $(flags) -o $(exec)

%.o: %.c include/%.h
	$(CC) -c $(flags) $< -o $@

run:
	make
	./$(exec)

clean:
	-rm *.out
	-rm *.o
	-rm src/*.o
