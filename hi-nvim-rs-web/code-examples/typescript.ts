/**
 * Calculate A+B.
 *
 * From: https://rosettacode.org/wiki/A+B
 */
process.stdin.on("data", (buffer) => {
  console.log(
    (buffer + "")
      .trim()
      .split(" ")
      .map(Number)
      .reduce((a, v) => a + v, 0),
  );
});
