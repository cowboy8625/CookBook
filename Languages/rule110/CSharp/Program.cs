class Program {
  const int SIZE = 70;
  static void Main(string[] args) {
    List<bool> Grid = new List<bool>();
    for(var i = 0; i < SIZE; i++) {
      if (i == SIZE-1) {
        Grid.Add(true);
        continue;
      }
      Grid.Add(false);
    }


    while (true) {
      Display(Grid);
      Grid = next_gen(Grid);
      System.Threading.Thread.Sleep(100);
    }
  }

  static List<bool> next_gen(List<bool> last_gen) {
    List<bool> next = new List<bool>();
    int len = last_gen.Count();
    for(var i = 0; i < len; i++) {
      bool a = last_gen[((i - 1) + len) % len];
      bool b = last_gen[i];
      bool c = last_gen[((i + 1) + len) % len];
      next.Add(is_alive(a, b, c));
    }
    return next;
  }

  static bool is_alive(bool a, bool b, bool c) {
    switch ((a, b, c)) {
      case (true, true, true): // 7
        return false;
      case (true, true, false): // 6
        return true;
      case (true, false, true): // 5
        return true;
      case (true, false, false): // 4
        return false;
      case (false, true, true): // 3
        return true;
      case (false, true, false): // 2
        return true;
      case (false, false, true): // 1
        return true;
      case (false, false, false): // 0
        return false;
    }
  }

  static void Display(List<bool> grid) {
    for(var i = 0; i < grid.Count(); i++) {
      char l = grid[i] ? '#' : ' ';
      System.Console.Write(l);
    }
    System.Console.WriteLine();
  }
}
