from aoc_utils.data import data_text
import unittest


class TestSpaceImageFormat(unittest.TestCase):
    def test_create_empty_image(self):
        img = SpaceImage([])
        self.assertEqual(0, len(img))

    def test_create_image(self):
        img = SpaceImage(list("123456789012"), width=3, height=2)
        self.assertEqual(2, len(img))

    def test_find_best_layer_example(self):
        img = SpaceImage(list("123456789012"), width=3, height=2)
        best_layer = find_best_layer(img)
        self.assertEqual(1, best_layer.count_of("1") * best_layer.count_of("2"))

    def test_find_best_layer_mine(self):
        image_data = data_text(2019, "day_08_mine.txt")
        img = SpaceImage(image_data)
        best_layer = find_best_layer(img)
        self.assertEqual(2318, best_layer.count_of("1") * best_layer.count_of("2"))

    def test_render(self):
        img = SpaceImage(list("0222112222120000"), width=2, height=2)
        self.assertEqual([" #", "# "], img.render())

    def test_render_mine(self):
        image_data = data_text(2019, "day_08_mine.txt")
        img = SpaceImage(image_data)
        rendered = img.render()
        self.assertEqual([
            " ##  #  # ####  ##  ###  ",
            "#  # #  # #    #  # #  # ",
            "#  # #### ###  #    ###  ",
            "#### #  # #    #    #  # ",
            "#  # #  # #    #  # #  # ",
            "#  # #  # #     ##  ###  ",
        ], rendered)


def find_best_layer(img):
    best_zeros = None
    for layer in img:
        zeros = layer.count_of("0")
        if best_zeros is None or zeros < best_zeros:
            best_zeros = zeros
            best_layer = layer
    return best_layer


class SpaceImage:
    def __init__(self, data, width=25, height=6):
        self.width = width
        self.height = height
        self.data = data

    def __getitem__(self, layer):
        area = self.width * self.height
        lower_bound = layer * area

        return Layer(
            self.data[lower_bound: lower_bound + area],
            self.width, self.height,
        )

    def __len__(self):
        return len(self.data) // (self.width * self.height)

    def __iter__(self):
        return ImgIter(self)

    def render(self):
        result = Layer(
            [None for _ in range(self.width*self.height)],
            self.width, self.height,
        )

        for layer in self:
            for y in range(self.height):
                for x in range(self.width):
                    value = layer[x, y]
                    if value != "2" and result[x, y] is None:
                        result[x, y] = value
        mapping = {
            "0": " ",
            "1": "#",
        }

        return [
            "".join(mapping[result[x, y]] for x in range(self.width))
            for y in range(self.height)
        ]


class ImgIter:
    def __init__(self, image):
        self.image = image
        self.layer = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.layer >= len(self.image):
            raise StopIteration
        value = self.image[self.layer]
        self.layer += 1
        return value

    next = __next__  # Python 2 support


class Layer:
    def __init__(self, data, width, height):
        self.data = data
        self.width = width
        self.height = height

    def count_of(self, character):
        return sum(1 for c in self.data if c == character)

    def __getitem__(self, coordinates):
        return self.data[coordinates[0] + self.width * coordinates[1]]

    def __setitem__(self, coordinates, value):
        self.data[coordinates[0] + self.width * coordinates[1]] = value

    def __iter__(self):
        return LayerIter(self)


class LayerIter:
    def __init__(self, layer):
        self.layer = layer
        self.x, self.y = 0, 0

    def __iter__(self):
        return self

    def __next__(self):
        value = self.layer[(x, y)]

        if self.x >= self.layer.width:
            if self.y >= self.layer.height:
                raise StopIteration
            else:
                self.x = 0
                self.y += 1
        else:
            self.x += 1

        return value
