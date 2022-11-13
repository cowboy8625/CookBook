enum Player
{
    O = 'O',
    X = 'X',
    E = ' ',
}

namespace Program
{
    class Program
    {
        static void Main(string[] args)
        {
            Player[] spots = {
                Player.E,
                Player.E,
                Player.E,
                Player.E,
                Player.E,
                Player.E,
                Player.E,
                Player.E,
                Player.E
            };
            int loc;
            Player player = Player.O;

            do
            {
                player = (player == Player.X) ? Player.O : Player.X;
                render(spots, player);

                do
                {
                    loc = input("Input location 1-9:> ");
                } while (!isFree(spots, loc));
                place(spots, loc, player);

            } while (isFreeSlots(spots) && !isWin(spots));

            render(spots, player);
            if (isWin(spots))
            {
                System.Console.WriteLine("{0} is the winner!", player);
            }
            else if (!isFreeSlots(spots))
            {
                System.Console.WriteLine("Draw this round!");
            }
            else
            {
                System.Console.WriteLine("ERROR");
            }
        }

        static bool isFreeSlots(Player[] spots)
        {
            for (int i = 0; i < spots.Length; i++)
            {
                if (spots[i] == Player.E) return true;
            }
            return false;
        }

        static bool isFree(Player[] spots, int loc)
        {
            if (spots[loc] == Player.E)
            {
                return true;
            }
            return false;
        }

        static int input(string msg)
        {
            int result = 100;
            do
            {
                System.Console.Write(msg);
                if (int.TryParse(System.Console.ReadLine(), out result))
                {
                    System.Console.WriteLine(result);
                }
            } while (result < 0 && result > 10);
            return result - 1;
        }

        static void place(Player[] s, int n, Player sign)
        {
            s[n] = sign;
        }

        static void clear()
        {
            System.Console.WriteLine("\x1b[2J\x1b[1;1H");
        }

        static void render(Player[] s, Player sign)
        {
            clear();
            System.Console.WriteLine("   |   |   Player {0} turn", sign);
            System.Console.WriteLine(" {0} | {1} | {2} ", (char)s[0], (char)s[1], (char)s[2]);
            System.Console.WriteLine("   |   |   ");
            System.Console.WriteLine("-----------");
            System.Console.WriteLine("   |   |   ");
            System.Console.WriteLine(" {0} | {1} | {2} ", (char)s[3], (char)s[4], (char)s[5]);
            System.Console.WriteLine("   |   |   ");
            System.Console.WriteLine("-----------");
            System.Console.WriteLine("   |   |   ");
            System.Console.WriteLine(" {0} | {1} | {2} ", (char)s[6], (char)s[7], (char)s[8]);
            System.Console.WriteLine("   |   |   ");
        }

        static bool isWin(Player[] s)
        {
            return
              (s[0] == s[1]) && (s[1] == s[2]) && (s[0] != Player.E) ||
              (s[3] == s[4]) && (s[4] == s[5]) && (s[3] != Player.E) ||
              (s[6] == s[7]) && (s[7] == s[8]) && (s[6] != Player.E) ||
              (s[0] == s[3]) && (s[3] == s[6]) && (s[3] != Player.E) ||
              (s[1] == s[4]) && (s[4] == s[7]) && (s[1] != Player.E) ||
              (s[2] == s[5]) && (s[5] == s[8]) && (s[2] != Player.E) ||
              (s[0] == s[4]) && (s[4] == s[8]) && (s[0] != Player.E) ||
              (s[2] == s[4]) && (s[4] == s[6]) && (s[2] != Player.E);
        }
    }
}
