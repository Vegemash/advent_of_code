
export function part1(input: string): string {
  return input;

}

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const data = Deno.readTextFileSync("./input.txt");
  console.log(part1(data));
}
