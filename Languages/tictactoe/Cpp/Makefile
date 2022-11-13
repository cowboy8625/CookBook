CC=g++
exec = tictactoe.out
sources = $(wildcard src/*.cpp)
objects = $(sources:.c=.o)
flags = -Wall


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
