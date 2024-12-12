using Lib;

namespace Test;

using System;
using System.IO;

[TestClass]
public class UnitTest1
{
    [TestMethod]
    public void TestMethod1()
    {
        // This will get the current Test directory
        string TestDir =
            Directory.GetParent(Directory.GetCurrentDirectory())?.Parent?.Parent?.FullName
            ?? throw new FileNotFoundException();

        Assert.AreEqual("161", LibC.Part1(File.ReadAllText(Path.Join(TestDir, "input.txt"))));
    }

    [TestMethod]
    public void TestMethod2()
    {
        // This will get the current Test directory
        string TestDir =
            Directory.GetParent(Directory.GetCurrentDirectory())?.Parent?.Parent?.FullName
            ?? throw new FileNotFoundException();

        Assert.AreEqual("48", LibC.Part2(File.ReadAllText(Path.Join(TestDir, "input2.txt"))));
    }
}
