namespace Lib;

using Grid = Dictionary<(int, int), char>;

public static class LibC
{
    public static string Part1(string input)
    {
        int count = 0;
        Grid grid = new Grid();
        string[] lines = input.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        {
            int y = 0;
            foreach (string line in lines)
            {
                if (!String.IsNullOrEmpty(line))
                {
                    int x = 0;
                    foreach (char ch in line)
                    {
                        grid[(x, y)] = ch;
                        x++;
                    }
                }
                y++;
            }
        }
        var dirs = new List<(int, int)>();
        for (int x = -1; x < 2; x++)
        {
            for (int y = -1; y < 2; y++)
            {
                if (x == 0 && y == 0)
                {
                    continue;
                }
                dirs.Add((x, y));
            }
        }

        for (int y = 0; y < lines.Length; y++)
        {
            for (int x = 0; x < lines[0].Length; x++)
            {
                if (grid[(x, y)] == 'X')
                {
                    foreach ((int, int) dir in dirs)
                    {
                        if (HasXMAS(grid, lines[0].Length, lines.Length, (x, y), dir))
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        return count.ToString();
    }

    private static bool HasXMAS(Grid grid, int maxx, int maxy, (int, int) start, (int, int) dir)
    {
        var pos = start;
        pos = (start.Item1 + dir.Item1, start.Item2 + dir.Item2);
        if (!InBounds(maxx, maxy, pos) || grid[pos] != 'M')
        {
            return false;
        }
        pos = (pos.Item1 + dir.Item1, pos.Item2 + dir.Item2);
        if (!InBounds(maxx, maxy, pos) || grid[pos] != 'A')
        {
            return false;
        }
        pos = (pos.Item1 + dir.Item1, pos.Item2 + dir.Item2);

        return InBounds(maxx, maxy, pos) && grid[pos] == 'S';
    }

    private static bool InBounds(int maxx, int maxy, (int, int) point)
    {
        return point.Item1 >= 0 && point.Item1 < maxx && point.Item2 >= 0 && point.Item2 < maxy;
    }

    public static string Part2(string input)
    {
        int count = 0;
        Grid grid = new Grid();
        string[] lines = input.Split("\n", StringSplitOptions.RemoveEmptyEntries);

        {
            int y = 0;
            foreach (string line in lines)
            {
                if (!String.IsNullOrEmpty(line))
                {
                    int x = 0;
                    foreach (char ch in line)
                    {
                        grid[(x, y)] = ch;
                        x++;
                    }
                }
                y++;
            }
        }

        for (int y = 0; y < lines.Length; y++)
        {
            for (int x = 0; x < lines[0].Length; x++)
            {
                if (grid[(x, y)] == 'A')
                {
                    if (HasMASX(grid, lines[0].Length, lines.Length, (x, y)))
                    {
                        count += 1;
                    }
                }
            }
        }
        return count.ToString();
    }

    private static bool HasMASX(Grid grid, int maxx, int maxy, (int, int) start)
    {
        char tl,
            bl,
            tr,
            br;
        try
        {
            tl = grid[(start.Item1 - 1, start.Item2 - 1)];
            br = grid[(start.Item1 + 1, start.Item2 + 1)];

            tr = grid[(start.Item1 + 1, start.Item2 - 1)];
            bl = grid[(start.Item1 - 1, start.Item2 + 1)];
        }
        catch (Exception ex)
        {
            return false;
        }

        if (!("MS".Contains(tl) && "MS".Contains(br) && tl != br))
        {
            return false;
        }
        return ("MS".Contains(bl) && "MS".Contains(tr) && bl != tr);
    }
}
