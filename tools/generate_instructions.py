
if __name__ == '__main__':
    with open('instructions.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]
        # print(lines)
    print("#[derive(Debug)]\nenum Instruction {")
    for i in range(len(lines) // 2):
        print(f"    /// {lines[2 * i+1].capitalize()}\n    {lines[2 * i]},")
    print("}\n")
    print('''impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        f.write_str(match *self {''')
    for i in range(len(lines) // 2):
        print(f"            {lines[2 * i]} => \"{lines[2 * i + 1].capitalize()}\",")
    print("        })\n    }\n}")

