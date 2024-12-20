
export function part1(input: string): string {
  let [rules, updates] = input
    .split("\n\n")
    .map((x) => x.split("\n").filter((x) => x != ""));
  console.log("rules", rules);
  console.log("updates", updates);

  let pageRules = new Map<number, Set<number>>();
  rules.forEach((rule) => {
    let [before, after] = rule.split("|").map((x) => parseInt(x));
    if (!pageRules.has(after)) {
      pageRules.set(after, new Set([before]));
    } else {
      pageRules.get(after)?.add(before);
    }
  });
  console.log(pageRules);
  let good_updates: Array<Array<number>> = new Array();
  updates.forEach((update) => {
    let pages: number[] = update.split(",").filter((x) => x != "").map((x) => parseInt(x));
    let well_ordered: boolean = true;
    for (let i = 0; i < pages.length; i++) {
      let current_page = pages[i];
      let after_pages = pages.slice(i);
      if (pageRules.has(current_page)) {
        let violations: Set<number> = pageRules.get(current_page)?.intersection(new Set(after_pages)) ?? new Set([]);
        if (violations.size > 0) {
          console.log(current_page, "violations", violations, " ", pages);
          well_ordered = false;
          break;
        }
      }
    }
    if (well_ordered) {
      good_updates.push(pages);
    }

  });
  return good_updates.map((x) => x[Math.floor(x.length / 2)]).reduce((a, b) => a + b).toString();

}

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const data = Deno.readTextFileSync("./input.txt");
  console.log(part1(data));
}
