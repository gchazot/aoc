from unittest import TestCase


class Keypad:
    commands = {"U": [-1, 0], "D": [1, 0], "L": [0, -1], "R": [0, 1]}

    def __init__(self, keypad):
        self.keypad = keypad
        self.result = ""
        self.location = None

    def apply_instructions(self, start, instructions):
        self.location = start
        lines = instructions.split('\n')
        instruction_lines = map(lambda l: l.strip(), lines)
        for instruction in instruction_lines:
            self._apply_commands(instruction)
        return self.result

    def _apply_commands(self, commands):
        for command in commands:
            self._apply_command(command)
        self.result += self._key_at(self.location)

    def _apply_command(self, command):
        movement = self.commands[command]
        new_location = list(map(sum, zip(self.location, movement)))
        if self._key_at(new_location) is not None:
            self.location = new_location

    def _size(self, dimension):
        array = self.keypad
        for _ in range(dimension):
            array = array[0]
        return len(array)

    def _key_at(self, location):
        result = self.keypad
        for coordinate in location:
            if coordinate < 0 or coordinate >= len(result):
                return None
            result = result[coordinate]
        return result


def first_keypad_code(instructions):
    keypad_keys = [["1", "2", "3"],
                   ["4", "5", "6"],
                   ["7", "8", "9"]]

    start = [1, 1]
    return  Keypad(keypad_keys).apply_instructions(start, instructions)


def second_keypad_code(instructions):
    keypad_keys = [[None, None, "1", None, None],
                   [None, "2",  "3", "4",  None],
                   ["5",  "6",  "7", "8",  "9"],
                   [None, "A", "B", "C", None],
                   [None, None, "D", None, None]]
    start = [2, 0]
    return Keypad(keypad_keys).apply_instructions(start, instructions)


class TestBathroomKeyPad(TestCase):
    example = """ULL
                 RRDDD
                 LURDL
                 UUUUD"""

    mine = """LRULLRLDUUUDUDDDRLUDRDLDDLUUDLDDLRDRLDRLLURRULURLDRLDUDURLURRULLDDDUDDRRRDLRRDDLDURDULLRDLLLDRDLLDULDUDLLDLDRUDLLDLDDRRRDRLUDRDDLUDRRDUDUDLLDDUUDLRDUDRRUDUDRULRULUDRUUDLDLULLRLDLDDRULLRLLLULUULDURURLUUULDURLDDDURRUUDURDDDULDLURLRDRURDRUDRLLDLDRUURLLLRDRURUDLRLUDULLDDURLRURDLRDUUURRLULRRLDDULUUURLRRRLLLLLURDDRUULUDRRRUDDLLULRRUULDRDDULRLDDDRRUULUDRLRUDURUUULDLDULUUDURLLLRRDDRDLURDDDLDDDLRDRLDDURLRLLRUDRRLLDDDDDURDURRDDULDULLRULDRUURDRRDUDDUDDDDRRDULDUURDRUDRLDULRULURLLRRDRDRDLUUDRRLRLDULDDLUUUUUURRLRRRULLDDDRLRDRRRRRRRDUUDLLUDURUDDLURRUDL
              UDUUURRLRLLDDRRDRRRLDDDLURURLLUDDRLUUDRRRDURRLLRURDLLRRDUUDDDDRDRURRLLLLURDLRRRULLLDLLLUDDLDRRRDLDUUDDRDUDDUURDDLULULDURDURDRUULURURRURDUURUDRRUDRLLLLRRDLLDRDDRLLURDDDUDUDUDRUURDDRUURDLRUUDDRDUURUDDLLUURDLUDRUUDRRDLLUUURDULUULDUUDLLULUUDLUDRUUDUUURLDDDRLRURDDULLRDRULULUDLUUDDDUUDLDUUDRULLDUURDDRUDURULDRDDLRUULRRRDLDLRDULRDDRLLRRLURDLDRUDLRLUDLRLDLDURRUULRLUURDULDRRULLRULRDLLDLDUDRUDDUDLDDURDDDRDLUDRULRUULLRURLDDDRDLRRDRULURULDULRDLDULDURDRDRDRDURDRLUURLRDDLDDRLDDRURLLLURURDULDUDDLLUURDUUUDRUDDRDLDRLRLDURRULDULUUDDLRULDLRRRRDLLDRUUDRLLDLUDUULRDRDLRUUDLRRDDLUULDUULRUDRURLDDDURLRRULURR
              LDURLLLRLLLUURLLULDLRLLDLURULRULRDUDLDDUDRLRRDLULLDDULUUULDRLDURURLURLDLRUDULLLULDUURLLRDLUULRULLLULRDRULUDLUUULDDURLUDDUDDRDLDRDRUDLUURDDLULDUULURLUULRDRDLURUDRUDLDRLUUUUULUDUDRRURUDRULDLDRDRLRURUUDRDLULLUDLLRUUDUUDUDLLRRRLDUDDDRDUDLDLLULRDURULLLUDLLRUDDUUDRLDUULLDLUUDUULURURLLULDUULLDLUDUURLURDLUULRRLLRUDRDLLLRRRLDDLUULUURLLDRDLUUULLDUDLLLLURDULLRUDUUULLDLRLDRLLULDUDUDRULLRRLULURUURLRLURRLRRRDDRLUDULURUDRRDLUDDRRDRUDRUDLDDRLRDRRLDDRLLDDDULDLRLDURRRRRULRULLUUULUUUDRRDRDRLLURRRRUULUDDUDDDLDURDRLDLLLLLRDUDLRDRUULU
              URURRUUULLLLUURDULULLDLLULRUURRDRRLUULRDDRUDRRDUURDUDRUDDRUULURULDRLDRDDDLDLRLUDDRURULRLRLLLDLRRUDLLLLRLULDLUUDUUDRDLRRULLRDRLRLUUDDRRLLDDRULLLRLLURDLRRRRRLLDDRRDLDULDULLDLULLURURRLULRLRLLLLURDDRDDDUUDRRRDUUDDLRDLDRRLLRURUDUUUDLDUULLLRLURULRULRDRLLLDLDLRDRDLLLRUURDDUDDLULRULDLRULUURLLLRRLLLLLLRUURRLULRUUUDLDUDLLRRDDRUUUURRRDRRDULRDUUDULRRRDUUUUURRDUURRRRLDUDDRURULDDURDDRDLLLRDDURUDLLRURLRRRUDDLULULDUULURLUULRDLRDUDDRUULLLRURLDLRRLUDLULDRLUDDDRURUULLDLRLLLDULUDDRLRULURLRDRRDDLDLURUDDUUURRDDLUDDRDUULRRDLDRLLLULLRULRURULRLULULRDUD
              RUDLLUDRRDRRLRURRULRLRDUDLRRLRDDUDRDLRRLLRURRDDLRLLRRURULRUULDUDUULDULDLRLRDLRDLRUURLDRLUDRRDDDRDRRRDDLLLRRLULLRRDDUDULRDRDUURLDLRULULUDLLDRUDUURRUDLLRDRLRRUUUDLDUDRRULLDURRDUDDLRURDLDRLULDDURRLULLRDDDRLURLULDLRUDLURDURRUDULDUUDLLLDDDUUURRRDLLDURRDLULRULULLRDURULLURDRLLRUUDDRRUDRDRRRURUUDLDDRLDRURULDDLLULULURDLDLDULLRLRDLLUUDDUDUDDDDRURLUDUDDDRRUDDLUDULLRDLDLURDDUURDLRLUUDRRULLRDLDDDLDULDUDRDUUULULDULUDLULRLRUULLDURLDULDRDLLDULLLULRLRD"""

    def test_first_example(self):
        self.assertEqual("1985", first_keypad_code(self.example))

    def test_first_mine(self):
        self.assertEqual("53255", first_keypad_code(self.mine))

    def test_second_example(self):
        self.assertEqual("5DB3", second_keypad_code(self.example))

    def test_second_mine(self):
        self.assertEqual("7423A", second_keypad_code(self.mine))
