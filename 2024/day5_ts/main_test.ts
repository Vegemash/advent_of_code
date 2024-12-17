import { assertEquals } from "@std/assert";
import { part1 } from "./main.ts";

Deno.test({
  name: "part 1 test",
  permissions: { read: true },
  fn: () => {
    const data = Deno.readTextFileSync("./test_input.txt");
    assertEquals(part1(data), "143");
  }
});
