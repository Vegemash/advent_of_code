namespace Lib;

using System.Text.RegularExpressions;

public static class LibC
{
    public static string Part1(string input)
    {
        Int32 mul = 0;

        string pattern = @"mul\((\d*),(\d*)\)";
        MatchCollection matches = Regex.Matches(input, pattern);
        foreach (Match match in matches)
        {
            /* Console.Write($"match={match.ToString()} "); */
            /* foreach(Group group in match.Groups) */
            /* { */

            /*   Console.Write($"'{group.ToString()}' "); */
            /* } */
            /* Console.Write($"\n"); */
            mul +=
                Int32.Parse(match.Groups[1].ToString()) * Int32.Parse(match.Groups[2].ToString());
        }

        return mul.ToString();
    }

    public static string Part2(string input)
    {
        Int32 mul = 0;
        bool enabled = true;

        string pattern = @"do\(\)|don't\(\)|mul\((\d*),(\d*)\)";
        MatchCollection matches = Regex.Matches(input, pattern);
        foreach (Match match in matches)
        {
            /* Console.Write($"match={match.ToString()} "); */
            /* foreach (Group group in match.Groups) */
            /* { */
            /*     Console.Write($"'{group.ToString()}' "); */
            /* } */
            /* Console.Write($"\n"); */
            if (match.Groups[0].ToString() == "do()")
            {
                enabled = true;
            }
            else if (match.Groups[0].ToString() == "don't()")
            {
                enabled = false;
            }
            else if (enabled)
            {
                mul +=
                    Int32.Parse(match.Groups[1].ToString())
                    * Int32.Parse(match.Groups[2].ToString());
            }
        }

        return mul.ToString();
    }
}
