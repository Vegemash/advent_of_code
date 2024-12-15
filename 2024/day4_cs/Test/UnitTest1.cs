using Lib;

namespace Test;

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

        Assert.AreEqual("18", LibC.Part1(File.ReadAllText(Path.Join(TestDir, "input.txt"))));
    }

    [TestMethod]
    /* [Ignore] */
    public void TestMethod2()
    {
        // This will get the current Test directory
        string TestDir =
            Directory.GetParent(Directory.GetCurrentDirectory())?.Parent?.Parent?.FullName
            ?? throw new FileNotFoundException();

        Assert.AreEqual("9", LibC.Part2(File.ReadAllText(Path.Join(TestDir, "input2.txt"))));
    }
}
