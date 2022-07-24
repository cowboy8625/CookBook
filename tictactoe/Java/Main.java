import java.util.Scanner;

class Main
{
    public static void main(String[] args)
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
            System.out.format("%s is the winner!", player);
        }
        else if (!isFreeSlots(spots))
        {
            System.out.println("Draw this round!");
        }
        else
        {
            System.out.println("ERROR");
        }
    }

    static boolean isFreeSlots(Player[] spots)
    {
        for (int i = 0; i < spots.length; i++)
        {
            if (spots[i] == Player.E) return true;
        }
        return false;
    }

    static boolean isFree(Player[] spots, int loc)
    {
        if (spots[loc] == Player.E)
        {
            return true;
        }
        return false;
    }

    static int input(String msg)
    {
        int result = 100;
        do
        {
            System.out.print(msg);
            result = Integer.valueOf(System.console().readLine());
        } while (result < 0 && result > 10);
        return result - 1;
    }

    static void place(Player[] s, int n, Player sign)
    {
        s[n] = sign;
    }

    static void clear()
    {
        System.out.println("\033[2J\033[1;1H");
    }

    static void render(Player[] s, Player sign)
    {
        clear();
        System.out.printf("   |   |   Player %s turn\n", sign);
        System.out.printf(" %s | %s | %s \n", s[0], s[1], s[2]);
        System.out.println("   |   |   ");
        System.out.println("-----------");
        System.out.println("   |   |   ");
        System.out.printf(" %s | %s | %s \n", s[3], s[4], s[5]);
        System.out.println("   |   |   ");
        System.out.println("-----------");
        System.out.println("   |   |   ");
        System.out.printf(" %s | %s | %s \n", s[6], s[7], s[8]);
        System.out.println("   |   |   ");
    }

    static boolean isWin(Player[] s)
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

enum Player
{
    O {
        public String toString() {
            return "O";
        }
    },
    X {
        public String toString() {
            return "X";
        }
    },
    E {
        public String toString() {
            return " ";
        }
    }
}
