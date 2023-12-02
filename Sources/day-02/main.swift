import Foundation

struct Move {
  var red: Int
  var green: Int
  var blue: Int

  static func > (lhs: Move, rhs: Move) -> Bool {
    return lhs.red > rhs.red || lhs.green > rhs.green || lhs.blue > rhs.blue
  }

  static func max(_ lhs: Move, _ rhs: Move) -> Move {
    return Move(
      red: Swift.max(lhs.red, rhs.red), green: Swift.max(lhs.green, rhs.green),
      blue: Swift.max(lhs.blue, rhs.blue))
  }

  static func power(_ self: Move) -> Int {
    return self.red * self.green * self.blue
  }
}

func parseInput(_ input: String) -> [[Move]] {
  let trimmed = input.trimmingCharacters(in: .whitespacesAndNewlines)
  let lines = trimmed.components(separatedBy: .newlines)
  return lines.map { (line: String) -> [Move] in
    // strip "Game <num>: "
    let game = line.dropFirst(line.firstIndex(of: ":")!.utf16Offset(in: line) + 2)
    let moves = game.components(separatedBy: "; ")
    return moves.map { move in
      return move.split(separator: ", ").reduce(Move(red: 0, green: 0, blue: 0)) {
        (move: Move, color: Substring) -> Move in
        let split = color.split(separator: " ", maxSplits: 1)
        let count = Int(split[0])!
        let color = String(split[1])
        switch color {
        case "red":
          return Move(red: move.red + count, green: move.green, blue: move.blue)
        case "green":
          return Move(red: move.red, green: move.green + count, blue: move.blue)
        case "blue":
          return Move(red: move.red, green: move.green, blue: move.blue + count)
        default:
          fatalError("Unknown color: \(color)")
        }
      }
    }
  }
}

func part1(_ parsed: [[Move]]) -> Int {
  let maxUsable = Move(red: 13, green: 13, blue: 14)

  return parsed.enumerated().map({ (game: Int, moves: [Move]) -> Int in
    let maxUsage = moves.reduce(Move(red: 0, green: 0, blue: 0)) {
      (move: Move, next: Move) -> Move in
      Move.max(move, next)
    }

    if maxUsage > maxUsable {
      return 0
    }
    return game + 1
  }).reduce(0, +)
}

func part2(_ parsed: [[Move]]) -> Int {
  return parsed.map({ (moves: [Move]) -> Int in
    let maxUsage = moves.reduce(Move(red: 0, green: 0, blue: 0)) {
      (move: Move, next: Move) -> Move in
      Move.max(move, next)
    }
    return Move.power(maxUsage)
  }).reduce(0, +)
}

let example = """
  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
  Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
  Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
  Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
  Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
  """

let realInput = try! String(contentsOfFile: "input.txt", encoding: .utf8)
let parsed = parseInput(realInput)
let parsedExample = parseInput(example)

assert(part1(parsedExample) == 8)
assert(part2(parsedExample) == 2286)

print("Part 1: \(part1(parsed))")
print("Part 2: \(part2(parsed))")
