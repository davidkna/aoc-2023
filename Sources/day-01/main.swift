import Foundation

func part1(_ input: String) -> Int {
  let trimmed = input.trimmingCharacters(in: .whitespacesAndNewlines)
  let lines = trimmed.components(separatedBy: .newlines)
  let numbers = lines.map { line in
    line.filter { char in
      char.isNumber
    }.map { char in
      Int(String(char))!
    }
  }.map { numbers in
    print(numbers.first!, numbers.last!)
    return numbers.first! * 10 + numbers.last!
  }

  return numbers.reduce(0, +)
}

func part2(_ input: String) -> Int {
  let part1Input = input.replacingOccurrences(of: "one", with: "o1e")
    .replacingOccurrences(of: "two", with: "t2o")
    .replacingOccurrences(of: "three", with: "t3e")
    .replacingOccurrences(of: "four", with: "f4r")
    .replacingOccurrences(of: "five", with: "f5e")
    .replacingOccurrences(of: "six", with: "s6x")
    .replacingOccurrences(of: "seven", with: "s7n")
    .replacingOccurrences(of: "eight", with: "e8t")
    .replacingOccurrences(of: "nine", with: "n9e")

  return part1(part1Input)
}

let realInput = try! String(contentsOfFile: "input.txt", encoding: .utf8)

let example01 = """
  1abc2
  pqr3stu8vwx
  a1b2c3d4e5f
  treb7uchet
  """

let example02 = """
  two1nine
  eightwothree
  abcone2threexyz
  xtwone3four
  4nineeightseven2
  zoneight234
  7pqrstsixteen
  """

assert(part1(example01) == 142)
assert(part2(example02) == 281)

print("Part 1: \(part1(realInput))")
print("Part 2: \(part2(realInput))")
