// See https://aka.ms/new-console-template for more information
using System.IO;
using Lib;

string AppDir = Directory.GetCurrentDirectory() ?? throw new FileNotFoundException();


string input = File.ReadAllText(Path.Join(AppDir, "App/input.txt"));
Console.WriteLine("Part 1: " + LibC.Part1( input));
Console.WriteLine("Part 2: " + LibC.Part2( input));
