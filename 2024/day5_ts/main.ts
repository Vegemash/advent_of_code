
export function part1(input: string): string {
  let [rules, updates] = input
    .split("\n\n")
    .map((x) => x.split("\n").filter((x) => x != ""));
  console.log(rules);
  console.log(updates);

  let pages = new Set<number>();
  let pageRules = new Map<number, Set<number>>();
  rules.forEach((rule) => {
    let [before, after] = rule.split("|").map((x) => parseInt(x));
    if (!pageRules.has(before)) {
      pageRules.set(before, new Set([after]));
    } else { pageRules.get(before)?.add(after) }
    pages.add(before);
    pages.add(after);

  });
  console.log(pages);
  console.log(pageRules);
  return input;
}

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
}
