amodes = {
    'immediate': 'Immediate',
    'zeropage': 'ZeroPage',
    'zeropage,X': 'ZeroPageX',
    'zeropage,Y': 'ZeroPageY',
    'absolute': 'Absolute',
    'absolute,X': 'AbsoluteX',
    'absolute,Y': 'AbsoluteY',
    'indirect': 'Indirect',
    '(indirect,X)': 'IndirectX',
    '(indirect),Y': 'IndirectY',
    'accumulator': 'Implied',
    'implied': 'Implied',
    'relative': 'Relative',
}

ctypes = ['None', 'AddOnCross', 'AddOneTwo']

if __name__ == "__main__":
    lines = []
    while True:
        line = input()
        if line == '':
            break
        parts = line.split()
        addressing = parts[0]
        instruction = parts[1]
        opcode = parts[-3]
        bytecount = parts[-2]
        cyclecount = parts[-1]
        cycletype = cyclecount.count('*')
        cyclecount = cyclecount.replace('*', '')
        lines.append(f"OpcodeEntry::new(0x{opcode}, Instruction::{instruction}, AddressingMode::{amodes[addressing]}, {bytecount}, {cyclecount}, CycleRule::{ctypes[cycletype]}),")
    for i in lines:
        print(i)