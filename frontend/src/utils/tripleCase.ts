export function tripleCase<T>(
  condition1: boolean,
  condition2: boolean,
  both: () => T,
  left: () => T,
  right: () => T,
  none: () => T,
): T {
  if (condition1 && condition2) {
    return both();
  }
  if (condition1) {
    return left();
  }
  if (condition2) {
    return right();
  }
  return none();
}
